# raise-data
This repo is exclusively for classes which handle the data objects, storage and retrieval mechanisms of Raise.

## Overview
The idea of this repo is to keep everything as generic as we can for other components of the system. Geopolitical events in late 2025 - 2026, combined with heavy vendor lock-in for hyperscalers, means that we have had to build our frameworks with a vendor-agnostic focus. Therefore, the philosophie is as follows:
* Traits First: Design interfaces which fit a need of the framework
* Modules Second: Build multiple modules which duplicate functionality across vendors
* Private Methods Third: Methods for vendor-specific features should be kept private, and called from public methods required in traits, unless absolutely necessary.

This way, if we need to migrate from one vendor to another, the only things we need to change are the constructor calls, and module names.

## Extern 'c' and FFIs
To avoid writing multiple libraries for different projects that require the same functionality, C ABI functions should be exposed, and conversion between C and Rust types managed accordingly. Policy on this has yet to be finalised, so be aware that submissions for custom ABIs may be subject to increased scrutiny before merging.

## Documentation
The most up to date technical documentation can be obtained by cloning the repo and running `cargo doc --open`

Efforts are being made to make a daily documentation build available for those who have no interest in installing the development toolchain; this file will be updated with details on where to find that when it exists.