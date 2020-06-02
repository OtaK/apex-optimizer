mod fix_dvr;
pub use self::fix_dvr::*;

mod mousefix;
pub use self::mousefix::*;

mod tcp_tweaks;
pub use self::tcp_tweaks::*;

mod gaming_tweaks;
pub use self::gaming_tweaks::*;

#[derive(Debug, Copy, Clone)]
enum WindowsFix {
    FSE = 0,
    MouseFix = 1,
    TCP = 2,
    Gaming = 3,
}

#[derive(Debug, Copy, Clone, Default)]
struct WindowsFixes {
    fse: bool,
    mousefix: bool,
    tcp: bool,
    gaming: bool,
}

impl From<Vec<usize>> for WindowsFixes {
    fn from(v: Vec<usize>) -> Self {
        Self {
            fse: v.contains(&(WindowsFix::FSE as usize)),
            mousefix: v.contains(&(WindowsFix::MouseFix as usize)),
            tcp: v.contains(&(WindowsFix::TCP as usize)),
            gaming: v.contains(&(WindowsFix::Gaming as usize)),
        }
    }
}

pub mod prompt {
    pub fn registry_prompt<T: dialoguer::theme::Theme>(
        theme: &T,
        pretend: bool,
    ) -> std::io::Result<bool> {
        let mut reboot_required = false;
        let mut windows_cfg_prompt = dialoguer::Checkboxes::with_theme(theme);
        windows_cfg_prompt
            .with_prompt("[WINDOWS] Please select the set of fixes you want to apply");
        windows_cfg_prompt.items(&[
            "Exclusive FullScreen/GameDVR - Tells Windows to respect the Exclusive fullscreen setting. Reduces input lag.",
            "MouseFix - Registry tweak to tell windows to stop altering your mouse inputs. Requires 6/11 mouse speed setting in the Control Panel",
            "TCP / Nagling tweaks - Disable Nagle's algorithm and optimizes TCP handling for modern/gaming workloads",
            "Gaming Tweaks - Improves system responsiveness when using games. Might reduce input lag/latency when gaming & improve performance",
        ]);

        if let Ok(fixes) = windows_cfg_prompt.interact().map(super::WindowsFixes::from) {
            debug!("reg fixes selected: {:?}", fixes);

            if fixes.fse {
                crate::registry::apply_fse_fix(pretend)?;
                reboot_required = true;
            }

            if fixes.mousefix {
                crate::registry::apply_mousefix(pretend)?;
                reboot_required = true;
            }

            if fixes.tcp {
                crate::registry::apply_tcp_tweaks(pretend)?;
            }

            if fixes.gaming {
                crate::registry::apply_gaming_tweaks(pretend)?;
            }
        }

        Ok(reboot_required)
    }
}
