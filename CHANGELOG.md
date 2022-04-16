# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.12.1] - 2022-04-16
This is a small maintenance release that fixes clippy issues to allow clean
build of *rsdns* with the latest `beta` toolchain.

## Fixed
- fix `clippy::ptr-arg` error
- fix `clippy::await_holding_refcell_ref` warning


## [0.12.0] - 2022-01-14
This is a small maintenance release, to allow building *rsdns* with the latest
`nightly` toolchain.

### Fixed
- fix `clippy::single_char_pattern` warning
- fix `clippy::return-self-not-must-use` warning. This change may affect compilation
  of a user-crate, thus *rsdns* bumps the minor version in this release.


## [0.11.1] - 2021-11-20
### Fixed
- fix compilation warnings when compiled without `net-*` features


## [0.11.0] - 2021-11-19
### Added
- implement EDNS0 support [RFC 2671], [RFC 6891].
  From now on DNS responses longer than 512 bytes can be received over UDP.

### Changed
- enable EDNS in default `ClientConfig` with parameters `version: 0` and `udp_payload_size: 1232`.

### Deleted
- remove `Error::NoNameservers` and `Error::NoBuffer` in favor of the generic `Error::BadParam`.


[RFC 2671]: https://www.rfc-editor.org/rfc/rfc2671.html
[RFC 6891]: https://www.rfc-editor.org/rfc/rfc6891.html


## [0.10.0] - 2021-11-13
### Changed
- upgrade to `tera v0.15.0`, as `v0.13.0` and `v0.14.0` were yanked


## [0.9.0] - 2021-11-12
### Fixed
- fixed a bug in encoding of the root DNS zone `.`

  Previously, an attempt to query the root zone failed because the `.` query name was considered invalid.

### Changed
- clients allocate the incoming message buffer once (in constructor) instead of on every call to `query_rrset`

### Added
- add `ClientConfig::buffer_size` configuration option. It controls the size of the internal buffer allocated
  by clients.


## [0.8.0] - 2021-10-22

This release completes the two-step transition from `RecordsReader` to the new
`MessageReader`. Additionally, this release makes the transition to `Rust 2021`.

### Added

- `MessageReader` is the new and recommended primitive for parsing messages.
  It is very customizable and fast.

### Changed

- `Error::MessageWithoutQuestion` was replaced with `Error::BadQuestionsCount`
- following the transition to `Rust 2021` the minimum supported rust version was raised to `1.56`
  (`MSRV 1.56`)

### Removed

- `RecordsReader` was removed in favor of the new `MessageReader`
- `MessageIterator::question_ref`, `MessageIterator::records_reader` and
  `MessageIterator::records_reader_for` were removed

## [0.7.0] - 2021-10-14

This is a very small release done in preparation for changing the `MessageReader`
API in a future release. Specifically, current `RecordsReader` is going to become,
after some modifications, the future `MessageReader`.

To reduce the impact on user applications, this release renames `MessageReader` to
`MessageIterator`. `MessageIterator` will continue with the iterator-based approach
for reading message parts, i.e. `MessageIterator::questions` and `MessageIterator::records`.
However, in a following release, `MessageIterator::records_reader` and
`MessageIterator::records_reader_for` methods will be removed in favor of using
the revamped `MessageReader`.


### Changed

- refactor `MessageReader` to stop using `RefCell` for section offsets.
- rename `MessageReader` to `MessageIterator` in preparation for transforming
  `RecordsReader` into a full-fledged message reader


## [0.6.0] - 2021-10-01
### Added
- add `NameRef` - for efficient comparison of encoded domain names
- add `RecordsReader` - a flexible and more efficient reader of resource records

### Changed
- implement the `Debug` trait on `MessageReader`
- make all public methods of `MessageReader` to be `inline`
- reimplement `RecordSet::from_msg` with `RecordsReader`.
  Benchmarks show the new implementation is `~35%` faster.


## [0.5.0] - 2021-09-10
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
