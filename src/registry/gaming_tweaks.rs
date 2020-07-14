use winreg::enums::*;

pub fn apply_gaming_tweaks(pretend: bool) -> std::io::Result<()> {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let sys_profile_t = winreg::transaction::Transaction::new()?;
    let (sys_profile, _) = hklm.create_subkey_transacted_with_flags(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
        &sys_profile_t,
        KEY_WRITE,
    )?;

    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\NetworkThrottlingIndex = dword:0xffffffff");
    sys_profile.set_value("NetworkThrottlingIndex", &0xffff_ffff_u32)?;
    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\SystemResponsiveness = dword:5");
    sys_profile.set_value("SystemResponsiveness", &5u32)?;

    if !pretend {
        sys_profile_t.commit()?;
    }

    let games_profile_t = winreg::transaction::Transaction::new()?;
    let (games_profile, _) = hklm.create_subkey_transacted_with_flags(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
        &games_profile_t,
        KEY_WRITE,
    )?;

    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\\GPU Priority = dword:8");
    games_profile.set_value("GPU Priority", &8u32)?;
    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\\Priority = dword:8");
    games_profile.set_value("Priority", &8u32)?;
    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\\Scheduling Priority = sz:High");
    games_profile.set_value("Scheduling Category", &"High")?;
    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games\\SFIO Priority = sz:High");
    games_profile.set_value("SFIO Priority", &"High")?;

    if !pretend {
        games_profile_t.commit()?;
    }

    Ok(())
}
