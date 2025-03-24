# dofus-dmg-calculator

This binary crate computes the estimated damages one of your Dofus spell may
inflict.

## Usage

Here are the available options:

```bash
$ dofus-dmg-calculator --help
Usage: dofus-dmg-calculator [OPTIONS]

Compute the estimated damages your Dofus spell may inflict

Options:
  -i, --default-min <DMIN>  The minimum default damage of your spell [default: 0]
  -j, --default-max <DMAX>  The maximum default damage of your spell [default: 0]
  -k, --crit-min <CMIN>     The minimum critical damage of your spell [default: 0]
  -l, --crit-max <CMAX>     The maximum critical damage of your spell [default: 0]
  -s, --stat <STAT>         The stat points of your spell's type your character has [default: 0]
  -p, --power <POWER>       The power of your character [default: 0]
  -f, --fixed-dmg <DFIXED>  The fixed damage of your spell's type your character has [default: 0]
  -a, --author              Print author infos
  -h, --help                Print help
  -V, --version             Print version

Version: 1.0.0
Author: Patacode <pata.codegineer@gmail.com>
```

And here is an example which computes the estimated damages of a spell
inflicting `9-11 (12-14)` (meaning 9 to 11 damages by default and 12 to 14 if a
critical hit occurs), by a character having 128 points allocated in the
spell's stat (e.g. agility) and 1 fixed damage:

```bash
$ dofus-dmg-calculator \
  --default-min 9 \
  --default-max 11 \
  --crit-min 12 \
  --crit-max 14 \
  --stat 128 \
  --fixed-dmg 1
Damage estimation = 21 - 26 (28 - 32)
```
