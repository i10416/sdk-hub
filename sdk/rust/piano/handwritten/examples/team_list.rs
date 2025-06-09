use piano_handwritten_api::publisher::team::Permission;
use piano_handwritten_api::{publisher::team::ListTeamRequest, PianoAPI};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Piano API client from environment variables
    let client = PianoAPI::from_env();

    // List all team members
    println!("Listing all team members...");
    let request = ListTeamRequest::new();
    let team_members = client
        .list_team_members(&request)
        .await
        .expect("Failed to list team members");

    println!("Found {} team members:", team_members.team_members.len());
    for member in &team_members.team_members {
        println!(
            "- {} {} ({}) - {} permissions",
            member.first_name(),
            member.last_name(),
            member.email(),
            member.permissions().len()
        );
    }

    // List team members with specific permissions filter
    println!("\nListing team members with business management permissions...");
    let filtered_request = ListTeamRequest::new().with_permissions(&[Permission::ManageBusiness]);

    let filtered_members = client
        .list_team_members(&filtered_request)
        .await
        .expect("Failed to list team members");

    println!(
        "Found {} team members with business management permissions:",
        filtered_members.team_members.len()
    );
    for member in &filtered_members.team_members {
        println!(
            "- {} {} ({}) - Permissions: {:?}",
            member.first_name(),
            member.last_name(),
            member.email(),
            member.permissions()
        );
    }

    Ok(())
}
