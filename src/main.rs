use ui::cli::CliArgs;

#[macro_use]
extern crate log;

mod apex;
mod registry;
mod win_elevated;

mod ui;

const ALGS_STR: &'static str = concat!("ALGS - Respects ALGS ruleset (as of ", env!("PKG_BUILD_DATE"), ") with the most optimizations possible");
const ALGS_STR_SHORT: &'static str = concat!("ALGS (", env!("PKG_BUILD_DATE"), ")");

#[cfg(not(windows))]
fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    error!("This program is intended to be ran on Windows only, as Apex Legends is a Windows-only game too.");
    Ok(())
}

#[cfg(windows)]
#[paw::main]
fn main(args: CliArgs) -> std::io::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.filter(None, log::LevelFilter::Trace);
    log_builder.init();

    if ui::cli::start_cli(args)? {
        return ui::gui::start_gui();
    }

    Ok(())
}
