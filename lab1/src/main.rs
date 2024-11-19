use lab1::error;

#[tokio::main]
async fn main() -> error::InternalResult<()> {
    lab1::main().await
}