pub const SHADOWS_AE: &str = r#"
// Shadows
csm_enabled 0
csm_enableunlock 1
csm_coverage 0
csm_enabled 0
csm_world_shadows 0
csm_cascade_res 0
shadow_enable 0
shadow_capable 0
shadow_maxdynamic 0
shadow_max_dynamic_lobby 0
shadow_depth_dimen_min 0
shadow_depth_upres_factor_max 0
static_shadow 1
static_shadow_res 0

"#;

pub const OPTIMS_AE: &str = r#"
// Optims, dunno if they work
r_fastzreject 1
cl_forcepreload 1
mat_queue_mode 2

// SIMD bones, dunno if it works
cl_use_simd_bones 1
cl_simdbones_slerp 1

"#;
