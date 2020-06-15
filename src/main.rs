#[macro_use]
extern crate log;

mod apex;
mod registry;
mod win_elevated;

mod ui;

use clap::{clap_app, crate_authors, crate_description, crate_version};

const ALGS_STR: &'static str = concat!("ALGS - Respects ALGS ruleset (as of ", env!("PKG_BUILD_DATE"), ") with the most optimizations possible");

#[cfg(not(windows))]
fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    error!("This program is intended to be ran on Windows only, as Apex Legends is a Windows-only game too.");
    Ok(())
}

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.filter(None, log::LevelFilter::Trace);
    log_builder.init();
    ui::gui::start_gui()
}

/*#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let matches = clap_app!(apex_optimizer =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg pretend: --pretend -p "Do not do anything, just pretend and write out what will be done to your system")
        (@arg no_backup: --no_backup -n "Don't perform config backups, but please be aware of what you're getting into")
    ).get_matches();

    let pretend = matches.is_present("pretend");
    let no_backup = matches.is_present("no_backup");
    let mut log_builder = pretty_env_logger::formatted_builder();

    if let Ok(s) = ::std::env::var("RUST_LOG") {
        log_builder.parse_filters(&s);
    } else {
        log_builder.filter(None, log::LevelFilter::Info);
    }

    if pretend {
        log_builder.filter(None, log::LevelFilter::Trace);
    }

    log_builder.init();

    if pretend {
        info!("Pretend mode enabled, no actions will be taken on your system");
    }

    if !win_elevated::is_app_elevated() {
        error!("The app has not been launched with administrator permissions, please run it from an admin terminal");
        return Ok(());
    }

    if !no_backup {
        info!("Performing incremental config backup...");
        apex::backup::incremental_backup()?;
        info!("Backup completed!");
    } else {
        info!("Skipping incremental backup...");
    }

    let theme = dialoguer::theme::ColorfulTheme::default();

    let reboot_required = registry::prompt::registry_prompt(&theme, pretend)?;

    if dialoguer::Confirmation::new()
        .with_text("Do you want to apply custom configs to Apex?")
        .interact()?
    {
        apex::prompt::apex_prompt(&theme, pretend)?;
    } else {
        info!("Skipping Apex configs...");
    }

    if reboot_required {
        info!("You should now reboot your computer for the fixes to take effect.");
    }

    Ok(())
}*/
