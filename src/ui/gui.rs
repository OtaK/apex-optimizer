use druid::widget::{Button, Checkbox, Flex, Label, Container, RadioGroup};
use druid::{Data, Lens, LensExt, AppLauncher, LocalizedString, Widget, LensWrap, WidgetExt, Color, WindowDesc};

#[derive(Debug, Data, Lens, Clone, Copy, Eq, PartialEq, Default)]
struct OptimizerOptions {
    pretend: bool,
    no_backup: bool,
    registry_fixes: crate::registry::WindowsFixes,
    apex_videoconfig_level: crate::apex::OptimizationLevel,
    apex_autoexec_level: crate::apex::OptimizationLevel,
}

pub fn start_gui() -> std::io::Result<()> {
    let window = WindowDesc::new(ui_builder)
        .title(format!("Apex Optimizer - v{}", env!("CARGO_PKG_VERSION")))
        .window_size((800., 400.))
        .resizable(true);

    let data = OptimizerOptions::default();
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(data)
        .expect("Launch failed!");

    Ok(())
}


fn ui_builder() -> impl Widget<OptimizerOptions> {
    let mut col = Flex::column();

    let apex_row = Flex::row().with_flex_child(Flex::column()
        .with_flex_child(
            Flex::row()
                .with_child(Label::new("Apex Videoconfig"))
                .with_child(Container::new(RadioGroup::new(vec![
                    ("Performance - Game looks like trash. Might be unstable and/or reduce visibility, but FPS is maxed out.", crate::apex::OptimizationLevel::Performance),
                    ("Safe - Crash-safe videoconfig with a few optims here and there", crate::apex::OptimizationLevel::Safe),
                    (crate::ALGS_STR, crate::apex::OptimizationLevel::ALGS),
                    ("Default - Deletes the custom videoconfig", crate::apex::OptimizationLevel::Default),
                ]).lens(OptimizerOptions::apex_videoconfig_level)).border(Color::BLACK, 2.).rounded(10.)),
            1.
        )
        .with_spacer(5.)
        .with_flex_child(
            Flex::row()
                .with_child(Label::new("Apex Autoexec"))
                .with_child(Container::new(RadioGroup::new(vec![
                    ("Performance - Good FPS gains. Might be unstable on some systems. Do not use in competitive.", crate::apex::OptimizationLevel::Performance),
                    ("Safe - Crash-safe values with small FPS gains. Probably banned in competitive as well.", crate::apex::OptimizationLevel::Safe),
                    (crate::ALGS_STR, crate::apex::OptimizationLevel::ALGS),
                    ("Default - Deletes the custom autoexec", crate::apex::OptimizationLevel::Default),
                ]).lens(OptimizerOptions::apex_autoexec_level)).border(Color::BLACK, 2.).rounded(10.)),
            1.
        ),
        1.
    );


    col.add_flex_child(apex_row, 1.);

    let registry_row = Flex::row().with_flex_child(Flex::column()
        .with_child(
            Checkbox::new("Exclusive FullScreen/GameDVR - Tells Windows to respect the Exclusive fullscreen setting. Reduces input lag.")
            .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::FSE))
        )
        .with_child(
            Checkbox::new("MouseFix - Registry tweak to tell windows to stop altering your mouse inputs. Requires 6/11 mouse speed setting in the Control Panel")
            .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::MouseFix))
        )
        .with_child(
            Checkbox::new("TCP / Nagling tweaks - Disable Nagle's algorithm and optimizes TCP handling for modern/gaming workloads")
            .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::TCP))
        )
        .with_child(
            Checkbox::new("Gaming Tweaks - Improves system responsiveness when using games. Might reduce input lag/latency when gaming & improve performance")
            .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::Gaming))
        ),
        1.
    );

    col.add_flex_child(registry_row, 1.);
    col.center()
}
