/// Example demonstrating Piano SDK user management functionality.
///
/// This example shows how to:
/// - Create users
/// - Update user information
/// - Retrieve user details
/// - Search for users
/// - List users with pagination
use piano_handwritten_api::{publisher::user::*, PianoAPI};

#[tokio::main]
async fn main() -> Result<(), piano_handwritten_api::Error> {
    // Initialize the Piano API client
    // In production, these would come from environment variables or configuration
    let api = PianoAPI::new(
        "https://sandbox.piano.io/api/v3", // Use sandbox for testing
        "your_app_id",
        "your_api_token",
    );

    println!("Piano SDK User Management Example");
    println!("=================================");

    // Example 1: Create a new user
    println!("\n1. Creating a new user...");
    let create_request = CreateUserRequest::new("test.user@example.com")
        .with_first_name("Test")
        .with_last_name("User")
        .with_phone("+1-555-0123");

    match api.create_user(&create_request).await {
        Ok(user) => {
            println!("✓ User created successfully!");
            println!("  UID: {}", user.uid());
            println!("  Email: {}", user.email());
            println!(
                "  Name: {} {}",
                user.first_name().unwrap_or(""),
                user.last_name().unwrap_or("")
            );
        }
        Err(e) => println!("✗ Failed to create user: {}", e),
    }

    // Example 2: Get user by UID
    println!("\n2. Retrieving user information...");
    let user_uid = "example_user_uid"; // Replace with actual UID

    match api.get_user(user_uid).await {
        Ok(Some(user)) => {
            println!("✓ User found!");
            println!("  UID: {}", user.uid());
            println!("  Email: {}", user.email());
            if let Some(create_date) = user.create_date() {
                println!("  Created: {}", create_date);
            }
        }
        Ok(None) => println!("ℹ User not found"),
        Err(e) => println!("✗ Failed to get user: {}", e),
    }

    // Example 3: Update user information
    println!("\n3. Updating user information...");
    let update_request = UpdateUserRequest::new(user_uid)
        .with_email("updated.email@example.com")
        .with_first_name("Updated")
        .with_phone("+1-555-9999");

    match api.update_user(&update_request).await {
        Ok(user) => {
            println!("✓ User updated successfully!");
            println!("  New email: {}", user.email());
            println!("  New first name: {}", user.first_name().unwrap_or(""));
        }
        Err(e) => println!("✗ Failed to update user: {}", e),
    }

    // Example 4: Search for users by email
    println!("\n4. Searching for users...");
    let search_request = SearchUserRequest::new()
        .with_email("test.user@example.com")
        .with_limit(10);

    match api.search_users(&search_request).await {
        Ok(results) => {
            println!("✓ Search completed!");
            println!("  Found {} users", results.value.users.len());
            for user in &results.value.users {
                println!("    - {} ({})", user.email(), user.uid());
            }
        }
        Err(e) => println!("✗ Search failed: {}", e),
    }

    // Example 5: List users with pagination
    println!("\n5. Listing users with pagination...");
    let list_request = ListUserRequest::new()
        .with_limit(5)
        .with_offset(0)
        .with_order_by("create_date")
        .with_order_direction("desc");

    match api.list_users(&list_request).await {
        Ok(results) => {
            println!("✓ User list retrieved!");
            println!("  Total users: {}", results.total);
            println!("  Showing {} of {} users:", results.count, results.total);

            for (i, user) in results.value.users.iter().enumerate() {
                println!(
                    "    {}. {} - {} ({})",
                    i + 1,
                    user.email(),
                    user.first_name().unwrap_or("No name"),
                    user.uid()
                );
            }

            // Show pagination info
            if results.total > results.count {
                println!(
                    "  Use offset {} to see more results",
                    results.offset + results.count
                );
            }
        }
        Err(e) => println!("✗ Failed to list users: {}", e),
    }

    // Example 6: Advanced filtering
    println!("\n6. Advanced user filtering...");
    let filtered_request = ListUserRequest::new()
        .with_query("test") // Search for users with "test" in their data
        .with_status("ACTIVE")
        .with_limit(10);

    match api.list_users(&filtered_request).await {
        Ok(results) => {
            println!("✓ Filtered search completed!");
            println!(
                "  Found {} active users matching 'test'",
                results.value.users.len()
            );
        }
        Err(e) => println!("✗ Filtered search failed: {}", e),
    }

    println!("\n✓ Example completed!");

    Ok(())
}
