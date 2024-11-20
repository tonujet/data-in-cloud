use ia_11_vorobei_ant::error;

#[tokio::main]
async fn main() -> error::InternalResult<()> {
    ia_11_vorobei_ant::main().await
}
