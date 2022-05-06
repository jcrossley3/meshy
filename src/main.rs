#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    env_logger::init();
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    println!(
        "Bluetooth adapter {} with address {}",
        adapter.name(),
        adapter.address().await?
    );
    Ok(())
}
