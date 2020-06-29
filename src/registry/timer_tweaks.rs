pub fn apply_timer_tweaks(pretend: bool) -> std::io::Result<()> {
    debug!("Setting Fixed Timer BCD Flag: disabledynamictick = yes");
    if !pretend {
        let _ = std::process::Command::new("bcdedit")
            .args(&["/set", "disabledynamictick", "yes"])
            .output()?;
    }

    debug!("Setting HPET Off BCD Flag: useplatformclock = no");
    if !pretend {
        let _ = std::process::Command::new("bcdedit")
            .args(&["/set", "useplatformclock", "no"])
            .output()?;
    }

    Ok(())
}
