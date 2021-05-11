# Changelog

All notable changes to this project will be documented in this file.

## [0.3.2] - 2021-05-10
### Added
- Add list support
- Add three types: `list str`, `list num` and `list bool`

### Changed
- Force list to be homogeneous, or else raise `HeterogeneousList` error
- Wrap `ReamValue` in `ReamValueAnnotated`

## [0.3.1] - 2021-04-27
### Added
- Add README and CHANGELOG

### Fixed
- Change CSV separators from semicolons to commas

## [0.3.0] - 2021-04-26
### Added
- Add type checking for numbers and boolean

### Changed
- Chang syntax for `num`. Numbers are no longer wrapped by dollar signs.
- Chang syntax for `bool`. Booleans are no longer wrapped by back ticks.

### Removed
- Drop support for list
