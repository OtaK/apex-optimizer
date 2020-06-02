use winreg::enums::*;

pub fn apply_tcp_tweaks(pretend: bool) -> std::io::Result<()> {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let nics = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces",
        KEY_READ,
    )?;

    let addr_type_value = winreg::RegValue {
        bytes: vec![0u8, 0, 0, 0],
        vtype: REG_DWORD,
    };

    let mut nic_key = None;
    for maybe_nic in nics.enum_keys() {
        let nic_id = maybe_nic?;
        let nic = nics.open_subkey_with_flags(nic_id.clone(), KEY_READ)?;
        if nic
            .enum_values()
            .filter(|r| match r {
                Ok((k, v)) => k == "AddressType" && *v == addr_type_value,
                _ => false,
            })
            .count()
            > 0
        {
            nic_key = Some(nic_id);
            break;
        }
    }

    if let Some(nic_id) = nic_key {
        let nic_t = winreg::transaction::Transaction::new()?;
        let nic = nics.open_subkey_transacted_with_flags(nic_id.clone(), &nic_t, KEY_WRITE)?;
        debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}\\TcpAckFrequency = dword:1", nic_id);
        nic.set_value("TcpAckFrequency", &1u32)?;
        debug!("Writing reg key: HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}\\TCPNoDelay = dword:1", nic_id);
        nic.set_value("TCPNoDelay", &1u32)?;
        if !pretend {
            nic_t.commit()?;
        }
    } else {
        info!("Could not find your current network interface!");
    }

    Ok(())
}
