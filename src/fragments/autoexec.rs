pub static AUTOEXEC_OPTIMIZED: &'static str = r#"
cl_showfps 4
m_rawinput 1
rate 128000
cl_interp 0
cl_interp_ratio 1
cl_cmdrate "120" //default 60
cl_updaterate_mp "60" //default 20
match_updateRate "60"
cl_updatevisibility "1"
cl_timeout "30"
net_compresspackets 0
cl_smooth 0
cl_smoothtime 0.01
cl_pred_optimize 1
cl_wpn_sway_interp "0"
cl_lagcompensation "1"
net_compresspackets_minsize 128
net_maxcleartime 0.020346
host_limitlocal 0
////////////////////////////////////////////////////
//////////////supercrouch audio
miles_channels "2"
miles_occlusion "0"
miles_occlusion_force "0"
miles_occlusion_partial "0"
miles_max_sounds_per_server_frame "400" //default 50
cl_footstep_event_max_dist "5000" //default 2500

mat_compressedtextures 1
cl_ragdoll_collide 0
r_rimlight 0
r_modeldecal_maxtotal 0
r_decals 0
r_jiggle_bones 0
r_updaterefracttexture 0
r_updaterefracttexture_allowmultiple 0
r_WaterDrawReflection 0
r_WaterDrawRefraction 0
r_forcecheapwater 1 // water bad
r_dopixelvisibility 0
r_drawbatchdecals 0
cl_ragdoll_force_fade_time 0
cl_ragdoll_force_fade_time_local_view_player 0
cl_ragdoll_force_fade_time_on_moving_geo 0
lightmap_realtimelight 0
lightmap_realtimeshadows 0
lightmap_ambient 0
mat_colcorrection_disableentities 0
mat_diffuse 0
mat_detail_tex 0
mat_phong 0
mat_filtertextures 0
mat_filterlightmaps 1
mat_screen_blur_enabled 0
mat_local_contrast_scale_override 0
mat_max_worldmesh_vertices "0"
mat_maxframelatency "0"
csm_enableunlock 1
csm_coverage 0
csm_enabled 0
csm_world_shadows 0
csm_cascade_res 0
//shadow options need csm_enabled 1
shadow_capable 0
shadow_enable 0
viewmodel_selfshadow 0
static_shadow "1"
static_shadow_res "0"
cl_disable_ragdolls 1
cl_use_simd_bones 1 // not sure they use this
cl_simdbones_slerp 1
cl_headbob_amp 0
cl_headbob_freq 0
r_fastzreject 1
"#;
