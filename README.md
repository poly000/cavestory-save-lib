# cavestory save lib

[![](https://img.shields.io/docsrs/cavestory-save-lib)](https://docs.rs/cavestory-save-lib/latest/cavestory_save/)

modify major values in CaveStory's `profile.dat`

You may want to check [Fandom wiki](https://cavestory.fandom.com/wiki/Cave_Story_Wiki) for items.

## Note

As `impl From ...` in `src/items/*.rs`, if you get invalid value from `profile.dat`, it will set to `None`(Nothing for BGM).

## TODO

- [ ] flags ([`bitvec`](https://docs.rs/bitvec/latest/bitvec/)) - currently not in plan, as modify this may break game experience
