# MISB Standards Parsing / Generation Implementation in Rust, using `tinyklv`

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/misb.svg)](https://crates.io/crates/misb)
<!-- [![Latest Release](https://img.shields.io/github/v/release/arpadav/misb)](https://github.com/arpadav/misb) -->
<!-- [![Coverage Status](https://coveralls.io/repos/github/arpadav/misb/badge.svg?branch=main)](https://coveralls.io/github/arpadav/misb?branch=main) -->

## ***THIS CRATE IS CURRENTLY UNDER ACTIVE DEVELOPMENT. THIS IS ONLY MEANT FOR CRATE RESERVATION. DO NOT USE WHILE VERSION IS x.x.x-alpha***

<!-- ![nsg-gwg-nga](https://arpadvoros.com/public/misb-crate-banner.png) -->

* Documentation: [https://nsgreg.nga.mil/misb.jsp](https://nsgreg.nga.mil/misb.jsp)
* Framework: [`tinyklv`: A KLV framework written in Rust](https://crates.io/crates/tinyklv)

## Current standards implemented

* [MISB 0102 - Security Metadata Universal and Local Sets for Motion Imagery Data](./src/misb0102.rs)
  * Versions: v12
  * Feature: `latest` | `misb0102-latest` | `misb0102-12`
* [MISB 0601 - UAS Datalink Local Set](./src/misb0601/mod.rs)
  * Versions: v19
  * Feature: `latest` | `misb0601-latest` | `misb0601-19`
* [MISB 0903 - Video Moving Target Indicator Metadata](./src/misb0903/mod.rs)
  * Versions: v6
  * Feature: `latest` | `misb0903-latest` | `misb0903-6`
* [MISB 1201 - Floating Point to Integer Mapping](./src/misb1201.rs)
  * Versions: v5
  * Feature: `latest` | `misb1201-latest` | `misb1201-5`

## Features

* `default` encompasses `latest`
* `latest` corresponds to all the latest versions, e.g. all the `misbXXXX-latest` features

Each remaining feature corresponds to a specific standard of MISB (e.g. `misb0903`) appended with a specific version (`misb0601-19`)

See the [current standards implemented](#current-standards-implemented) and [Cargo.toml](./Cargo.toml) for details.

<!-- ## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) -->

## License

`misb` is licensed under the [MIT License](./LICENSE). [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT).
