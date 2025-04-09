# dofus-dmg-calculator

[<img alt="github" src="https://img.shields.io/badge/github-black?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Patacode/dofus-dmg-calculator)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dofus-dmg-calculator?logoColor=E3A835&style=for-the-badge&color=9c7325&logo=rust" height="20">](https://crates.io/crates/dofus-dmg-calculator)
[<img alt="crates.io" src="https://img.shields.io/crates/d/dofus-dmg-calculator?logoColor=E3A835&style=for-the-badge&color=152673" height="20">](https://crates.io/crates/dofus-dmg-calculator)

A binary crate to estimate Dofus spells' output damage

## Installation

Current release: [1.1.6](CHANGELOG.md#1.1.6)

```bash
cargo install dofus-dmg-calculator
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
  -x, --fixed-res <FIXED_RES>          The enemy fixed resistance [default: 0]
  -r, --variable-res <VARIABLE_RES>    The enemy variable resistance [default: 0]
  -a, --author                         Print author
  -h, --help                           Print help
  -V, --version                        Print version

Version: 1.1.6
Author: Patacode <pata.codegineer@gmail.com>
```

Examples:

*Agility spell inflicting `9 - 11 (12 - 14)` (i.e. 9 to 11 damage by default
and 12 to 14 on critical hit) by a character having 128 points allocated in
agility stat and 1 point in fixed agility damage (no power and no resistances):*

```
$ dofus-dmg-calculator \
  --default-min 9 \
  --default-max 11 \
  --crit-min 12 \
  --crit-max 14 \
  --stat-points 128 \
  --fixed-dmg 1
Damage estimation = 21 - 26 (28 - 32)
```

*Same scenario than above but against an enemy with 5 points in fixed agility
resistance and 20% in variable agility resistance:*

```
$ dofus-dmg-calculator \
  --default-min 9 \
  --default-max 11 \
  --crit-min 12 \
  --crit-max 14 \
  --stat-points 128 \
  --fixed-dmg 1 \
  --fixed-res 5 \
  --variable-res 20
Damage estimation = 13 - 16 (18 - 22)
```

## Development

Install `cargo-make`, dev tools and build the package:

```bash
./bootstrap.sh
```

Run the tests:

```bash
cargo test # unit + integration tests
cargo test --lib # unit tests
cargo test --test integration_tests # integration tests
```

Generate code coverage report in HTML format under `target/tarpaulin`:

```bash
cargo tarpaulin \
  --out Html \
  --exclude-files src/lib.rs src/main.rs benches/* tests/* \
  --output-dir target/tarpaulin
```

Benchmark binary and library crates:

```bash
cargo bench
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
