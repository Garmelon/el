# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
A dependency update to an incompatible version is considered a breaking change.

## Releasing a new version

0. Ensure tests don't fail
1. Update dependencies in a separate commit, if necessary
2. Set new version number in [`Cargo.toml`](Cargo.toml)
3. Add new section in this changelog
4. Commit with message `Bump version to X.Y.Z`
5. Create tag named `vX.Y.Z`
6. Push `master` and the new tag to GitHub
7. Publish new version of the crate

## Unreleased

## v0.1.2 - 2024-12-14

### Added

- `Attr::set`
- `html::attr`

### Deprecated

- `Attr::new` in favor of `Attr::set`
- `Attr::id` in favor of `html::attr::id`
- `Attr::class` in favor of `html::attr::class`
- `Attr::style` in favor of `html::attr::style`
- `Attr::data` in favor of `html::attr::data_x`

## v0.1.1 - 2024-12-08

### Added

- `Element::into_document`
- `impl From<&String> for Content`
- Eponymous JS helper function in readme

## v0.1.0 - 2024-12-02

Initial release
