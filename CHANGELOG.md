# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [@Unreleased] - @ReleaseDate

### Added

- Add link to crate documentation stored on docs.rs in `Cargo.toml`
- Add useful badge links in README using shields.io API 

### Changed

- Extract utility functions in library crate for better reusability
- Use `pre-release-replacements` config field instead of 'pre-release-hook' to
update version in README.md

## [1.0.4] - 2025-03-24

### Fixed

- Add missing version specifier in commit message when releasing crate

## [1.0.3] - 2025-03-24

### Added

- Integrate hook with crate releases managed by `cargo release` to auto update
version specifications in README file

### Fixed

- Correct version specifications in README to match latest version

## [1.0.2] - 2025-03-24

### Changed

- Change internal type of data to store computed damage estimation to increase
bit range from 32 to 64 bits

### Fixed

- Correct version specifications in README to match latest version

## [1.0.1] - 2025-03-24

### Changed

- Improve quality of README with more subsections

### Fixed

- Remove template language specifier in README to avoid weird-looking snippet

## [1.0.0] - 2025-03-24

### Added

- Release of binary crate with basic functionalities to compute estimated
damages of Dofus spell given some required input data
