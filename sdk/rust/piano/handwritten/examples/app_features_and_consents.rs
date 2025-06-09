use piano_handwritten_api::{
    publisher::{
        app::{GetAppFeaturesRequest, ListAppsRequest},
        consent::{GetConsentRequest, ListConsentsRequest},
    },
    PianoAPI,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Piano API client from environment variables
    let client = PianoAPI::from_env();

    // 1. Get app features
    println!("=== Getting App Features ===");
    let features_request = GetAppFeaturesRequest::new();
    let features_result = client
        .get_app_features(&features_request)
        .await
        .expect("Failed to get app features");
    let features = &features_result.app_features;

    println!("My Account Enabled: {}", features.is_my_account_enabled());
    println!("Composer Enabled: {}", features.is_composer_enabled());
    println!(
        "Redemption Page Enabled: {}",
        features.is_redemption_page_enabled()
    );
    println!(
        "Payment Mock Enabled: {}",
        features.is_payment_mock_enabled()
    );
    println!(
        "Dashboard Localization Enabled: {}",
        features.is_dashboard_localization_enabled()
    );

    let restrictions = features.subscription_restrictions();
    println!(
        "Can switch payment method: {}",
        restrictions.allow_switch_payment_method
    );
    println!(
        "Can enable auto renew: {}",
        restrictions.allow_enable_auto_renew
    );

    // 2. List apps
    println!("\n=== Listing Apps ===");
    let apps_request = ListAppsRequest::new();
    let apps_result = client
        .list_apps(&apps_request)
        .await
        .expect("Failed to list apps");

    println!(
        "Found {} apps (total: {}):",
        apps_result.value.apps.len(),
        apps_result.total
    );
    for app in &apps_result.value.apps {
        println!("- {} ({}) - {:?}", app.name(), app.aid(), app.state());
    }

    // 3. List consents
    println!("\n=== Listing Consents ===");
    let consents_request = ListConsentsRequest::new(0, 10);
    let consents_result = client
        .list_consents(&consents_request)
        .await
        .expect("Failed to list consents");

    println!(
        "Found {} consents (total: {}):",
        consents_result.value.consents.len(),
        consents_result.total
    );
    for consent in &consents_result.value.consents {
        println!(
            "- {} ({}) - Type: {:?}, Required: {}, Enabled: {}",
            consent.field_name(),
            consent.consent_id(),
            consent.consent_type(),
            consent.is_required(),
            consent.is_enabled()
        );
    }

    // 4. Get specific consent (using the first one from the list)
    if let Some(first_consent) = consents_result.value.consents.first() {
        println!("\n=== Getting Specific Consent ===");
        let consent_request = GetConsentRequest::new(first_consent.consent_id());
        let consent_result = client
            .get_consent(&consent_request)
            .await
            .expect("Failed to get consent");

        let consent = &consent_result.consent;
        println!("Consent Details:");
        println!("  ID: {}", consent.consent_id());
        println!("  Name: {}", consent.field_name());
        println!("  Field ID: {}", consent.field_id());
        println!("  Type: {:?}", consent.consent_type());
        println!("  Pre-checked: {}", consent.is_pre_checked());
        println!("  Required: {}", consent.is_required());
        println!("  Enabled: {}", consent.is_enabled());
        println!("  Error Message: {}", consent.error_message());
        println!(
            "  Display Text (truncated): {}...",
            &consent.display_text().chars().take(100).collect::<String>()
        );
    }

    Ok(())
}
