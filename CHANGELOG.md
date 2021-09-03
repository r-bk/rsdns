# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Changed
- starting from this release the tags are prefixed with `v`.
  Old tags were adjusted accordingly.

## [0.3.0] - 2021-08-13
### Added
- add a link to **ch4** in README

### Changed
- support leading digits in domain name labels `RFC 1101`


## [0.2.0] - 2021-08-12
### Fixed
- fix `Display` trait implementation. Padding, alignment and fill specifiers are
  supported now: `println!("{:+>32}", name)`

### Changed
- update dependencies to be stricter, using tilde specification
- cleanup documentation
- refactor templates for readability


## [0.1.0] - 2021-08-06

- Initial crate release.
