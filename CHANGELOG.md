# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1]

### Changed

- Updated all dependencies to their latest compatible versions, including
  `hickory-resolver`, `igd-next`, `toml`, `validator`, `colored`, and the
  AWS SDK crates.

## [0.2.0]

### Added

- Support for synchronizing multiple domains in a single configuration.

## [0.1.0]

### Added

- Initial release.
- Monitor the host's public IP address and compare it to the current DNS record.
- Automatically update AWS Route 53 DNS records when the public IP changes.
- Configurable polling interval and DNS record TTL.
