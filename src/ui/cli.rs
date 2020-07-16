#[derive(Debug, structopt::StructOpt)]
#[structopt(author = "Mathieu Amiot <amiot.mathieu@gmail.com>")]
pub struct CliArgs {
    #[structopt(long)]
    /// Disables GUI and switches to the cli interface
    cli: bool,

    #[structopt(short, long)]
    /// Do not do anything, just pretend and write out what will be done to your system
    pretend: bool,

    #[structopt(short, long)]
    /// Don't perform incremental config backups, but please be aware of what you're getting into
    no_backup: bool,

    #[structopt(long)]
    /// Only perform a backup and then exit
    only_backup: bool,

    #[structopt(subcommand)]
    /// Apply tweaks to system/apex
    cmd: Option<TweakArgs>,
}
#[derive(Debug, structopt::StructOpt)]
pub enum TweakArgs {
    Apex(ApexArgs),
    Registry(RegistryArgs),
}

#[derive(Debug, structopt::StructOpt, Default)]
pub struct RegistryArgs {
    #[structopt(long)]
    /// TCP / Nagling tweaks - Disable Nagle's algorithm and optimizes TCP handling for modern/gaming workloads
    tcp: bool,

    #[structopt(long)]
    /// Exclusive FullScreen/GameDVR - Tells Windows to respect the Exclusive fullscreen setting. Reduces input lag.
    fse: bool,

    #[structopt(long)]
    /// Fixed Timer & HPET Off - Improves FPS & system latency a LOT. Requires a program (TimerSet) to set a fixed 0.5ms timer at boot.
    timer: bool,

    #[structopt(long)]
    /// MouseFix - Registry tweak to tell windows to stop altering your mouse inputs. Requires 6/11 mouse speed setting in the Control Panel
    mousefix: bool,

    #[structopt(long)]
    /// Gaming Tweaks - Improves system responsiveness when using games. Might reduce input lag/latency when gaming & improve performance
    gaming: bool,
}

#[derive(Debug, structopt::StructOpt, Default)]
/// Apex optimizations. Levels are "safe", "performance|perf", "algs" or "default".
/// An empty argument also acts as "default", which reverts your configuration to Apex's default configs.
pub struct ApexArgs {
    #[structopt(long)]
    /// Autoexec level
    autoexec: Option<crate::apex::OptimizationLevel>,
    #[structopt(long)]
    /// VideoConfig level
    videoconfig: Option<crate::apex::OptimizationLevel>,
}

pub fn start_cli(args: CliArgs) -> std::io::Result<bool> {
    if !args.cli {
        return Ok(true);
    }

    if args.pretend {
        info!("Pretend mode enabled, no actions will be taken on your system");
    }

    if !crate::win_elevated::is_app_elevated() {
        error!("The app has not been launched with administrator permissions, please run it from an admin terminal");
        return Ok(false);
    }

    if !args.no_backup {
        info!("Performing incremental config backup...");
        crate::apex::backup::incremental_backup()?;
        info!("Backup completed!");
        if args.only_backup {
            return Ok(false);
        }
    } else {
        info!("Skipping incremental backup...");
    }

    let reboot_required = if let Some(tweaks) = args.cmd {
        apply_tweaks(tweaks, args.pretend)?
    } else { // Interactive mode
        let theme = dialoguer::theme::ColorfulTheme::default();
        let reboot_required = crate::registry::prompt::registry_prompt(&theme, args.pretend)?;

        if dialoguer::Confirmation::new()
            .with_text("Do you want to apply custom configs to Apex?")
            .interact()?
        {
            crate::apex::prompt::apex_prompt(&theme, args.pretend)?;
        } else {
            info!("Skipping Apex configs...");
        }

        reboot_required
    };

    if reboot_required {
        info!("You should now reboot your computer for the fixes to take effect.");
    }

    Ok(false)
}

fn apply_tweaks(tweaks: TweakArgs, pretend: bool) -> std::io::Result<bool> {
    let reboot_required = match tweaks {
        TweakArgs::Apex(apex_args) => {
            let mut wrote_autoexec = false;

            let video_config = if let Some(mut config) = apex_args.videoconfig.map(crate::apex::VideoConfig::from) {
                config.detect_res()?;
                if !pretend {
                    config.write()?;
                }

                config
            } else {
                let mut config = crate::apex::VideoConfig::defaults_algs();
                config.detect_res()?;
                config
            };

            if let Some(mut autoexec) = apex_args.autoexec.map(crate::apex::AutoExec::from) {
                autoexec.letterbox_ratio = video_config.letterbox_ratio;
                if !pretend {
                    autoexec.write()?;
                }

                wrote_autoexec = true;
            }

            info!("All done! You can now go to Origin, right-click on Apex Legends > Game Properties, and in the Advanced Launch Options tab, paste the next line! (also set your game to english ;))");
            info!("{}", crate::apex::generate_launch_args(&video_config, wrote_autoexec));

            false
        },
        TweakArgs::Registry(registry_args) => {
            let mut reboot_required = false;
            if registry_args.fse {
                crate::registry::apply_fse_fix(pretend)?;
                reboot_required = true;
            }

            if registry_args.mousefix {
                crate::registry::apply_mousefix(pretend)?;
                reboot_required = true;
            }

            if registry_args.tcp {
                crate::registry::apply_tcp_tweaks(pretend)?;
                reboot_required = true;
            }

            if registry_args.gaming {
                crate::registry::apply_gaming_tweaks(pretend)?;
                reboot_required = true;
            }

            if registry_args.timer {
                crate::registry::apply_timer_tweaks(pretend)?;
                reboot_required = true;
            }

            reboot_required
        }
    };

    Ok(reboot_required)
}
