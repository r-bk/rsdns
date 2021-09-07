# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Changed
- define *rsdns* as *DNS Client* instead of *DNS Stub Resolver*.
  The term *Resolver* may be confused with a system-resolver (e.g. `systemd-resolved`),
  which *rsdns* is not. A more accurate definition for *rsdns* is *DNS Client*.
  A fully featured Resolver can be build above *rsdns*.
- rename the `resolvers` module to `clients`
- rename `resolvers::ResolverConfig` to `clients::ClientConfig`
- rename `resolvers::*::Resolver` to `clients::*::Client`
- reimplement `ClassValue`, `OpCodeValue`, `RCodeValue` and `TypeValue` as new types


## [0.4.1] - 2021-09-04
### Fixed
- fix documentation


## [0.4.0] - 2021-09-04
### Added
- add the `DName` marker-trait, for domain name types who own the domain name bytes
- add the `names` module for domain name types
- implement the `Default` trait on `ResolverConfig`

### Changed
- starting from this release the tags are prefixed with `v`.
  Old tags were adjusted accordingly.
- rename `Unrecognized*` errors to `Unknown*`, for compatibility with the language used
  in RFCs
- rename `RType` to `Type` and `RClass` to `Class`. These enums are used not only for
  records now, so the `R` prefix is obsolete.
- rename `RecordType` to `TypeValue` and `RecordClass` to `ClassValue`. These types are
  relevant not only for records, so the `Record` prefix is obsolete.
- rename `OperationCode` to `OpCodeValue`, for consistency with the rest of enum
  and value types
- move `Name` and `InlineName` structs to the newly added `names` module
- rename `ResolverConfig::new` to `ResolverConfig::with_nameserver`.
  `ResolverConfig::new` returns the default configuration now, without specifying a nameserver.

### Removed
- remove `ProtocolStrategy::Default`. `ProtocolStrategy::Udp` is now the default strategy
  in resolver configuration.


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
