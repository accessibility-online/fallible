//! Integration tests for S3Facade
//!
//! These tests require valid AWS credentials and will create real S3 resources.
//! Each test operates under an isolated prefix within a shared test bucket.
//!
//! # Test Isolation
//!
//! Each test writes to a unique prefix within a shared bucket, preventing
//! collisions when tests run in parallel. We use a fixed bucket name
//! (`a11y-online-fallible-library-tests`) rather than random names to avoid
//! orphaned buckets when tests fail or are cancelled.
//!
//! # Cleanup
//!
//! Test data accumulates in the bucket. Periodic cleanup:
//! ```sh
//! aws s3 rm s3://a11y-online-fallible-library-tests --recursive
//! ```

use aws_config::{self as aws, BehaviorVersion};
use aws_sdk_s3 as s3;
use fallible::s3_facade::S3Facade;
use fallible::storage_facade::StorageFacade;
use std::sync::LazyLock;
use tokio::sync::OnceCell;
use uuid::Uuid;

const TEST_BUCKET_NAME: &str = "a11y-online-fallible-library-tests";

/// Ensures bucket creation happens exactly once, even with parallel tests.
static BUCKET_INITIALIZED: LazyLock<OnceCell<()>> = LazyLock::new(OnceCell::new);

async fn ensure_bucket_exists() {
    BUCKET_INITIALIZED
        .get_or_init(|| async {
            let config = aws::load_defaults(BehaviorVersion::v2026_01_12()).await;
            let client = s3::Client::new(&config);

            let exists = client
                .head_bucket()
                .bucket(TEST_BUCKET_NAME)
                .send()
                .await
                .is_ok();

            if !exists {
                let region = config.region().map(|r| r.as_ref().to_string());
                let mut create_req = client.create_bucket().bucket(TEST_BUCKET_NAME);

                // S3 quirk: us-east-1 rejects LocationConstraint, other regions require it
                if let Some(ref region_str) = region {
                    if region_str != "us-east-1" {
                        let constraint =
                            s3::types::BucketLocationConstraint::from(region_str.as_str());
                        let bucket_config = s3::types::CreateBucketConfiguration::builder()
                            .location_constraint(constraint)
                            .build();
                        create_req = create_req.create_bucket_configuration(bucket_config);
                    }
                }

                create_req
                    .send()
                    .await
                    .expect("Failed to create test bucket");
            }
        })
        .await;
}

/// Provides test isolation via unique prefixes within the shared bucket.
struct S3TestContext {
    prefix: String,
    facade: S3Facade,
}

impl S3TestContext {
    /// Creates a context with a unique prefix like `<uuid>/test_name/`.
    async fn new(test_name: &str) -> Self {
        ensure_bucket_exists().await;

        let prefix = format!("{}/{}/", Uuid::new_v4(), test_name);

        let facade = S3Facade::new(TEST_BUCKET_NAME, &format!("Test context for {}", test_name))
            .await
            .expect("Failed to create S3Facade for test");

        Self { prefix, facade }
    }

    fn facade(&self) -> &S3Facade {
        &self.facade
    }

    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn path(&self, relative: &str) -> String {
        format!("{}{}", self.prefix, relative)
    }
}

#[tokio::test]
async fn test_write_and_read_data() {
    let ctx = S3TestContext::new("write-read").await;
    let facade = ctx.facade();
    let path = ctx.path("test-file.txt");

    let data = b"Hello, S3 integration test!";

    // Write data
    facade
        .write_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &path, data, None,
        )
        .await
        .expect("write_data should succeed");

    // Read it back
    let result = facade
        .read_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &path, None,
        )
        .await
        .expect("read_data should succeed");

    assert_eq!(result, data.to_vec());
}

#[tokio::test]
async fn test_write_and_read_with_encryption() {
    let ctx = S3TestContext::new("encrypt-decrypt").await;
    let facade = ctx.facade();
    let path = ctx.path("encrypted-file.txt");

    let original_data = b"Secret message for encryption test";

    // Simple XOR "encryption" for testing (NOT secure, just for testing the callback mechanism)
    let key: u8 = 0x42;

    let encrypt = |data: &[u8]| -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(data.iter().map(|b| b ^ key).collect())
    };

    let decrypt = |data: &[u8]| -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(data.iter().map(|b| b ^ key).collect())
    };

    // Write with encryption
    facade
        .write_data(&path, original_data, Some(encrypt))
        .await
        .expect("write_data with encryption should succeed");

    // Read with decryption
    let result = facade
        .read_data(&path, Some(decrypt))
        .await
        .expect("read_data with decryption should succeed");

    assert_eq!(result, original_data.to_vec());

    // Verify data is actually encrypted on S3 (read without decrypt)
    let raw_result = facade
        .read_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &path, None,
        )
        .await
        .expect("read_data without decryption should succeed");

    assert_ne!(raw_result, original_data.to_vec(), "Data should be encrypted on S3");
}

#[tokio::test]
async fn test_file_exists_true_and_false() {
    let ctx = S3TestContext::new("file-exists").await;
    let facade = ctx.facade();

    let existing_path = ctx.path("existing-file.txt");
    let nonexistent_path = ctx.path("nonexistent-file.txt");

    // File should not exist initially
    assert!(
        !facade.file_exists(&nonexistent_path).await,
        "file_exists should return false for nonexistent file"
    );

    // Create a file
    facade
        .write_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &existing_path,
            b"test content",
            None,
        )
        .await
        .expect("write_data should succeed");

    // Now it should exist
    assert!(
        facade.file_exists(&existing_path).await,
        "file_exists should return true for existing file"
    );

    // Other file still should not exist
    assert!(
        !facade.file_exists(&nonexistent_path).await,
        "file_exists should still return false for nonexistent file"
    );
}

#[tokio::test]
async fn test_list_objects() {
    let ctx = S3TestContext::new("list-objects").await;
    let facade = ctx.facade();

    // Create multiple files
    let files = ["file-a.txt", "file-b.txt", "subdir/file-c.txt"];
    for file in &files {
        let path = ctx.path(file);
        facade
            .write_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
                &path,
                format!("content of {}", file).as_bytes(),
                None,
            )
            .await
            .expect("write_data should succeed");
    }

    // List objects under prefix
    let listed = facade
        .list_objects(ctx.prefix())
        .await
        .expect("list_objects should succeed");

    // Verify all files are present
    assert_eq!(listed.len(), files.len(), "Should list all created files");

    for file in &files {
        let expected_path = ctx.path(file);
        assert!(
            listed.contains(&expected_path),
            "Listed objects should contain {}",
            expected_path
        );
    }
}

#[tokio::test]
async fn test_delete_file() {
    let ctx = S3TestContext::new("delete-file").await;
    let facade = ctx.facade();
    let path = ctx.path("file-to-delete.txt");

    // Create the file
    facade
        .write_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &path, b"delete me", None,
        )
        .await
        .expect("write_data should succeed");

    // Verify it exists
    assert!(facade.file_exists(&path).await, "File should exist before deletion");

    // Delete it
    facade
        .delete_file(&path)
        .await
        .expect("delete_file should succeed");

    // Verify it's gone
    assert!(
        !facade.file_exists(&path).await,
        "File should not exist after deletion"
    );
}

#[tokio::test]
async fn test_move_file() {
    let ctx = S3TestContext::new("move-file").await;
    let facade = ctx.facade();

    let source_path = ctx.path("source-file.txt");
    let dest_path = ctx.path("dest-file.txt");
    let content = b"content to be moved";

    // Create source file
    facade
        .write_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &source_path, content, None,
        )
        .await
        .expect("write_data should succeed");

    // Move it
    facade
        .move_file(&source_path, &dest_path)
        .await
        .expect("move_file should succeed");

    // Source should be gone
    assert!(
        !facade.file_exists(&source_path).await,
        "Source file should not exist after move"
    );

    // Destination should exist
    assert!(
        facade.file_exists(&dest_path).await,
        "Destination file should exist after move"
    );

    // Content should match
    let result = facade
        .read_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &dest_path, None,
        )
        .await
        .expect("read_data should succeed");

    assert_eq!(result, content.to_vec());
}

#[tokio::test]
async fn test_copy_file() {
    let ctx = S3TestContext::new("copy-file").await;
    let facade = ctx.facade();

    let source_path = ctx.path("original-file.txt");
    let dest_path = ctx.path("copied-file.txt");
    let content = b"content to be copied";

    // Create source file
    facade
        .write_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &source_path, content, None,
        )
        .await
        .expect("write_data should succeed");

    // Copy it
    facade
        .copy_file(&source_path, &dest_path)
        .await
        .expect("copy_file should succeed");

    // Both should exist
    assert!(
        facade.file_exists(&source_path).await,
        "Source file should still exist after copy"
    );
    assert!(
        facade.file_exists(&dest_path).await,
        "Destination file should exist after copy"
    );

    // Both should have same content
    let source_content = facade
        .read_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &source_path, None,
        )
        .await
        .expect("read source should succeed");

    let dest_content = facade
        .read_data::<fn(&[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>>(
            &dest_path, None,
        )
        .await
        .expect("read destination should succeed");

    assert_eq!(source_content, dest_content);
    assert_eq!(source_content, content.to_vec());
}

#[tokio::test]
async fn test_metadata() {
    let ctx = S3TestContext::new("metadata").await;
    let facade = ctx.facade();

    let metadata = facade.metadata();

    // Name should match the fixed test bucket name
    assert_eq!(
        metadata.name, "a11y-online-fallible-library-tests",
        "Bucket name should match expected"
    );

    // Description should be set
    assert!(
        !metadata.description.is_empty(),
        "Description should not be empty"
    );

    // ID should be an S3 ARN
    match &metadata.id {
        fallible::storage_facade::DataStoreId::S3(arn) => {
            assert!(
                arn.starts_with("arn:aws:s3:::"),
                "S3 ARN should have correct prefix"
            );
        }
        _ => panic!("DataStoreId should be S3 variant"),
    }
}
