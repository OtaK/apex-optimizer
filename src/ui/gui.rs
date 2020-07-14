use druid::{
    Data, Lens, LensExt, AppLauncher, Widget, WidgetExt, Color, WindowDesc,
    widget::{Button, Checkbox, Flex, Label, Container, RadioGroup, Switch, Align, ProgressBar}, UnitPoint
};

#[derive(Debug, Data, Lens, Clone, Copy, PartialEq, Default)]
struct OptimizerOptions {
    progress: f64,
    pretend: bool,
    no_backup: bool,
    registry_fixes: crate::registry::WindowsFixes,
    enable_videoconfig: bool,
    apex_videoconfig_level: crate::apex::OptimizationLevel,
    enable_autoexec: bool,
    apex_autoexec_level: crate::apex::OptimizationLevel,
}

impl OptimizerOptions {
    fn apply(&mut self) {
        self.progress = 0.5;

        // TODO: Apply tweaks and update progress
    }
}

pub fn start_gui() -> std::io::Result<()> {
    let window = WindowDesc::new(ui_builder)
        .title(format!("Apex Optimizer - v{}", env!("CARGO_PKG_VERSION")))
        .window_size((700., 320.))
        .resizable(true);

    let data = OptimizerOptions::default();
    AppLauncher::with_window(window)
        .launch(data)
        .expect("Launch failed!");

    Ok(())
}

fn ui_builder() -> impl Widget<OptimizerOptions> {
    let mut root = Flex::column();

    let vc_radiogroup = RadioGroup::new(vec![
        ("Performance", crate::apex::OptimizationLevel::Performance),
        ("Safe", crate::apex::OptimizationLevel::Safe),
        (crate::ALGS_STR_SHORT, crate::apex::OptimizationLevel::ALGS),
        ("Default", crate::apex::OptimizationLevel::Default),
    ]).lens(OptimizerOptions::apex_videoconfig_level);

    let ae_radiogroup = RadioGroup::new(vec![
        ("Performance", crate::apex::OptimizationLevel::Performance),
        ("Safe", crate::apex::OptimizationLevel::Safe),
        (crate::ALGS_STR_SHORT, crate::apex::OptimizationLevel::ALGS),
        ("Default", crate::apex::OptimizationLevel::Default),
    ]).lens(OptimizerOptions::apex_autoexec_level);

    let apex_row = Flex::row()
        .with_flex_child(
            Flex::column()
                .with_child(Flex::row()
                    .with_child(Label::new("Apex Videoconfig"))
                    .with_child(Switch::new().lens(OptimizerOptions::enable_videoconfig))
                )
                .with_spacer(10.)
                .with_child(Container::new(vc_radiogroup).border(Color::BLACK, 1.).rounded(10.)),
            1.
        )
        .with_spacer(30.)
        .with_flex_child(
            Flex::column()
                .with_child(Flex::row()
                    .with_child(Label::new("Apex Autoexec"))
                    .with_child(Switch::new().lens(OptimizerOptions::enable_autoexec))
                )
                .with_spacer(10.)
                .with_child(Container::new(ae_radiogroup).border(Color::BLACK, 1.).rounded(10.)),
            1.
        )
        .with_spacer(30.)
        .with_flex_child(Flex::column()
            .with_child(Label::new("Registry tweaks"))
            .with_spacer(10.)
            .with_child(
                Checkbox::new("Exclusive FullScreen/GameDVR")
                .align_left()
                .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::FSE))
            )
            .with_spacer(5.)
            .with_child(
                Checkbox::new("MouseFix")
                .align_left()
                .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::MouseFix))
            )
            .with_spacer(5.)
            .with_child(
                Checkbox::new("TCP / Nagling tweaks")
                .align_left()
                .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::TCP))
            )
            .with_spacer(5.)
            .with_child(
                Checkbox::new("Gaming Tweaks")
                .align_left()
                .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::Gaming))
            )
            .with_spacer(5.)
            .with_child(
                Checkbox::new("Fixed Timer & HPET Off")
                .align_left()
                .lens(OptimizerOptions::registry_fixes.index(crate::registry::WindowsFix::Timer))
            ),
            1.
        );

    root.add_flex_child(apex_row.padding(20.), 1.);

    root.add_child(Button::new("Apply").on_click(|_, state: &mut OptimizerOptions, _| {
        info!("State: {:?}", state);

        state.apply();
    }).padding(20.));

    root.add_child(ProgressBar::new().lens(OptimizerOptions::progress));

    root.center()
}
