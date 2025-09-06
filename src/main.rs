use color_eyre::Result;
use pi_stats::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new()
        .with_tick_duration(std::time::Duration::from_millis(1000))
        .run(terminal)
        .await;
    ratatui::restore();
    app_result
}
