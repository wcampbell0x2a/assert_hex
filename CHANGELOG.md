# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2023-08-30
- Update panic message to modern Rust

## [0.2.2] - 2020-01-29
- Fix typo

## [0.2.1] - 2020-01-29
- Fix README.md CHANGELOG.md links
- Improve documentation

## [0.2.0] - 2020-01-13
- Use MIT license ([#2](https://github.com/wcampbell0x2a/assert_hex/issues/2))
- Update `assert_{eq/ne}_bytes()` to using `{:#x?}` for debug output

## [0.1.1] - 2020-01-07
- Use format_args! from global scope instead of $crate ([#1](https://github.com/wcampbell0x2a/assert_hex/pull/1))
- Deal with optional semicolon without forwarding to assert_eq! ([#1](https://github.com/wcampbell0x2a/assert_hex/pull/1))

## [0.1.0] - 2020-03-09
- Initial Release
