# MISB Standards Parsing / Generation Implementation in Rust, using `tinyklv`

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/misb.svg)](https://crates.io/crates/misb)
<!-- [![Latest Release](https://img.shields.io/github/v/release/arpadav/misb)](https://github.com/arpadav/misb) -->
<!-- [![Coverage Status](https://coveralls.io/repos/github/arpadav/misb/badge.svg?branch=main)](https://coveralls.io/github/arpadav/misb?branch=main) -->

<div style="text-align: center;">
  <img src="https://nsgreg.nga.mil/images/nsg_logo.png" alt="NGA" style="width: 100px; height: auto; display: inline-block; margin: 0 10px;">
  <img src="https://gwg.nga.mil/generated/assets/files/newgwglogosmall-1920px.webp" alt="GWG" style="width: 175px; height: auto; display: inline-block; margin: 0 10px;">
</div>

* Documentation: [https://nsgreg.nga.mil/misb.jsp](nsgreg.nga.mil/misb.jsp)
* Framework: [`tinyklv`: A KLV framework written in Rust](https://crates.io/crates/tinyklv)

## Current standards implemented

* [MISB 0102 - Security Metadata Universal and Local Sets for Motion Imagery Data](./src/misb0102.rs)
  * Versions: v12
  * Usage: `--features misb0102-latest`, or `--features misb0102-12`
* [MISB 0601 - UAS Datalink Local Set](./src/misb0601/mod.rs)
  * Versions: v19
  * Usage: `--features misb0601-latest`, or `--features misb0601-19`
* [MISB 0903 - Video Moving Target Indicator Metadata](./src/misb0903/mod.rs)
  * Versions: v6
  * Usage: `--features misb0903-latest`, or `--features misb0903-6`

## Features

* `default` encompasses `latest`
* `latest` corresponds to all the latest versions, e.g. all the `misbXXXX-latest` features

Each remaining feature corresponds to a specific standard of MISB (e.g. `misb0903`) appended with a specific version (`misb0601-19`)

See the [current standards implemented](#current-standards-implemented) and [Cargo.toml](./Cargo.toml) for details.

<!-- ## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) -->

## License

`misb` is licensed under the [MIT License](./LICENSE). [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT).
