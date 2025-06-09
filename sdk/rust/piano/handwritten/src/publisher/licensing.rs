//! # Piano Site Licensing API
//!
//! This module provides comprehensive access to Piano's Site Licensing system, which enables
//! publishers to engage with hundreds or thousands of users at once through business-to-business
//! content licensing agreements.
//!
//! ## Overview
//!
//! Piano Site Licensing allows your team to create site licenses on behalf of clients via the
//! Piano dashboard or API. These licenses enable any users associated with the contract to
//! redeem access to your site by visiting a landing page and registering.
//!
//! **Key Capabilities:**
//! - Manage licensing contracts for business customers (companies, universities, etc.)
//! - Support up to 200,000 users per contract
//! - Three contract types: specific emails, email domains, and IP ranges
//! - Automated access redemption through landing pages
//! - Schedule-based billing periods and access control
//! - Integration with Composer experiences and My Account
//!
//! ## Core Concepts
//!
//! ### Licensee
//! A **Licensee** is the organization or group you have a licensing agreement with, such as
//! another company or university. Each licensee can have multiple contracts.
//!
//! ### Contract
//! A **Contract** defines the specific logic that manages user access permissions to resources.
//! This includes the number of eligible users (seats), duration of access, and which resources
//! they can access.
//!
//! ### Contract Types
//! 1. **Specific Email Addresses**: Users must register with pre-approved email addresses
//! 2. **Email Domain**: Any user with an email from specified domains can register
//! 3. **IP Range**: Automatic access based on IP address (no registration required)
//!
//! ### Schedule
//! Governs when access is active or inactive, handling billing periods and subscription renewals.
//!
//! ## Implementation Workflow
//!
//! 1. **Create Licensee**: Set up the organization (company/university) that will hold the license
//! 2. **Create Contract**: Define access rules, resource, seats, and landing page URL
//! 3. **Configure Access**:
//!    - For email contracts: Upload user lists or specify domains
//!    - For IP contracts: Define IP ranges in CIDR notation
//! 4. **Attach Schedule**: Set billing periods and access duration
//! 5. **Setup Composer Experience**: Create landing page templates (for email/domain contracts)
//! 6. **User Redemption**: Users visit landing page, register, and gain access
//! 7. **Manage Users**: Add/remove users, handle notifications, monitor usage
//!
//! ## API Reference
//!
//! For complete licensing API documentation, see the [Piano Developer API Documentation](https://docs.piano.io/api/).

/// Core contract management and lifecycle operations
///
/// Handles the fundamental site licensing contract functionality:
/// - **Creating contracts**: Define access rules, resources, seats (max 200,000 users)
/// - **Contract types**: Specific emails, email domains, or IP ranges
/// - **Updating contracts**: Modify terms, resources, and settings
/// - **Contract status**: Active, archived, deactivated states
/// - **Landing pages**: Auto-generated or custom URLs for user redemption
/// - **Seat management**: Track and enforce user limits per contract
///
/// **Note**: Resources cannot be changed once active users are added to the contract.
pub mod contract;

/// Domain-based access control for email domain contracts
///
/// Manages domain restrictions and validation for email domain contracts:
/// - **Domain specification**: Add domains like "company.com" for user access
/// - **Wildcard support**: Use asterisk (*) as wildcard character in domains  
/// - **Domain validation**: Automatic validation of domain formats
/// - **Access rules**: Any user with email from specified domains can redeem
/// - **Domain management**: Add, remove, and list domains for contracts
///
/// Users with emails matching these domains can register and gain access automatically.
pub mod contract_domain;

/// User management within domain-restricted contracts
///
/// Handles users who have redeemed access through email domain contracts:
/// - **User listing**: View all users who redeemed through domain matching
/// - **Access tracking**: Monitor which domains users registered with
/// - **User management**: Revoke access for specific domain users
/// - **Domain removal**: Handle access revocation when domains are removed
///
/// **Note**: Users retain access even if they later change their email to a different domain.
pub mod contract_domain_user;

/// IP range-based access control for automatic access contracts
///
/// Manages IP address restrictions for automatic, registration-free access:
/// - **IP range definition**: Support IPv4/IPv6 addresses and CIDR notation (e.g., 192.168.0.1/24)
/// - **DNS name support**: Accept DNS names in addition to IP addresses
/// - **Automatic access**: Users get immediate access based on IP, no registration needed
/// - **Anonymous support**: Works with unregistered/anonymous users
/// - **Composer integration**: Automatic paywall bypass for matching IP addresses
///
/// **Note**: IP range contracts don't create access entities or require user redemption.
pub mod contract_ip_range;

/// User management for specific email address contracts
///
/// Handles individual users in specific email address contracts:
/// - **User invitation**: Add users manually or via CSV upload (email, first name, last name)
/// - **Invitation emails**: Send and resend invitation emails to users
/// - **User status tracking**: Pending, Active, Revoked, Invalid states
/// - **Access management**: Grant, revoke, and restore user access
/// - **Bulk operations**: Upload CSV files with user lists
/// - **User limits**: Enforce seat limits and capacity restrictions
///
/// **CSV Format**: `email@domain.com, FirstName, LastName` (UTF-8 encoded, comma-separated)
pub mod contract_user;

/// Licensee organization management and representatives
///
/// Manages the business organizations that hold licensing agreements:
/// - **Licensee creation**: Set up companies, universities, or other organizations
/// - **Organization details**: Name, description, logo, and branding
/// - **Licensee managers**: Assign team members to manage relationships
/// - **Representatives**: Designate licensee contacts with elevated permissions
/// - **Branding support**: Upload logos for use in licensing communications
/// - **Multi-contract support**: One licensee can have multiple contracts
///
/// **Representative permissions**: Can modify licenses, add users, and manage their organization's contracts.
pub mod licensee;

/// Site licensing notifications and alerts system
///
/// Handles automated notifications and monitoring for licensing events:
/// - **Capacity alerts**: Notifications when seat usage exceeds thresholds
/// - **Expiration warnings**: Alerts before access periods expire
/// - **Sale notifications**: Updates about contract sales and renewals
/// - **Custom triggers**: Configure percentage-based or date-based alerts
/// - **Dashboard integration**: View notifications in Piano dashboard UI
/// - **Multi-contract support**: Set notifications for specific contracts or all contracts
///
/// **Note**: Notifications appear in dashboard only; no emails are sent to managers or representatives.
pub mod notification;

/// Licensing schedules and billing period management
///
/// Manages temporal aspects and billing cycles of licensing contracts:
/// - **Schedule creation**: Define when contracts are active or inactive
/// - **Billing periods**: Set subscription billing cycles and renewal dates
/// - **Access duration**: Control how long users retain access
/// - **Subscription reallocation**: Update all users to new schedules (hourly batch process)
/// - **Renewal management**: Automatic renewals when new periods are available
/// - **Expiration handling**: "Won't renew" state when no next period exists
/// - **Schedule reuse**: Same schedule can be used across multiple contracts
///
/// **Important**: Users lose access when schedules expire unless new periods are added.
pub mod schedule;
