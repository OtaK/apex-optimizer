#![allow(dead_code)]

mod videoconfig;
pub use self::videoconfig::*;

mod autoexec;
pub use self::autoexec::*;

mod optim_level;
pub use self::optim_level::*;

pub mod backup;
pub mod prompt;

pub fn apex_user_dir() -> std::io::Result<std::path::PathBuf> {
    let mut ret =
        dirs::home_dir().ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;

    ret.push("Saved Games");
    ret.push("Respawn");
    ret.push("Apex");

    Ok(ret)
}

#[allow(dead_code)]
pub fn apex_install_dir() -> std::io::Result<std::path::PathBuf> {
    use winreg::enums::*;
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let apex = hklm.open_subkey("SOFTWARE\\Respawn\\Apex")?;
    Ok(apex.get_value::<String, &str>("Install Dir")?.into())
}
