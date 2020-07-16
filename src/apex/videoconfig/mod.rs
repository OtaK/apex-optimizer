use super::OptimizationLevel;

const WIDESCREEN_RATIO: f32 = 16. / 9.;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct VideoConfig {
    cl_gib_allow: bool,
    cl_particle_fallback_base: u8,
    cl_particle_fallback_multiplier: u8,
    cl_ragdoll_maxcount: u8,
    cl_ragdoll_self_collision: bool,
    mat_forceaniso: u8,
    mat_mip_linear: u8,
    stream_memory: u32,
    mat_picmip: u8,
    particle_cpu_level: u8,
    r_createmodeldecals: bool,
    r_decals: u8,
    r_lod_switch_scale: f64,
    shadow_enable: bool,
    shadow_depth_dimen_min: u16,
    shadow_depth_upres_factor_max: u16,
    shadow_maxdynamic: u16,
    ssao_enabled: bool,
    ssao_downsample: u8,
    modeldecals_forceAllowed: bool,
    dvs_enable: bool,
    dvs_gpuframetime_min: u32,
    dvs_gpuframetime_max: u32,
    defaultres: u32,
    defaultresheight: u32,
    fullscreen: bool,
    nowindowborder: bool,
    volumetric_lighting: bool,
    mat_vsync_mode: bool,
    mat_backbuffer_count: u8,
    mat_antialias_mode: u8,
    csm_enabled: bool,
    csm_coverage: u8,
    csm_cascade_res: u32,
    fadeDistScale: f64,
    dvs_supersample_enable: bool,
    gamma: f64,
    configversion: u16,
    pub screen_width: u32,
    pub screen_height: u32,
    pub screen_refresh_rate: u16,
    pub letterbox_ratio: Option<f32>,
}

mod algs;
mod default;
mod performance;
mod safe;

impl From<OptimizationLevel> for VideoConfig {
    fn from(level: OptimizationLevel) -> Self {
        match level {
            OptimizationLevel::ALGS => crate::apex::VideoConfig::defaults_algs(),
            OptimizationLevel::Performance => crate::apex::VideoConfig::defaults_performance(),
            OptimizationLevel::Safe => crate::apex::VideoConfig::defaults_safe(),
            OptimizationLevel::Default => crate::apex::VideoConfig::default(),
        }
    }
}

impl VideoConfig {
    pub fn get_best_video_mode() -> std::io::Result<glutin::monitor::VideoMode> {
        let ev = glutin::event_loop::EventLoop::new();
        let m = ev.primary_monitor();
        debug!(
            "monitor: {:?}\n\t{:?} @ {:?}",
            m.name(),
            m.size(),
            m.position()
        );

        let refresh = m
            .video_modes()
            .max_by_key(|mode| mode.refresh_rate())
            .map(|mode| mode.refresh_rate())
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;

        let mode = m
            .video_modes()
            .filter(|mode| mode.refresh_rate() == refresh)
            .max_by_key(|mode| {
                let glutin::dpi::PhysicalSize::<u32> { width, height } = mode.size();
                width * height
            })
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;

        debug!("Found best video mode: {:?}", mode);
        Ok(mode)
    }

    pub fn detect_res(&mut self) -> std::io::Result<()> {
        let mode = Self::get_best_video_mode()?;

        let glutin::dpi::PhysicalSize::<u32> { width, height } = mode.size();
        let refresh_rate = mode.refresh_rate();
        self.custom_res((width, height), refresh_rate)
    }

    pub fn custom_res(&mut self, res: (u32, u32), refresh: u16) -> std::io::Result<()> {
        let screen_ratio = res.0 as f32 / res.1 as f32;
        if screen_ratio < WIDESCREEN_RATIO {
            self.letterbox_ratio = Some(screen_ratio);
        }

        self.screen_width = res.0;
        self.defaultres = res.0;
        self.screen_height = res.1;
        self.defaultresheight = res.1;

        self.screen_refresh_rate = refresh;

        let target_frametime = ((1. / self.screen_refresh_rate as f64) * 1_000_000.) as u32;
        debug!("Frametime: {}", target_frametime);
        self.dvs_gpuframetime_max = target_frametime + 200;
        self.dvs_gpuframetime_min = self.dvs_gpuframetime_max - 1500;

        Ok(())
    }

    pub fn write(&mut self) -> std::io::Result<()> {
        use std::io::Write as _;

        let mut apex_dir = crate::apex::apex_user_dir()?;
        apex_dir.push("local");
        apex_dir.push("videoconfig.txt");

        let mut file = std::fs::File::create(apex_dir)?;
        write!(file, "{}", self)?;
        file.flush()
    }
}

impl std::fmt::Display for VideoConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\"VideoConfig\"\n{{")?;
        writeln!(
            f,
            "\t\"setting.cl_gib_allow\"\t\t\"{}\"",
            self.cl_gib_allow as u8
        )?;
        writeln!(
            f,
            "\t\"setting.cl_particle_fallback_base\"\t\t\"{}\"",
            self.cl_particle_fallback_base
        )?;
        writeln!(
            f,
            "\t\"setting.cl_particle_fallback_multiplier\"\t\t\"{}\"",
            self.cl_particle_fallback_multiplier
        )?;
        writeln!(
            f,
            "\t\"setting.cl_ragdoll_maxcount\"\t\t\"{}\"",
            self.cl_ragdoll_maxcount
        )?;
        writeln!(
            f,
            "\t\"setting.cl_ragdoll_self_collision\"\t\t\"{}\"",
            self.cl_ragdoll_self_collision as u8
        )?;
        writeln!(
            f,
            "\t\"setting.mat_forceaniso\"\t\t\"{}\"",
            self.mat_forceaniso
        )?;
        writeln!(
            f,
            "\t\"setting.mat_mip_linear\"\t\t\"{}\"",
            self.mat_mip_linear
        )?;
        writeln!(
            f,
            "\t\"setting.stream_memory\"\t\t\"{}\"",
            self.stream_memory
        )?;
        writeln!(f, "\t\"setting.mat_picmip\"\t\t\"{}\"", self.mat_picmip)?;
        writeln!(
            f,
            "\t\"setting.particle_cpu_level\"\t\t\"{}\"",
            self.particle_cpu_level
        )?;
        writeln!(
            f,
            "\t\"setting.r_createmodeldecals\"\t\t\"{}\"",
            self.r_createmodeldecals as u8
        )?;
        writeln!(f, "\t\"setting.r_decals\"\t\t\"{}\"", self.r_decals)?;
        writeln!(
            f,
            "\t\"setting.r_lod_switch_scale\"\t\t\"{}\"",
            self.r_lod_switch_scale
        )?;
        writeln!(
            f,
            "\t\"setting.shadow_enable\"\t\t\"{}\"",
            self.shadow_enable as u8
        )?;
        writeln!(
            f,
            "\t\"setting.shadow_depth_dimen_min\"\t\t\"{}\"",
            self.shadow_depth_dimen_min
        )?;
        writeln!(
            f,
            "\t\"setting.shadow_depth_upres_factor_max\"\t\t\"{}\"",
            self.shadow_depth_upres_factor_max
        )?;
        writeln!(
            f,
            "\t\"setting.shadow_maxdynamic\"\t\t\"{}\"",
            self.shadow_maxdynamic
        )?;
        writeln!(
            f,
            "\t\"setting.ssao_enabled\"\t\t\"{}\"",
            self.ssao_enabled as u8
        )?;
        writeln!(
            f,
            "\t\"setting.ssao_downsample\"\t\t\"{}\"",
            self.ssao_downsample
        )?;
        writeln!(
            f,
            "\t\"setting.modeldecals_forceAllowed\"\t\t\"{}\"",
            self.modeldecals_forceAllowed as u8
        )?;
        writeln!(
            f,
            "\t\"setting.dvs_enable\"\t\t\"{}\"",
            self.dvs_enable as u8
        )?;
        writeln!(
            f,
            "\t\"setting.dvs_gpuframetime_min\"\t\t\"{}\"",
            self.dvs_gpuframetime_min
        )?;
        writeln!(
            f,
            "\t\"setting.dvs_gpuframetime_max\"\t\t\"{}\"",
            self.dvs_gpuframetime_max
        )?;
        writeln!(f, "\t\"setting.defaultres\"\t\t\"{}\"", self.defaultres)?;
        writeln!(
            f,
            "\t\"setting.defaultresheight\"\t\t\"{}\"",
            self.defaultresheight
        )?;
        writeln!(
            f,
            "\t\"setting.fullscreen\"\t\t\"{}\"",
            self.fullscreen as u8
        )?;
        writeln!(
            f,
            "\t\"setting.nowindowborder\"\t\t\"{}\"",
            self.nowindowborder as u8
        )?;
        writeln!(
            f,
            "\t\"setting.volumetric_lighting\"\t\t\"{}\"",
            self.volumetric_lighting as u8
        )?;
        writeln!(
            f,
            "\t\"setting.mat_vsync_mode\"\t\t\"{}\"",
            self.mat_vsync_mode as u8
        )?;
        writeln!(
            f,
            "\t\"setting.mat_backbuffer_count\"\t\t\"{}\"",
            self.mat_backbuffer_count
        )?;
        writeln!(
            f,
            "\t\"setting.mat_antialias_mode\"\t\t\"{}\"",
            self.mat_antialias_mode
        )?;
        writeln!(
            f,
            "\t\"setting.csm_enabled\"\t\t\"{}\"",
            self.csm_enabled as u8
        )?;
        writeln!(f, "\t\"setting.csm_coverage\"\t\t\"{}\"", self.csm_coverage)?;
        writeln!(
            f,
            "\t\"setting.csm_cascade_res\"\t\t\"{}\"",
            self.csm_cascade_res
        )?;
        writeln!(
            f,
            "\t\"setting.fadeDistScale\"\t\t\"{}\"",
            self.fadeDistScale
        )?;
        writeln!(
            f,
            "\t\"setting.dvs_supersample_enable\"\t\t\"{}\"",
            self.dvs_supersample_enable as u8
        )?;
        writeln!(f, "\t\"setting.gamma\"\t\t\"{}\"", self.gamma)?;
        writeln!(
            f,
            "\t\"setting.configversion\"\t\t\"{}\"",
            self.configversion
        )?;
        writeln!(f, "}}")?;
        Ok(())
    }
}
