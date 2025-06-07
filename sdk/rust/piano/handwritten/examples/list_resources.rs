use piano_handwritten_api::{publisher::resource::*, PianoAPI};

#[tokio::main]
async fn main() -> Result<(), piano_handwritten_api::Error> {
    let api = PianoAPI::from_env();

    let head = api
        .list_resources(&ListResourceRequest::new().with_limit(5))
        .await?;
    println!("Resources: {:?}", head);
    Ok(())
}
