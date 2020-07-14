const TIMERSET_RELEASE_URL: &str = "https://github.com/OtaK/timertool-rs/releases/download/v0.2.0/timerset_windows_v0.2.0.exe";

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

    info!("Downloading TimerSet...");
    if let Ok(mut res) = reqwest::blocking::get(TIMERSET_RELEASE_URL) {
        let tmp_dir = tempfile::Builder::new().prefix("apex_optimizer").tempdir()?;
        let exe_name = std::path::Path::new(TIMERSET_RELEASE_URL).file_name();
        let timerset_location = tmp_dir
            .path()
            .join(
                exe_name
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("timerset_windows_v0.2.0.exe")
            );
        debug!("TimerSet location: {:?}", timerset_location);

        let mut dest = std::fs::File::create(timerset_location.clone())?;

        if res.copy_to(&mut dest).is_err() {
            error!("Could not write temporary file, any disk space issue?");
        } else {
            drop(res);
            drop(dest);

            info!("Waiting 5 seconds for installation to finish...");
            if !pretend {
                let mut timerset_install = std::process::Command::new(timerset_location)
                    .arg("--install")
                    .spawn()?;
                std::thread::sleep(std::time::Duration::from_secs(5));
                let _ = timerset_install.kill();
            }
        }
    } else {
        error!("Could not download TimerSet! No internet connection?");
    }

    info!("TimerSet Successfully installed");

    Ok(())
}
