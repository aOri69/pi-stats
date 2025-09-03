use pi_stats::{App, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    App::new()
        .with_tick_duration(std::time::Duration::from_millis(500))
        .run()
        .await;
    Ok(())
}
