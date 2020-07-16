use super::OptimizationLevel;

pub fn apex_prompt<T: dialoguer::theme::Theme>(theme: &T, pretend: bool) -> std::io::Result<()> {
    let mut apex_videoconfig_prompt = dialoguer::Select::with_theme(theme);
    apex_videoconfig_prompt
        .with_prompt("[APEX-VIDEOCONFIG] Please select a level of optimization: ");
    apex_videoconfig_prompt.items(&[
        "Performance - Game looks like trash. Might be unstable and/or reduce visibility, but FPS is maxed out.",
        "Safe - Crash-safe videoconfig with a few optims here and there",
        crate::ALGS_STR,
        "Default - Deletes the custom videoconfig",
    ]);

    let video_config = if let Ok(level) = apex_videoconfig_prompt
        .interact()
        .map(OptimizationLevel::from)
    {
        let mut config = crate::apex::VideoConfig::from(level);

        debug!("Writing new videoconfig ({:?})", level);
        config.detect_res()?;
        debug!("VideoConfig: {:?}", config);

        if !pretend {
            config.write()?;
        }

        config
    } else {
        let mut config = crate::apex::VideoConfig::default();
        config.detect_res()?;
        config
    };

    debug!(
        "Detected main screen settings: {}x{}@{}Hz",
        video_config.screen_width, video_config.screen_height, video_config.screen_refresh_rate
    );

    let mut apex_autoexec_prompt = dialoguer::Select::with_theme(theme);
    apex_autoexec_prompt.with_prompt("[APEX-AUTOEXEC] Please select a kind of autoexec: ");
    apex_autoexec_prompt.items(&[
        "Performance - Good FPS gains. Might be unstable on some systems. Do not use in competitive.",
        "Safe - Crash-safe values with small FPS gains. Probably banned in competitive as well.",
        crate::ALGS_STR,
        "Default - Deletes the custom autoexec",
    ]);

    let mut wrote_autoexec = false;

    if let Ok(level) = apex_autoexec_prompt.interact().map(OptimizationLevel::from) {
        debug!("Writing new autoexec ({:?})", level);
        let mut autoexec: super::autoexec::AutoExec = level.into();
        autoexec.letterbox_ratio = video_config.letterbox_ratio;
        debug!("AutoExec: {}", autoexec);
        if !pretend {
            autoexec.write()?;
        }

        wrote_autoexec = true;
    }

    info!("All done! You can now go to Origin, right-click on Apex Legends > Game Properties, and in the Advanced Launch Options tab, paste the next line! (also set your game to english ;))");
    info!("{}", crate::apex::generate_launch_args(&video_config, wrote_autoexec));

    Ok(())
}
