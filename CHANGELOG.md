# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added half-width support + full-width space conversion.
- Added support for sentence conversion.
- Added apostrophe between consonant and vowel.
- Added support for diagraphed `-ey`, `-oy`, `-uy`.
- Added apostrophe before consonant in Latin.

### Fixed

- Fixed `=` is preserved in conversion from cyrillic to latin.
- Fixed accent is preserved when converting to kana.
- Fixed `-r` coda not depending on the same syllable.
- Fixed accent in conversion between cyrillic and latin.
- Fixed missing conversion for `wo` to modern Japanese form `ウォ`.
- Fixed combined accent form is not used from cyrillic and latin.

### Test

- Added universal test.
- Use lossy version of latn from kana.
- Test against non-lossy version of latn from cyrillic.
- Removed legacy test cases.

### Chore

- Added [CHANGELOG.md](CHANGELOG.md).

## [0.1.2] - 2024-07-22

### Fixed

- Fixed missing conversion from チ to 'ci'.

### Test

- Added test cases for CI and CA.

### Chore

- Added release workflow.
- Fixed github token.
- Added GitHub release to post-release.
- Specified label to allow post-release to run.
- Fixed create pr permission.
- Used two step release workflow.

## [0.1.1] - 2024-03-03

### Added

- Added link to PyPI version.

### Fixed

- Fixed esm entry points.
- Fixed Cargo crate keywords

### Refactored

- Added comments to conversion functions
- Refactored code by moving `detect` to `detection.rs`.
- Format syllable code.

## [0.1.0] - 2024-02-28

### Added

- Added basic conversion between Katakana, Cyrillic and Latin scripts.
- Added [LICENSE](LICENSE) and metadata to [Cargo.toml](Cargo.toml).

### Removed

- Removed usage of regex.
