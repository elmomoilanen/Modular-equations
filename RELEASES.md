# Release 1.0.5 (30-12-2022)

### Changed

- Explicit derefs removed when auto-deref possible
- Disable overflow-checks in dev mode by default
- Documentation

### Added

- Version number CLI argument

# Release 1.0.4 (27-10-2022)

### Fixed

- Zero power in arithmetic

### Changed

- Optimize elliptic-curve computation (Montgomery ladder)
- Remove redundant zero checks in arithmetic
- Documentation

### Added

- itertools crate
- integration tests

# Release 1.0.3 (12-08-2022)

### Changed

- Default worker thread count for factorization

### Added

- Docs link (appears in crates.io)

# Release 1.0.2 (07-08-2022)

### Added

- Code documentation and examples (lib and modules)
- Unit tests

### Changed

- Optimise primality testing by dropping unneccessary modulo operations and if statements
- In Miller-Rabin test use labeled continue instead of extra jump variable
- Remove panics when casting equation type to unsigned
- unwrap calls to more thorough error handling in most cases
- README examples

# Release 1.0.1 (04-06-2022)

### Fixed

- Minor documentation and metadata fixes

# Release 1.0.0 (04-06-2022)

### Added

- Everything
