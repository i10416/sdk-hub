use piano_handwritten_api::{publisher::app::GetAppRequest, PianoAPI};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Piano API client from environment variables
    let client = PianoAPI::from_env();

    // Get app details
    println!("Getting app details...");
    let request = GetAppRequest::new();
    let app_result = client.get_app(&request).await.expect("Failed to get app");

    let app = &app_result.app;
    println!("App Details:");
    println!("  ID: {}", app.aid());
    println!("  Name: {}", app.name());
    println!("  Email: {}", app.email());
    println!("  URL: {}", app.url());
    println!("  Default Language: {}", app.default_lang());
    println!("  Email Language: {}", app.email_lang());
    println!("  User Provider: {:?}", app.user_provider());
    println!("  State: {:?}", app.state());
    println!("  Is Active: {}", app.is_active());

    if let Some(details) = app.details() {
        println!("  Details: {}", details);
    } else {
        println!("  Details: None");
    }

    println!("  Primary Logo: {}", app.logo1());

    if let Some(logo2) = app.logo2() {
        println!("  Secondary Logo: {}", logo2);
    } else {
        println!("  Secondary Logo: None");
    }

    Ok(())
}
