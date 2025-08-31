use pi_stats::{App, Error};

fn main() -> Result<(), Error> {
    let app = App::new().with_ctrlc_handler()?;

    while !app.exit() {
        app.refresh();
    }

    Ok(())
}
