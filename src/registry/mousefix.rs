use winreg::enums::*;

pub fn apply_mousefix(pretend: bool) -> std::io::Result<()> {
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);

    let cu_mouse_transaction = winreg::transaction::Transaction::new()?;
    let cu_mouse = hkcu.open_subkey_transacted_with_flags(
        "Control Panel\\Mouse",
        &cu_mouse_transaction,
        KEY_SET_VALUE,
    )?;
    debug!("Writing reg key: HKEY_CURRENT_USER\\Control Panel\\Mouse\\MouseSensitivity = 10");
    cu_mouse.set_value("MouseSensitivity", &"10")?;

    debug!("Writing reg key: HKEY_CURRENT_USER\\Control Panel\\Mouse\\SmoothMouseXCurve = [binary]");
    cu_mouse.set_raw_value(
        "SmoothMouseXCurve",
        &winreg::RegValue {
            vtype: RegType::REG_BINARY,
            bytes: vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xC0, 0xCC, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x80, 0x99, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x40, 0x66, 0x26, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x33, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        },
    )?;

    debug!("Writing reg key: HKEY_CURRENT_USER\\Control Panel\\Mouse\\SmoothMouseYCurve = [binary]");
    cu_mouse.set_raw_value(
        "SmoothMouseYCurve",
        &winreg::RegValue {
            vtype: RegType::REG_BINARY,
            bytes: vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0xA8, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        },
    )?;

    debug!("Committing reg keys at HKEY_CURRENT_USER\\Control Panel\\Mouse");
    if !pretend {
        cu_mouse_transaction.commit()?;
    }

    let u_mouse_transaction = winreg::transaction::Transaction::new()?;
    let hku = winreg::RegKey::predef(HKEY_USERS);
    let u_mouse = hku.open_subkey_transacted_with_flags(
        ".DEFAULT\\Control Panel\\Mouse",
        &u_mouse_transaction,
        KEY_SET_VALUE,
    )?;
    debug!("Writing reg key: HKEY_USERS\\.DEFAULT\\Control Panel\\Mouse\\MouseSpeed = 0");
    u_mouse.set_value("MouseSpeed", &"0")?;

    debug!("Writing reg key: HKEY_USERS\\.DEFAULT\\Control Panel\\Mouse\\MouseThreshold1 = 0");
    u_mouse.set_value("MouseThreshold1", &"0")?;

    debug!("Writing reg key: HKEY_USERS\\.DEFAULT\\Control Panel\\Mouse\\MouseThreshold2 = 0");
    u_mouse.set_value("MouseThreshold2", &"0")?;

    debug!("Committing reg keys at HKEY_USERS\\.DEFAULT\\Control Panel\\Mouse");
    if !pretend {
        u_mouse_transaction.commit()?;
    }

    Ok(())
}
