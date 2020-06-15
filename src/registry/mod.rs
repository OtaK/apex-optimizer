mod fix_dvr;
pub use self::fix_dvr::*;

mod mousefix;
pub use self::mousefix::*;

mod tcp_tweaks;
pub use self::tcp_tweaks::*;

mod gaming_tweaks;
pub use self::gaming_tweaks::*;

mod timer_tweaks;
pub use self::timer_tweaks::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WindowsFix {
    FSE = 0,
    MouseFix = 1,
    TCP = 2,
    Gaming = 3,
    Timer = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, druid::Data, druid::Lens)]
pub struct WindowsFixes {
    pub fse: bool,
    pub mousefix: bool,
    pub tcp: bool,
    pub gaming: bool,
    pub timer: bool,
}

impl From<Vec<usize>> for WindowsFixes {
    fn from(v: Vec<usize>) -> Self {
        Self {
            fse: v.contains(&(WindowsFix::FSE as usize)),
            mousefix: v.contains(&(WindowsFix::MouseFix as usize)),
            tcp: v.contains(&(WindowsFix::TCP as usize)),
            gaming: v.contains(&(WindowsFix::Gaming as usize)),
            timer: v.contains(&(WindowsFix::Timer as usize)),
        }
    }
}

impl std::ops::Index<WindowsFix> for WindowsFixes {
    type Output = bool;
    fn index(&self, f: WindowsFix) -> &Self::Output {
        match f {
            WindowsFix::FSE => &self.fse,
            WindowsFix::MouseFix => &self.mousefix,
            WindowsFix::TCP => &self.tcp,
            WindowsFix::Gaming => &self.gaming,
            WindowsFix::Timer => &self.timer,
        }
    }
}

impl std::ops::IndexMut<WindowsFix> for WindowsFixes {
    fn index_mut(&mut self, f: WindowsFix) -> &mut Self::Output {
        match f {
            WindowsFix::FSE => &mut self.fse,
            WindowsFix::MouseFix => &mut self.mousefix,
            WindowsFix::TCP => &mut self.tcp,
            WindowsFix::Gaming => &mut self.gaming,
            WindowsFix::Timer => &mut self.timer,
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
            "Fixed Timer & HPET Off - Improves FPS & system latency a LOT. Requires a program (TimerSet) to set a fixed 0.5ms timer at boot."
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
                reboot_required = true;
            }

            if fixes.gaming {
                crate::registry::apply_gaming_tweaks(pretend)?;
                reboot_required = true;
            }

            if fixes.timer {
                crate::registry::apply_timer_tweaks(pretend)?;
                reboot_required = true;
            }
        }

        Ok(reboot_required)
    }
}
