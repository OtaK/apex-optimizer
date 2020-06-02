use winreg::enums::*;

pub fn apply_fse_fix(pretend: bool) -> std::io::Result<()> {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let gdvr = hklm.open_subkey_with_flags(
        "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR",
        KEY_SET_VALUE,
    )?;

    debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR = dword:0");
    if !pretend {
        gdvr.set_value("AllowGameDVR", &0u32)?;
    }

    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);

    let gbar_t = winreg::transaction::Transaction::new()?;
    let gbar = hkcu.open_subkey_transacted_with_flags(
        "Software\\Microsoft\\GameBar",
        &gbar_t,
        KEY_SET_VALUE,
    )?;

    debug!("Writing reg key: HKEY_CURRENT_USER\\Software\\Microsoft\\GameBar\\ShowStartupPanel = dword:0");
    gbar.set_value("ShowStartupPanel", &0u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\Software\\Microsoft\\GameBar\\GamePanelStartupTipIndex = dword:3");
    gbar.set_value("GamePanelStartupTipIndex", &3u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\Software\\Microsoft\\GameBar\\AllowAutoGameMode = dword:0");
    gbar.set_value("AllowAutoGameMode", &0u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\Software\\Microsoft\\GameBar\\UseNexusForGameBarEnabled = dword:0");
    gbar.set_value("UseNexusForGameBarEnabled", &0u32)?;
    debug!("Committing reg keys at HKEY_CURRENT_USER\\Software\\Microsoft\\GameBar");
    if !pretend {
        gbar_t.commit()?;
    }

    let gcstore_t = winreg::transaction::Transaction::new()?;
    let gcstore = hkcu.open_subkey_transacted_with_flags(
        "System\\GameConfigStore",
        &gcstore_t,
        KEY_SET_VALUE,
    )?;
    debug!(
        "Writing reg key: HKEY_CURRENT_USER\\System\\GameConfigStore\\GameDVR_Enabled = dword:0"
    );
    gcstore.set_value("GameDVR_Enabled", &0u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\System\\GameConfigStore\\GameDVR_FSEBehaviorMode = dword:2");
    gcstore.set_value("GameDVR_FSEBehaviorMode", &2u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\System\\GameConfigStore\\GameDVR_FSEBehavior = dword:2");
    gcstore.set_value("GameDVR_FSEBehavior", &2u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\System\\GameConfigStore\\GameDVR_HonorUserFSEBehaviorMode = dword:1");
    gcstore.set_value("GameDVR_HonorUserFSEBehaviorMode", &1u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\System\\GameConfigStore\\GameDVR_DXGIHonorFSEWindowsCompatible = dword:1");
    gcstore.set_value("GameDVR_DXGIHonorFSEWindowsCompatible", &1u32)?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\System\\GameConfigStore\\GameDVR_EFSEFeatureFlags = dword:0");
    gcstore.set_value("GameDVR_EFSEFeatureFlags", &0u32)?;
    debug!("Committing reg keys at HKEY_CURRENT_USER\\System\\GameConfigStore");
    if !pretend {
        gcstore_t.commit()?;
    }

    Ok(())
}
