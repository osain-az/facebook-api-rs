# Facebook API RS - Seed Example

This example demonstrates how to use the `facebook_api_rs` crate in a WebAssembly application using the Seed framework.

## Features

- Facebook OAuth login flow
- Instagram content management
- WebAssembly-based implementation
- Uses the `web-sys` feature for browser APIs

## Setup

### 1. Install Dependencies

Make sure you have the following installed:

- Rust (latest stable)
- `cargo-make`: `cargo install cargo-make`
- `wasm-pack`: `cargo install wasm-pack`

### 2. Configure Your App

Create a `config.json` file at the root of the example project with the following format:

```json
{
  "facebook_oath_url": "YOUR_OAUTH_URL",
  "client_id": "YOUR_APP_ID",
  "redirect_uri": "YOUR_REDIRECT_URL"
}
```

**Fields:**

- `facebook_oath_url`: The Facebook OAuth endpoint URL
- `client_id`: The ID of your app, found in your app's dashboard at https://developers.facebook.com/apps
- `redirect_uri`: The URL that you want to redirect the person logging in back to (must match your app settings)

### 3. Build and Run

```bash
# Build the WASM application
cargo make build

# Serve the application (requires a local server)
cargo make serve
```

## Using the Published Crate

This example uses `facebook_api_rs` from crates.io:

```toml
[dependencies]
facebook_api_rs = { version = "0.1.0", default-features = false, features = ["web-sys"] }
```

## Development

If you want to test local changes to the `facebook_api_rs` crate:

```toml
[dependencies]
facebook_api_rs = { path = "../../", default-features = false, features = ["web-sys"] }
```

## Learn More

- [facebook_api_rs Documentation](https://docs.rs/facebook_api_rs)
- [Facebook Graph API](https://developers.facebook.com/docs/graph-api)
- [Seed Framework](https://seed-rs.org/)

## License

MIT
