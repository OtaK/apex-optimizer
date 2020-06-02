pub mod audio;
pub mod gameplay;
pub mod graphics;
pub mod misc;
pub mod network;
pub mod privacy;

#[derive(Debug, Clone, Copy)]
pub struct AutoExec(super::OptimizationLevel);

impl From<super::OptimizationLevel> for AutoExec {
    fn from(v: super::OptimizationLevel) -> Self {
        Self(v)
    }
}

impl std::fmt::Display for AutoExec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 == super::OptimizationLevel::Default {
            return Ok(());
        }

        gameplay::GAMEPLAY_AE.fmt(f)?;
        audio::AUDIO_AE.fmt(f)?;
        network::NETWORK_AE.fmt(f)?;
        privacy::PRIVACY_AE.fmt(f)?;
        audio::AUDIO_AE.fmt(f)?;

        match self.0 {
            super::OptimizationLevel::Safe => {
                graphics::GRAPHICS_AE.fmt(f)?;
            }
            super::OptimizationLevel::Performance => {
                graphics::GRAPHICS_AE.fmt(f)?;
                graphics::GRAPHICS_MISC_AE.fmt(f)?;
                misc::OPTIMS_AE.fmt(f)?;
                misc::SHADOWS_AE.fmt(f)?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl AutoExec {
    pub fn write(&mut self) -> std::io::Result<()> {
        use std::io::Write as _;

        let mut apex_dir = crate::apex::apex_install_dir()?;
        apex_dir.push("cfg");
        apex_dir.push("autoexec.cfg");

        let mut file = std::fs::File::create(apex_dir)?;
        writeln!(file, "{}", self)?;
        file.flush()
    }
}
