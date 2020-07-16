pub mod audio;
pub mod gameplay;
pub mod graphics;
pub mod misc;
pub mod network;
pub mod privacy;

#[derive(Debug, Clone, Copy)]
pub struct AutoExec {
    level: crate::apex::OptimizationLevel,
    pub letterbox_ratio: Option<f32>,
}

impl From<crate::apex::OptimizationLevel> for AutoExec {
    fn from(level: crate::apex::OptimizationLevel) -> Self {
        Self { level, letterbox_ratio: None }
    }
}

impl std::fmt::Display for AutoExec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.level == crate::apex::OptimizationLevel::Default {
            return Ok(());
        }

        if let Some(lb_ratio) = self.letterbox_ratio {
            writeln!(f, "// Letterbox Fixes")?;
            writeln!(f, "mat_letterbox_aspect_goal \"{:.4}\"", lb_ratio)?;
            writeln!(f, "mat_letterbox_aspect_threshold \"{:.4}\"", lb_ratio)?;
        }

        if self.level == crate::apex::OptimizationLevel::ALGS {
            return Ok(());
        }

        gameplay::GAMEPLAY_AE.fmt(f)?;
        audio::AUDIO_AE.fmt(f)?;
        network::NETWORK_AE.fmt(f)?;
        privacy::PRIVACY_AE.fmt(f)?;

        match self.level {
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
