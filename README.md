# facebook-api-rs

[![Crate](https://img.shields.io/crates/v/facebook_api_rs.svg)](https://crates.io/crates/facebook_api_rs)
[![Documentation](https://docs.rs/facebook_api_rs/badge.svg)](https://docs.rs/facebook_api_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.63%2B-orange.svg)](https://www.rust-lang.org/)
[![Facebook API](https://img.shields.io/badge/Facebook%20API-v23.0-blue.svg)](https://developers.facebook.com/docs/graph-api/)

A Rust client library for the **Facebook Graph API v23.0**, with full support for both native and WebAssembly (WASM) environments.

## Features

- ✅ **Facebook Graph API v23.0** - Latest version with 2-year support guarantee
- 🦀 **Rust & WASM** - Works in both backend servers and frontend browsers
- 🔐 **OAuth Flow** - Complete manual login flow implementation
- 📄 **Pages API** - Manage Facebook pages, posts, photos, and videos
- 📸 **Instagram Business API** - Access Instagram business accounts, media, and hashtags
- 🔄 **Batch Requests** - Optimize API calls with batch operations
- ⚡ **Async/Await** - Modern asynchronous Rust patterns
- 🎯 **Type-Safe** - Strongly-typed API responses with serde
- 🔧 **Flexible HTTP** - Choose between `reqwest` (default) or `web-sys` for HTTP

## Requirements

- Rust 1.63.0 or later
- A Facebook App with App ID and App Secret ([Create one here](https://developers.facebook.com/apps/))
- Valid Facebook access tokens for API calls

## Documentation

- [API Documentation on docs.rs](https://docs.rs/facebook_api_rs)
- [Facebook Graph API Reference](https://developers.facebook.com/docs/graph-api/)
- 🔄 [Migration Guide to v23.0](MIGRATION_TO_V23.md)
- ✨ [Enhancements Documentation](ENHANCEMENTS_V23.md)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
facebook_api_rs = "0.1.0"
```

Or install via cargo:

```bash
cargo add facebook_api_rs
```

### Alternative: Install from Git

You can also install directly from GitHub:

```toml
[dependencies]
facebook_api_rs = { git = "https://github.com/osain-az/facebook-api-rs" }
```

Or specify a particular branch, tag, or commit:

```toml
[dependencies]
# Use main branch
facebook_api_rs = { git = "https://github.com/osain-az/facebook-api-rs", branch = "main" }

# Use specific tag
facebook_api_rs = { git = "https://github.com/osain-az/facebook-api-rs", tag = "v0.1.0" }

# Use specific commit
facebook_api_rs = { git = "https://github.com/osain-az/facebook-api-rs", rev = "abc123" }
```

### Feature Flags

The crate provides two HTTP client implementations:

- **`reqwest`** (default) - Uses [reqwest](https://docs.rs/reqwest) for native and WASM targets
- **`web-sys`** - Uses [web-sys](https://docs.rs/web-sys) for browser-based WASM applications

#### Using the Default Feature (reqwest)

```toml
[dependencies]
facebook_api_rs = { git = "https://github.com/osain-az/facebook-api-rs" }
```

#### Using web-sys for WASM

```toml
[dependencies]
facebook_api_rs = { git = "https://github.com/osain-az/facebook-api-rs", default-features = false, features = ["web-sys"] }
```

## Quick Start

### 1. Build a Login URL

Generate a Facebook OAuth login URL to authenticate users:

```rust
use facebook_api_rs::prelude::{Config, LoginResponseType, LoginUrlParameters};

fn create_login_url() -> String {
    let config = Config::new(
        "YOUR_APP_ID".to_string(),
        "https://yourapp.com/callback".to_string()
    );

    LoginUrlParameters::new(config)
        .add_response_type(LoginResponseType::TOKEN)
        .add_scope(vec!["email", "public_profile"])
        .full_login_url()
}
```

### 2. Handle OAuth Callback

After users authenticate, extract tokens from the redirect URL:

```rust
use facebook_api_rs::prelude::UserToken;

async fn handle_callback(redirect_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = UserToken::extract_user_tokens(redirect_url);

    if let Some(error) = tokens.login_error {
        eprintln!("Login error: {:?}", error);
        return Err(Box::new(error));
    }

    println!("Access token: {}", tokens.access_token);
    Ok(())
}
```

### 3. Make API Calls

Use the client to interact with Facebook Graph API:

```rust
use facebook_api_rs::prelude::{Client, TokenLiveType};

async fn get_user_pages(user_token: UserToken) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(user_token, String::new());

    let accounts = client
        .accounts(TokenLiveType::LONGLIVE)
        .get()
        .await?;

    for account in accounts.data {
        println!("Page: {} (ID: {})", account.name, account.id);
    }

    Ok(())
}
```

## API Version Configuration

By default, this crate uses **Facebook Graph API v23.0**. You can specify a custom version if needed:

```rust
use facebook_api_rs::prelude::{Config, Client, UserToken};

// Using default v23.0
let config = Config::new(
    "your_app_id".to_string(),
    "redirect_uri".to_string()
);

// Using custom version (format: vXX.X)
let config = Config::new_with_version(
    "your_app_id".to_string(),
    "redirect_uri".to_string(),
    "v22.0".to_string()
);

// Client with custom version
let client = Client::new_with_version(
    UserToken::default(),
    "page_token".to_string(),
    "v22.0".to_string()
);
```

**Note**: Facebook guarantees each API version for at least 2 years. Always use the latest stable version when possible.

## Usage Examples

### Manual Login Flow

#### Step 1: Generate Login URL

Build the Facebook OAuth login URL (can be done on frontend or backend):

````rust
use facebook_api_rs::prelude::{Config, LoginResponseType, LoginUrlParameters};

```rust
use facebook_api_rs::prelude::{Config, LoginResponseType, LoginUrlParameters};

fn build_login_url() -> String {
    let config = Config::new(
        "YOUR_APP_ID".to_string(),
        "https://yourapp.com/callback".to_string()
    );

    // Use TOKEN response type to get access token directly
    let response_type = LoginResponseType::TOKEN;

    // Or use CODE to exchange for token on server
    // let response_type = LoginResponseType::CODE;

    LoginUrlParameters::new(config)
        .add_response_type(response_type)
        .add_scope(vec!["email", "public_profile", "pages_manage_posts"])
        .full_login_url()
}
````

#### Step 2: Handle Login Response

After successful login, extract tokens or handle errors:

```rust
use facebook_api_rs::prelude::{Client, Me, TokenLiveType, UserToken};

async fn handle_login_response(redirect_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let tokens = UserToken::extract_user_tokens(redirect_url);

    // Check for login errors
    if let Some(error) = tokens.login_error {
        eprintln!("Login failed: {:?}", error);
        return Err(Box::new(error));
    }

    // If using CODE response type, exchange it for access token
    if !tokens.code.is_empty() {
        // Send code to server for token exchange
        println!("Authorization code: {}", tokens.code);
    }

    // If using TOKEN response type, verify the access token
    if !tokens.access_token.is_empty() {
        // Get user information
        let client = Client::new(tokens.clone(), String::new());
        let user: Me = client
            .accounts(TokenLiveType::LONGLIVE)
            .user()
            .await?;

        println!("User ID: {}", user.id);
        println!("User Name: {}", user.name.unwrap_or_default());
    }

    Ok(())
}
```

#### Step 3: Server-Side Token Verification

Verify access tokens or exchange authorization codes on your server:

```rust
use facebook_api_rs::prelude::{Config, UserToken};

// Verify an access token
async fn verify_access_token(
    access_token: String,
    user_id: String,
    app_id: String
) -> Result<bool, Box<dyn std::error::Error>> {
    // Use a valid app token or admin token for verification
    let app_token = "YOUR_APP_TOKEN";

    let token_info = UserToken::access_token_information(
        app_token,
        &access_token
    ).await?;

    // Validate token properties
    if !token_info.is_valid {
        return Ok(false);
    }
    if token_info.app_id != app_id {
        return Ok(false);
    }
    if token_info.user_id != user_id {
        return Ok(false);
    }

    Ok(true)
}

// Exchange authorization code for access token
async fn exchange_code_for_token(
    code: String
) -> Result<UserToken, Box<dyn std::error::Error>> {
    let config = Config::new(
        "YOUR_APP_ID".to_string(),
        "https://yourapp.com/callback".to_string()
    );

    let token = UserToken::default()
        .exchange_code_for_access_token_at_server(
            code,
            "YOUR_APP_SECRET".to_string(),
            config
        )
        .await?;

    Ok(token)
}
```

### Working with Facebook Pages

Get user's pages and manage page content:

```rust
use facebook_api_rs::prelude::{Client, TokenLiveType};

async fn get_user_pages(user_token: UserToken) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(user_token, String::new());

    // Get all pages the user manages
    let accounts = client
        .accounts(TokenLiveType::LONGLIVE)
        .get()
        .await?;

    for account in accounts.data {
        println!("Page: {} (ID: {})", account.name, account.id);
        println!("Access Token: {}", account.access_token);
        println!("Category: {}", account.category);
    }

    Ok(())
}
```

### Publishing to Facebook Pages

Create posts, upload photos, and publish videos:

```rust
use facebook_api_rs::prelude::{Client, TokenLiveType};

async fn publish_page_post(
    user_token: UserToken,
    page_token: String
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(user_token, page_token);

    // Publish a text post
    let result = client
        .pages("PAGE_ID".to_string(), TokenLiveType::LONGLIVE)
        .feed()
        .publish_message("Hello from Rust! 🦀".to_string())
        .await?;

    println!("Post ID: {}", result.id);

    Ok(())
}
```

### Instagram Business API

Access Instagram business accounts and media:

```rust
use facebook_api_rs::prelude::{Client, TokenLiveType};

async fn get_instagram_media(
    user_token: UserToken,
    page_token: String
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(user_token, page_token);

    // Get Instagram account media
    let media = client
        .instagram_account("INSTAGRAM_BUSINESS_ACCOUNT_ID".to_string())
        .media()
        .fields(vec!["id", "caption", "media_type", "media_url", "timestamp"])
        .get()
        .await?;

    for item in media.data {
        println!("Media ID: {}", item.id);
    }

    Ok(())
}
```

## API Coverage

This crate currently supports:

### Authentication & Tokens

- ✅ OAuth login flow with customizable scopes
- ✅ Authorization code exchange
- ✅ Access token verification
- ✅ Short-lived and long-lived tokens
- ✅ Token inspection and debugging

### User & Accounts API

- ✅ Get user profile information
- ✅ List user's managed pages
- ✅ Page access token management

### Facebook Pages API

- ✅ Publish posts to pages
- ✅ Upload photos to pages
- ✅ Upload videos to pages
- ✅ Page feed management
- ✅ Search functionality

### Instagram Business API

- ✅ Instagram Business Account access
- ✅ Media publishing and management
- ✅ Hashtag search
- ✅ Media insights

### Other Features

- ✅ Batch API requests
- ✅ Custom API version support
- ✅ Error handling with typed errors

## Roadmap (v0.1.x)

- [x] OAuth login dialog and redirect URL handling
- [x] Login response parsing
- [x] Authorization code exchange for access tokens
- [x] JSON response handling
- [x] Access token inspection
- [x] Token storage and login status tracking
- [ ] Canceled login handling
- [ ] Identity confirmation
- [ ] Permission checking
- [ ] Re-requesting declined permissions
- [ ] User logout functionality
- [ ] App uninstall detection
- [ ] User data deletion request handling

## Error Handling

The crate uses strongly-typed errors through the `ClientErr` enum:

```rust
use facebook_api_rs::prelude::{Client, ClientErr};

async fn handle_api_errors() {
    match some_api_call().await {
        Ok(data) => println!("Success: {:?}", data),
        Err(ClientErr::HttpError(e)) => eprintln!("HTTP error: {}", e),
        Err(ClientErr::ParseError(e)) => eprintln!("Parse error: {}", e),
        Err(e) => eprintln!("Other error: {:?}", e),
    }
}
```

## Examples

Check out the [examples](examples/) directory for more detailed usage examples, including:

- Complete OAuth flow implementation
- Instagram integration
- Page management

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/osain-az/facebook-api-rs.git
cd facebook-api-rs

# Run tests
cargo test

# Build documentation
cargo doc --open

# Run examples (requires Facebook App credentials)
cargo run --example seed
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Facebook for Developers](https://developers.facebook.com/) - Official API documentation
- Built with ❤️ using Rust

## Support

- � [Repository](https://github.com/osain-az/facebook-api-rs)
- 🐛 [Issue Tracker](https://github.com/osain-az/facebook-api-rs/issues)
- 💬 [Discussions](https://github.com/osain-az/facebook-api-rs/discussions)

## Publishing to Crates.io

This crate is ready for publication to crates.io. When published, users will be able to install it with:

```toml
[dependencies]
facebook_api_rs = "0.1.0"
```

To publish (for maintainers):

```bash
# Ensure all tests pass
cargo test

# Check the package
cargo package

# Publish to crates.io
cargo publish
```

## Related Projects

- [facebook-graph-api](https://crates.io/crates/facebook-graph-api) - Alternative Facebook API client
- [social-media-api](https://crates.io/crates/social-media-api) - Multi-platform social media API

---

**Note**: This is an unofficial library and is not affiliated with Meta Platforms, Inc.
