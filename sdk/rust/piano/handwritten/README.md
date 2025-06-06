# Piano Rust SDK

A Rust SDK for the Piano API, providing easy-to-use interfaces for user management, access control, licensing, and subscription management.

This crate is tested against real data to check spec conformance using the response snapshot from read operations.
These tests can be found at schema.rs and `*.snapshot.json`.

## Quick Start

### Prerequisites

- Rust 1.70 or later
- A Piano account with API credentials (App ID and API Token)
- Access to Piano API endpoints (sandbox or production)

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
piano-handwritten-api = { git = "https://github.com/i1041/sdk-hub" }
tokio = { version = "1.0", features = ["full"] }
```

#### Feature Flags

This SDK supports the following feature flags to customize functionality and dependencies:

#### Default Features

- `rustls` - Uses rustls as the TLS backend (enabled by default)
- `tracing` - Enables structured logging and tracing support (enabled by default)

#### TLS Backend Selection

Choose one of the following TLS backends:

- `rustls` - Pure Rust TLS implementation (recommended, enabled by default)
- `native-tls` - Uses the system's native TLS implementation


### Usage Example

```rust
use piano_handwritten_api::{PianoAPI, publisher::user::*};

#[tokio::main]
async fn main() -> Result<(), piano_handwritten_api::Error> {
    // Initialize the Piano API client
    let api = PianoAPI::new(
        "https://sandbox.piano.io/api/v3", // or production URL
        "your_app_id",
        "your_api_token",
    );

    // Create a new user
    let user_request = CreateUserRequest::new("user@example.com")
        .with_first_name("John")
        .with_last_name("Doe");

    let user = api.create_user(&user_request).await?;
    println!("Created user: {} ({})", user.email(), user.uid());

    Ok(())
}
```

## Configuration

### Environment Variables

You can use environment variables for configuration:

```rust
use std::env;

let api = PianoAPI::new(
    &env::var("PIANO_ENDPOINT").unwrap_or_else(|_| "https://sandbox.piano.io/api/v3".to_string()),
    &env::var("PIANO_APP_ID").expect("PIANO_APP_ID must be set"),
    &env::var("PIANO_API_TOKEN").expect("PIANO_API_TOKEN must be set"),
);
```

## Examples

The `examples/` directory contains comprehensive examples:

- `user_management.rs` - Complete user management workflow
- `access_management.rs` - Access control and permissions

Run examples with:

```bash
cargo run --example user_management
cargo run --example access_management
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only documentation tests
cargo test --doc

# Run examples (compile check)
cargo check --examples
```
