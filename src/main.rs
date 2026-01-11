use color_eyre::Result;

mod connections;

mod app;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {

    todo!();
    /*
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::default().run(terminal);
    ratatui::restore();

    result
    */
}
