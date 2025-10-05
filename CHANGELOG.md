# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-05

### Added

- Initial release of facebook-api-rs
- Facebook Graph API v23.0 support
- OAuth 2.0 authentication flow
  - Authorization code exchange
  - Access token verification
  - Short-lived and long-lived token support
- Facebook Pages API integration
  - Page management
  - Post publishing
  - Photo and video uploads
  - Feed management
- Instagram Business API support
  - Media publishing and management
  - Hashtag search
  - Account information
- User and Accounts API
  - Profile information retrieval
  - Managed pages listing
- Batch API requests support
- Dual HTTP client support
  - reqwest (default, for native and WASM)
  - web-sys (for browser-based WASM)
- Custom API version support
  - Version validation helpers
  - Version extraction methods
- Comprehensive error handling with typed errors
- Full async/await support
- Type-safe API responses with serde

### Documentation

- Complete README with quick start guide
- API usage examples
- Migration guide to v23.0
- Facebook API compliance review document
- Enhancements documentation
- Inline code documentation

### Features

- `reqwest` - Default HTTP client using reqwest (enabled by default)
- `web-sys` - Alternative HTTP client for WASM environments

[0.1.0]: https://github.com/osain-az/facebook-api-rs/releases/tag/v0.1.0
