use config::Config;
mod config;
mod log;

fn main() -> anyhow::Result<()> {
    let conf = Config::from_any().unwrap();
    log::setup(&conf.log);
    
    // Setup gui

    Ok(())
}