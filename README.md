# dofus-dmg-calculator

[<img alt="github" src="https://img.shields.io/badge/github-black?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Patacode/dofus-dmg-calculator)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dofus-dmg-calculator?logoColor=E3A835&style=for-the-badge&color=9c7325&logo=rust" height="20">](https://crates.io/crates/dofus-dmg-calculator)
[<img alt="crates.io" src="https://img.shields.io/crates/d/dofus-dmg-calculator?logoColor=E3A835&style=for-the-badge&color=152673" height="20">](https://crates.io/crates/dofus-dmg-calculator)

A binary crate to estimate Dofus spells' output damage

## Installation

Current release: [1.0.10](CHANGELOG.md#1.0.10)

```
$ cargo install dofus-dmg-calculator
```

This crate can also be used as a library. Issue the following to add it to your
`Cargo.toml`:

```
$ cargo add dofus-dmg-calculator
```

## Usage

Available options:

```
$ dofus-dmg-calculator --help
Usage: dofus-dmg-calculator [OPTIONS]

Compute a Dofus spell damage estimation

Options:
  -i, --default-min <MIN_DEFAULT_DMG>  The spell minimum default damage [default: 0]
  -j, --default-max <MAX_DEFAULT_DMG>  The spell maximum default damage [default: 0]
  -k, --crit-min <MIN_CRIT_DMG>        The spell minimum critical damage [default: 0]
  -l, --crit-max <MAX_CRIT_DMG>        The spell maximum critical damage [default: 0]
  -s, --stat-points <STAT_POINTS>      The character stat points [default: 0]
  -p, --power <POWER>                  The character power [default: 0]
  -f, --fixed-dmg <FIXED_DMG>          The character fixed damage [default: 0]
  -a, --author                         Print author
  -h, --help                           Print help
  -V, --version                        Print version

Version: 1.0.10
Author: Patacode <pata.codegineer@gmail.com>
```

Example:

*Agility spell inflicting `9 - 11 (12 - 14)` (i.e. 9 to 11 damages by default
and 12 to 14 on critical hit) by a character having 128 points allocated in
agility stat and 1 point in fixed agility damage:*

```
$ dofus-dmg-calculator \
  --default-min 9 \
  --default-max 11 \
  --crit-min 12 \
  --crit-max 14 \
  --stat 128 \
  --fixed-dmg 1
Damage estimation = 21 - 26 (28 - 32)
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
