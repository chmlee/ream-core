# Changelog

All notable changes to this project will be documented in this file.

## [0.4.2] - 2021-05-23
### Fixed
- Fix unreachable code in downstream referencing
- Fix downstream referencing from type `<T>` to type `List <T>`

## [0.4.1] - 2021-05-23
### Added
- Add downstream referencing
- Add schema validator
- Implement `Display` trait

### Changed
- Untype `ref`

## [0.4.0] - 2021-05-21
### Added
- Add upstream referencing

## [0.3.3] - 2021-05-15
### Fixed
- Fix `flatten_entry`

## [0.3.2] - 2021-05-10
### Added
- Add list type `list <T>`

### Changed
- Force list to be homogeneous, or else raise `TypeError(HeterogeneousList)`
- Wrap `ReamValue` in `ReamValueAnnotated`

## [0.3.1] - 2021-04-27
### Added
- Add README and CHANGELOG

### Fixed
- Change CSV separators from semicolons to commas

## [0.3.0] - 2021-04-26
### Added
- Add type checking for `num` and `bool`

### Changed
- Change syntax for `num`. Numbers are no longer wrapped by dollar signs.
- Change syntax for `bool`. Booleans are no longer wrapped by back ticks.

### Removed
- Drop support for list
