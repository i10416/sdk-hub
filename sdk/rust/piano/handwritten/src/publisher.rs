//! # Piano Publisher API
//!
//! This module provides a comprehensive Rust SDK for the Piano Publisher API, which enables
//! content monetization and user management for digital publishing platforms.
//!
//! ## Overview
//!
//! The Piano Publisher API allows you to programmatically manage your Piano applications,
//! including user access, content licensing, promotions, resources, and more. This SDK
//! provides type-safe, async/await interfaces to all major Publisher API endpoints.
//!
//! ## API Reference
//!
//! For complete API documentation, see the [Piano Developer API Documentation](https://docs.piano.io/api/).
//!
//! ## Usage Examples
//!
//! ```rust,no_run
//! use piano_handwritten_api::{
//!     publisher::{
//!         app::{GetAppRequest, GetAppFeaturesRequest},
//!         user::{ListUsersRequest},
//!         team::{ListTeamRequest},
//!     },
//!     PianoAPI,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the Piano API client
//!     let client = PianoAPI::from_env();
//!
//!     // Get application details
//!     let app_request = GetAppRequest::new();
//!     let app = client.get_app(&app_request).await?;
//!     println!("App: {} ({})", app.app.name(), app.app.aid());
//!
//!     // Get application features
//!     let features_request = GetAppFeaturesRequest::new();
//!     let features = client.get_app_features(&features_request).await?;
//!     println!("My Account enabled: {}", features.app_features.is_my_account_enabled());
//!
//!     // List team members
//!     let team_request = ListTeamRequest::new();
//!     let team = client.list_team_members(&team_request).await?;
//!     println!("Team size: {}", team.team_members.len());
//!
//!     // List users with pagination
//!     let users_request = ListUsersRequest::new()
//!         .with_limit(10)
//!         .with_offset(0);
//!     let users = client.list_users(&users_request).await?;
//!     println!("Found {} users (total: {})", users.value.users.len(), users.total);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! The SDK requires Piano API credentials to be configured:
//!
//! - `PIANO_ENDPOINT` - The Piano API endpoint (e.g., "https://sandbox.piano.io/api/v3")
//! - `PIANO_APP_ID` - Your Piano application ID
//! - `PIANO_API_TOKEN` - Your Piano API token
//!
//! These can be set as environment variables or passed directly to the client.
//!
//! ## Error Handling
//!
//! All API methods return `Result<T, crate::Error>` for proper error handling.
//! Piano API errors are automatically parsed and converted to Rust error types.
//!
//! ## Async/Await Support
//!
//! All API methods are async and should be called with `.await`. The SDK is built
//! on top of `reqwest` and `tokio` for high-performance async HTTP operations.

/// User access management and verification
///
/// - Grant and revoke user access to resources
/// - Check user access status
/// - List user access grants
pub mod access;

/// Application management and configuration
///
/// - Get application details and features
/// - List all applications
/// - Retrieve app feature configurations
pub mod app;

/// Consent box configuration and management
///
/// - List consent box configurations
/// - Get specific consent box details
/// - Manage GDPR and privacy compliance
pub mod consent;

/// GDPR compliance and data privacy
///
/// - Export user data
/// - Delete user data
/// - Handle data privacy requests
pub mod gdpr;

/// Content licensing and contract management
///
/// - Manage licensing contracts and users
/// - Handle contract domains and IP ranges
/// - Process licensing notifications
/// - Manage licensees and schedules
pub mod licensing;

/// Promotional campaigns and discount codes
///
/// - Create and manage promotions
/// - Generate and track promotion codes
/// - Configure promotion terms and conditions
pub mod promotion;

/// Content resource management
///
/// - Create and manage content resources
/// - Handle resource bundles and tags
/// - Attach resources to licensing contracts
pub mod resource;

/// Scheduling and time-based operations
///
/// - Manage scheduling periods
/// - Configure contract schedules
/// - Handle time-based licensing
pub mod schedule;

/// Team member and permission management
///
/// - List team members
/// - Manage team permissions
/// - Control application access
pub mod team;

/// User account management
///
/// - Create and manage user accounts
/// - Update user information
/// - Handle user authentication
pub mod user;
