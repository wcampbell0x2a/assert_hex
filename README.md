# asterix_hex
[![Latest Version](https://img.shields.io/crates/v/assert_hex.svg)](https://crates.io/crates/assert_hex)
[![Rust Documentation](https://docs.rs/assert_hex/badge.svg)](https://docs.rs/assert_hex)

display expression using `{:#x?}` format when false assertion causes `panic!()`.

# why
Writing and testing protocol level libraries requires many tests to be written
with respect to byte level protocol sections in hex. This library simplifies the process
of viewing the differences between these types when tests fail by displaying by using the
`{:#x?}` representation.

# usage
Replace `assert_eq` or `assert_ne` with `assert_eq_hex` or `assert_ne_hex`
respectively.

## Changelog

See [CHANGELOG.md](https://github.com/wcampbell0x2a/assert_hex/blob/master/CHANGELOG.md)
