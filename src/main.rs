use lib_oracle::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
