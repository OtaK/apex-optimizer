pub const GRAPHICS_AE: &str = r#"
// Graphics
mat_picmip 4
stream_memory "0"
r_lod_switch_scale 0.45
fog_enableskybox 0
mat_screen_blur_enabled 0
mat_forceaniso 0
ssao_enabled 0
r_decals 0
particle_cpu_level 0
cl_ragdoll_maxcount 0
cl_particle_fallback_multiplier 0
cl_particle_fallback_base 0
noise_filter_scale 0
mat_bloom_scalefactor_scalar 0
r_createmodeldecals 0

"#;

pub const GRAPHICS_MISC_AE: &str = r#"
// Graphics Misc
mat_disable_bloom 1
mat_mip_linear 0
cl_gib_allow 0
sssss_enable 0
staticProp_budget 1
func_break_max_pieces 1
cheap_captions_fadetime 0
chroma_enable 0
cl_minimal_rtt_shadows 1
mat_compressedtextures 1
cl_disable_ragdolls 1
cl_ragdoll_collide 0
cl_ragdoll_force_fade_time 0
cl_ragdoll_force_fade_time_local_view_player 0
cl_ragdoll_force_fade_time_on_moving_geo 0
mat_maxframelatency 0

"#;
