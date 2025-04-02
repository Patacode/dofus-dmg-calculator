# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[comment]: <> (@PlannedForNextRelease)
## [@Unreleased] - @ReleaseDate

## [1.1.2] - 2025-04-02 <a id="1.1.2"></a>

### Added

- Cargo.lock file to ensure reproducible builds
- Rust doc for public functions

### Changed

- Source content of binary package (only necessary stuff)

## [1.1.1] - 2025-04-02 <a id="1.1.1"></a>

### Changed

- Split public/private api in different modules (facade pattern)

## [1.1.0] - 2025-04-02 <a id="1.1.0"></a>

### Added

- Webhook to update codegineer repo on push to main
- CLI option to compute damage estimation with fixed and/or variable
resistances
- Example in README with fixed and variable resistances
- Usage feature of clap crate

### Changed

- Some code refactoring

## [1.0.14] - 2025-03-27 <a id="1.0.14"></a>

### Removed

- criterion and test_bin from final archive
- suggestions and usage clap features

## [1.0.13] - 2025-03-27 <a id="1.0.13"></a>

### Removed

- Mention of usage as library in README

### Fixed

- Prevent deploy job from running of cancelled workflow

## [1.0.12] - 2025-03-27 <a id="1.0.12"></a>

### Fixed

- SemVer Regex in pre release hooks now support version > 9

## [1.0.11] - 2025-03-27 <a id="1.0.11"></a>

### Changed

- Improve help message with clearer description
- Improve README with more concise sentences

## [1.0.10] - 2025-03-27 <a id="1.0.10"></a>

### Added

- Link to latest version fo CHANGELOG in README

## [1.0.9] - 2025-03-27 <a id="1.0.9"></a>

### Added

- Rust linter
- Test report visualization from deployment pipeline
- Lint test in deployment pipeline
- VSCode task to format code

### Changed

- Optimize GitHub Action workflow to make it faster using various caching
techniques

## [1.0.8] - 2025-03-26 <a id="1.0.8"></a>

### Fixed

- Add missing tag due to broken pipeline

## [1.0.7] - 2025-03-26 <a id="1.0.7"></a>

### Added

- GitHub action with manual workflow to release new crate versions

### Changed

- Some code and test refactoring

## [1.0.6] - 2025-03-25 <a id="1.0.6"></a>

### Added

- Improve replacement hook on crate release to also add unreleased section with
proper replacement tags
- Improve README with note regarding crate uses as library
- Unit and integration tests
- Benchmarking

### Changed

- Custom rust formatter config to meet own standard
- Improve sentences quality in README and crate description
- Improve namings in crate source code

## [1.0.5] - 2025-03-24 <a id="1.0.5"></a>

### Added

- Add link to crate documentation stored on docs.rs in `Cargo.toml`
- Add useful badge links in README using shields.io API
- Automate changelog update on crate release 

### Changed

- Extract utility functions in library crate for better reusability
- Use `pre-release-replacements` config field instead of 'pre-release-hook' to
update version in README.md

## [1.0.4] - 2025-03-24 <a id="1.0.4"></a>

### Fixed

- Add missing version specifier in commit message when releasing crate

## [1.0.3] - 2025-03-24 <a id="1.0.3"></a>

### Added

- Integrate hook with crate releases managed by `cargo release` to auto update
version specifications in README file

### Fixed

- Correct version specifications in README to match latest version

## [1.0.2] - 2025-03-24 <a id="1.0.2"></a>

### Changed

- Change internal type of data to store computed damage estimation to increase
bit range from 32 to 64 bits

### Fixed

- Correct version specifications in README to match latest version

## [1.0.1] - 2025-03-24 <a id="1.0.1"></a>

### Changed

- Improve quality of README with more subsections

### Fixed

- Remove template language specifier in README to avoid weird-looking snippet

## [1.0.0] - 2025-03-24 <a id="1.0.0"></a>

### Added

- Release of binary crate with basic functionalities to compute estimated
damages of Dofus spell given some required input data
