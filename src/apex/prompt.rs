use super::OptimizationLevel;

pub fn apex_prompt<T: dialoguer::theme::Theme>(theme: &T, pretend: bool) -> std::io::Result<()> {
    let mut apex_videoconfig_prompt = dialoguer::Select::with_theme(theme);
    apex_videoconfig_prompt.with_prompt("[APEX-VIDEOCONFIG] Please select a level of optimization: ");
    apex_videoconfig_prompt.items(&[
        "Performance - Game looks like trash. Might be unstable and/or reduce visibility, but FPS is maxed out.",
        "Safe - Crash-safe videoconfig with a few optims here and there",
        &format!("ALGS - Respects ALGS ruleset (as of {}) with the most optimizations possible", env!("PKG_BUILD_DATE")),
        "Default - Deletes the custom videoconfig",
    ]);

    let video_config = if let Ok(level) = apex_videoconfig_prompt.interact().map(OptimizationLevel::from) {
        let mut config = match level {
            OptimizationLevel::ALGS => crate::apex::VideoConfig::defaults_algs(),
            OptimizationLevel::Performance => crate::apex::VideoConfig::defaults_performance(),
            OptimizationLevel::Safe => crate::apex::VideoConfig::defaults_safe(),
            OptimizationLevel::Default => crate::apex::VideoConfig::default(),
        };

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

    debug!("Detected main screen settings: {}x{}@{}", video_config.screen_width, video_config.screen_height, video_config.screen_refresh_rate);

    let mut apex_autoexec_prompt = dialoguer::Select::with_theme(theme);
    apex_autoexec_prompt.with_prompt("[APEX-AUTOEXEC] Please select a kind of autoexec: ");
    apex_autoexec_prompt.items(&[
        "Performance - Good FPS gains. Might be unstable on some systems. Do not use in competitive.",
        "Safe - Crash-safe values with small FPS gains. Probably banned in competitive as well.",
        &format!("ALGS - Respects ALGS ruleset (as of {}) with the most optimizations possible", env!("PKG_BUILD_DATE")),
        "Default - Deletes the custom autoexec",
    ]);

    let mut wrote_autoexec = false;

    if let Ok(level) = apex_autoexec_prompt.interact().map(OptimizationLevel::from) {
        debug!("Writing new autoexec ({:?})", level);
        let mut autoexec: super::autoexec::AutoExec = level.into();
        debug!("AutoExec: {}", autoexec);
        if !pretend {
            autoexec.write()?;
        }

        wrote_autoexec = true;
    }

    info!("All done! You can now go to Origin, right-click on Apex Legends > Game Properties, and in the Advanced Launch Options tab, paste the next line! (also set your game to english ;))");
    info!(
        "{}-forcenovsync -fullscreen -high -freq {} -refresh {} +fps_max {}",
        if wrote_autoexec { "+exec autoexec " } else { "" },
        video_config.screen_refresh_rate,
        video_config.screen_refresh_rate,
        std::cmp::min(video_config.screen_refresh_rate - 1, 190), // NOTE: 190fps cap to avoid engine stuttering
    );

    Ok(())
}
