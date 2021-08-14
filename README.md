# rsdns - DNS Client Library

**rsnds** is a **R**u**s**t lib crate implementing a **DNS** **S**tub **R**esolver
([RFC 1034](https://www.rfc-editor.org/rfc/rfc1034.html#section-5.3.1)).

Its main goal is to query the Domain Name System and return the results in easily
consumable Rust types.

*rsdns* strives to be simple and fast. To be simple *rsdns* keeps a minimal API.
To be fast *rsdns* aims to allow DNS message parsing with zero memory allocations at runtime.

There are DNS crates which implement asynchronous resolvers, but are built for a single
async runtime. This somewhat blocks usage of such resolver in an application built around
another runtime. *rsdns* closes this gap and genuinely supports three different async
runtimes. Moreover, for completeness, *rsdns* has an independent synchronous resolver as well.


## Notable Features

* Minimal API
* Asynchronous resolvers for `tokio`, `async-std` and `smol`
* Blocking resolver implemented on top of `std::net`
* Zero memory allocations when parsing records with no variable size fields
* Sockets can be bound to network interfaces by name (available on operating
  systems with `SO_BINDTODEVICE` support)
* Minimal set of dependencies


## ch4

*rsdns* comes with a small command-line tool [ch4](https://github.com/r-bk/ch4).
*ch4* uses *rsdns* to query the Domain Name System,
and shows the results in a zone-file format.
It can be used as a simple substitute for [dig](https://en.wikipedia.org/wiki/Dig_(command)),
especially on platforms where *dig* is not originally supported.


## Supported RFCs

*rsdns* focuses on querying the Domain Name System and strives to support all
essential data-type records.

* [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035.html) - the foundation of DNS
  protocol: `A`, `NS`, `TXT`, `CNAME`, `SOA`, `MX`, `PTR`, `ANY` etc.
* [RFC 1101](https://www.rfc-editor.org/rfc/rfc1101.html) - allow leading digits
  in domain name labels
* [RFC 3596](https://www.rfc-editor.org/rfc/rfc3596.html) - `AAAA`


## Roadmap

The following is a short list of features planned for the near future.

* Zero memory allocation for all essential data-types records
* EDNS0 [RFC 6891](https://www.rfc-editor.org/rfc/rfc6891.html) - support
  UDP messages longer than 512 bytes


## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
