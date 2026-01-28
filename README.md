# Fallible
A Filesystem Abstraction Layer Library for Libre and Enterprise Projects

## Overview
This library was originally conceived as part of Accessibility Online's Raise product, to keep everything as generic as we could for higher level components of the system wishing to store data. Geopolitical events in late 2025 - 2026, combined with heavy vendor lock-in for hyperscalers, meant we had to build our frameworks with a vendor-agnostic focus. Therefore, the philosophy is as follows:
* Traits First: Design interfaces which meet the needs of calling layers.
* Modules Second: Build multiple modules which duplicate functionality across vendors
* Private Methods Third: Methods for vendor-specific features should be kept private, and called from public methods required in traits.
* C and FFI layers atop publicly exposed methods, so this framework can be used across multiple projects and languages.

Ideally, a calling layer should be able to use nothing but the publicly exposed methods of traits, to fully leverage platform specific features that provide a business case for using a given vendor. Furthermore, if a calling layer seeks to transition from one provider to another, transitioning from one set of vendor specific features to another, dropping features that would otherwise cause lock-in, or operating two distinct data stores for redundancy, this should be as simple as choosing between different structs to call.

## Using the crate
We're still not ready to publish this on crates.io, so for now we encurrage you to use this as a git submodule in the necessary repo. As this project matures, this will change.

To add a git submodule, use `git submodule add https://github.com/accessibility-online/fallible <dir_name>` where <dir_name> is replaced with the name of the directory you wish to contain this repo.

Then, add the crate to your top level cargo workspace.

## Open Source
As a UK registered CIC, we are obliged to operate with a clear community benefit in mind. Given we already lost a lot of sleep over the issues which lead to the creation of this crate, we decided to make it available under the GNU Public License v2 (GPL 2). If you're also blind or visually impaired, looking to build or scale an app efficiently, but are worries that reliance on the USA tech giants isn't as sound as it used to be, then feel free to use this in your own projects. the same goes for sighted individuals and or teams.

this is our first open source repo, and we are learning as we go. Anyone wishing to contribute code can fork the project, and make a pull request to have their changes merged into the project. Review might take a while, due to lack of resource. However, we will do our best to get to every issue and pr.

## Contribution guidelines
guidelines on contributing have yet to be finalized, though the below points should serve as a starter for ten:
* Keep to the design principles of the project, as stated above.
* Be civil, remember opinions differ, facts do not.
* Anyone unable to justify why their change is necessary will have their PR and or issue closed. Repeated violations of this rule will result in the offending users being banned from the project.
* Do not belittle small contributions; spelling corrections and documentation are just as important as code changes, sometimes more so.

## Documentation
The most up to date technical documentation can be obtained by cloning the repo and running `cargo doc --open`

Efforts are being made to make a daily documentation build available for those who have no interest in installing the development toolchain; this file will be updated with details on where to find that when it exists.