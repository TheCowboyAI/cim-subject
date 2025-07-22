<!-- Copyright (c) 2025 Cowboy AI, LLC. -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2025-01-22

### Added
- GitHub Actions CI/CD workflows for automated testing and releases
- Code coverage reporting with tarpaulin and Codecov integration
- Issue and pull request templates for better collaboration
- Contributing guidelines and community standards
- Comprehensive copyright notices across all files
- Security audit workflow for dependency checking

### Fixed
- Format string syntax errors in translator tests
- Unused imports and variables warnings across test suites
- Doctest errors in SubjectParts documentation

### Changed
- Updated version from 0.3.0 to 0.5.0
- Repository URL now points to correct GitHub location
- Improved CI pipeline without unnecessary NATS dependencies

## [0.3.0] - 2025-01-17

### Added
- Correlation and message algebra implementation
- Comprehensive test suite for subject algebra
- Pattern matching tests with extensive coverage
- Translation system tests
- YubiKey integration support

### Fixed
- Clippy linting errors
- Synchronization with cim-domain module

## [0.2.0] - 2025-01-16

### Added
- Core subject algebra operations
- NATS subject pattern matching
- Message identity and correlation tracking
- Domain-driven design support infrastructure

### Changed
- Improved API ergonomics
- Enhanced documentation

## [0.1.0] - 2025-01-15

### Added
- Initial extraction from alchemist monorepo
- Basic subject manipulation functionality
- Pattern matching with wildcards (* and >)
- Subject translation framework
- Permission system for subject-based access control
- Parser for flexible subject validation
- Comprehensive documentation

[0.5.0]: https://github.com/thecowboyai/cim-subject/compare/v0.3.0...v0.5.0
[0.3.0]: https://github.com/thecowboyai/cim-subject/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/thecowboyai/cim-subject/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/thecowboyai/cim-subject/releases/tag/v0.1.0