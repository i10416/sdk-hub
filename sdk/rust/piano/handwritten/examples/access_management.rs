/// Example demonstrating Piano SDK access management functionality.
///
/// This example shows how to:
/// - Grant access to users for specific resources
/// - Check if users have access to resources
/// - Revoke access from users
/// - List access grants with filtering
use piano_handwritten_api::{publisher::access::*, PianoAPI};

#[tokio::main]
async fn main() -> Result<(), piano_handwritten_api::Error> {
    // Initialize the Piano API client
    // In production, these would come from environment variables or configuration
    let api = PianoAPI::from_env();

    println!("Piano SDK Access Management Example");
    println!("===================================");

    let user_uid = "example_user_uid"; // Replace with actual user UID
    let resource_id = "article_123"; // Replace with actual resource ID

    // Example 1: Grant access to a user
    println!("\n1. Granting access to user...");
    let grant_request = GrantAccessRequest::new(user_uid, resource_id)
        .with_resource_type("article")
        .with_expires(1999999999) // Set expiration timestamp
        .with_custom_params(r#"{"source": "subscription", "plan": "premium"}"#);

    match api.grant_access(&grant_request).await {
        Ok(access_grant) => {
            println!("✓ Access granted successfully!");
            println!("  Grant ID: {}", access_grant.access_grant_id());
            println!("  User UID: {}", access_grant.uid());
            println!("  Resource: {}", access_grant.resource_id());
            println!("  Type: {}", access_grant.resource_type().unwrap_or("N/A"));
            println!("  Status: {}", access_grant.status().unwrap_or("N/A"));
            if let Some(expires) = access_grant.expires_date() {
                println!("  Expires: {}", expires);
            }
        }
        Err(e) => println!("✗ Failed to grant access: {}", e),
    }

    // Example 2: Check if user has access
    println!("\n2. Checking user access...");
    match api.check_access(user_uid, resource_id).await {
        Ok(has_access) => {
            if has_access {
                println!("✓ User has access to the resource");
            } else {
                println!("ℹ User does not have access to the resource");
            }
        }
        Err(e) => println!("✗ Failed to check access: {}", e),
    }

    // Example 3: List user's access grants
    println!("\n3. Listing user's access grants...");
    let list_request = ListAccessRequest::new()
        .with_limit(10)
        .with_resource_type("article")
        .with_status("ACTIVE");

    match api.list_user_access(user_uid, &list_request).await {
        Ok(results) => {
            println!("✓ Access grants retrieved!");
            println!("  Total grants: {}", results.total);
            println!("  Showing {} grants:", results.count);

            for (i, grant) in results.value.access_grants.iter().enumerate() {
                println!(
                    "    {}. Resource: {} ({})",
                    i + 1,
                    grant.resource_id(),
                    grant.resource_type().unwrap_or("unknown")
                );
                println!(
                    "       Status: {}, Active: {}",
                    grant.status().unwrap_or("unknown"),
                    grant.is_active()
                );
            }
        }
        Err(e) => println!("✗ Failed to list access grants: {}", e),
    }

    // Example 4: List all access grants with filtering
    println!("\n4. Listing all access grants...");
    let list_all_request = ListAllAccessRequest::new()
        .with_limit(20)
        .with_resource_type("article")
        .with_status("ACTIVE");

    match api.list_all_access(&list_all_request).await {
        Ok(results) => {
            println!("✓ All access grants retrieved!");
            println!("  Total grants: {}", results.total);
            println!("  Active article grants: {}", results.count);

            // Group by user for better display
            let mut user_grants: std::collections::HashMap<String, Vec<&AccessGrant>> =
                std::collections::HashMap::new();

            for grant in &results.value.access_grants {
                user_grants
                    .entry(grant.uid().to_string())
                    .or_insert_with(Vec::new)
                    .push(grant);
            }

            for (uid, grants) in user_grants.iter().take(5) {
                // Show first 5 users
                println!("    User {}: {} grants", uid, grants.len());
                for grant in grants.iter().take(3) {
                    // Show first 3 grants per user
                    println!("      - {}", grant.resource_id());
                }
            }
        }
        Err(e) => println!("✗ Failed to list all access grants: {}", e),
    }

    // Example 5: Grant temporary access
    println!("\n5. Granting temporary access...");
    let temp_resource = "premium_article_456";
    let expires_in_hour = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + 3600; // 1 hour from now

    let temp_grant_request = GrantAccessRequest::new(user_uid, temp_resource)
        .with_resource_type("premium_article")
        .with_expires(expires_in_hour)
        .with_custom_params(r#"{"type": "trial", "duration": "1h"}"#);

    match api.grant_access(&temp_grant_request).await {
        Ok(access_grant) => {
            println!("✓ Temporary access granted!");
            println!("  Resource: {}", access_grant.resource_id());
            println!("  Expires in 1 hour");
            println!("  Is currently active: {}", access_grant.is_active());
        }
        Err(e) => println!("✗ Failed to grant temporary access: {}", e),
    }

    // Example 6: Revoke access
    println!("\n6. Revoking access...");
    let revoke_request =
        RevokeAccessRequest::new(user_uid, temp_resource).with_resource_type("premium_article");

    match api.revoke_access(&revoke_request).await {
        Ok(()) => {
            println!("✓ Access revoked successfully!");

            // Verify access was revoked
            match api.check_access(user_uid, temp_resource).await {
                Ok(has_access) => {
                    if has_access {
                        println!("  ⚠ User still has access (may take time to propagate)");
                    } else {
                        println!("  ✓ Access successfully removed");
                    }
                }
                Err(e) => println!("  ? Could not verify revocation: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to revoke access: {}", e),
    }

    println!("\n✓ Example completed!");

    Ok(())
}
