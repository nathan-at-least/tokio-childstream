#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    exoshell::cli::run().await
}
