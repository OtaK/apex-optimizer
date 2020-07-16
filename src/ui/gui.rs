use druid::{
    Data, Lens, LensExt, AppLauncher, Widget, WidgetExt, Color, WindowDesc,
    widget::{Button, Checkbox, Flex, Label, Container, RadioGroup, Switch, ProgressBar}
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Hinstance(winapi::shared::minwindef::DWORD);

impl Hinstance {
    fn is_err(&self) -> bool {
        self.0 <= 32
    }
}

impl std::fmt::Display for Hinstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "The operating system is out of memory or resources"),
            winapi::shared::winerror::ERROR_BAD_FORMAT => write!(f, "The .exe file is invalid (non-Win32 .exe or error in .exe image)."),
            winapi::um::shellapi::SE_ERR_ACCESSDENIED => write!(f, "The operating system denied access to the specified file."),
            winapi::um::shellapi::SE_ERR_ASSOCINCOMPLETE => write!(f, "The file name association is incomplete or invalid."),
            winapi::um::shellapi::SE_ERR_DDEBUSY => write!(f, "The DDE transaction could not be completed because other DDE transactions were being processed."),
            winapi::um::shellapi::SE_ERR_DDEFAIL => write!(f, "The DDE transaction failed."),
            winapi::um::shellapi::SE_ERR_DDETIMEOUT => write!(f, "The DDE transaction could not be completed because the request timed out."),
            winapi::um::shellapi::SE_ERR_DLLNOTFOUND => write!(f, "The specified DLL was not found."),
            winapi::um::shellapi::SE_ERR_FNF => write!(f, "The specified file was not found."),
            winapi::um::shellapi::SE_ERR_NOASSOC => write!(f, "There is no application associated with the given file name extension. This error will also be returned if you attempt to print a file that is not printable."),
            winapi::um::shellapi::SE_ERR_OOM => write!(f, "There was not enough memory to complete the operation."),
            winapi::um::shellapi::SE_ERR_PNF => write!(f, "The specified path was not found."),
            winapi::um::shellapi::SE_ERR_SHARE => write!(f, "A sharing violation occurred."),
            _ => Ok(())
        }
    }
}

impl From<winapi::shared::minwindef::DWORD> for Hinstance {
    fn from(v: winapi::shared::minwindef::DWORD) -> Self {
        Self(v)
    }
}

#[derive(Debug, Data, Lens, Clone, PartialEq, Default)]
struct OptimizerOptions {
    progress: f64,
    current_error: Option<String>,
    pretend: bool,
    no_backup: bool,
    registry_fixes: crate::registry::WindowsFixes,
    enable_videoconfig: bool,
    apex_videoconfig_level: crate::apex::OptimizationLevel,
    enable_autoexec: bool,
    apex_autoexec_level: crate::apex::OptimizationLevel,
}

impl OptimizerOptions {
    fn apply(&mut self) -> std::io::Result<()>{
        self.progress = 0.0;
        if self.pretend {
            self.progress = 1.;
            return Ok(());
        }

        let runas = std::ffi::CString::new("runas").unwrap().as_ptr() as winapi::shared::ntdef::LPCSTR;
        let target_exe = std::ffi::CString::new(std::env::current_exe()?.as_os_str().to_str().unwrap()).unwrap().as_ptr() as winapi::shared::ntdef::LPCSTR;

        // Apply registry tweaks
        let result: Hinstance = unsafe {
            winapi::um::shellapi::ShellExecuteA(
                winapi::um::winuser::GetActiveWindow(),
                runas,
                target_exe,
                std::ffi::CString::new(format!("registry {}", self.registry_fixes.as_cli_args())).unwrap().as_ptr() as winapi::shared::ntdef::LPCSTR,
                winapi::shared::ntdef::NULL as *const i8,
                winapi::um::winuser::SW_HIDE,
            ) as winapi::shared::minwindef::DWORD
        }.into();

        if result.is_err() {
            self.current_error = Some(format!("{}", result));
            return Err(std::io::ErrorKind::Other.into());
        }

        // Apply Apex tweaks
        if self.enable_videoconfig || self.enable_autoexec {
            self.progress = 0.5;

            let apex_args = {
                let mut tmp = vec![];
                if self.enable_autoexec {
                    tmp.push(format!("--autoexec {}", self.apex_autoexec_level));
                }
                if self.enable_videoconfig {
                    tmp.push(format!("--videoconfig {}", self.apex_videoconfig_level));
                }
                tmp
            };

            let result: Hinstance = unsafe {
                winapi::um::shellapi::ShellExecuteA(
                    winapi::um::winuser::GetActiveWindow(),
                    runas,
                    target_exe,
                    std::ffi::CString::new(format!("apex {}", apex_args.join(" "))).unwrap().as_ptr() as winapi::shared::ntdef::LPCSTR,
                    winapi::shared::ntdef::NULL as *const i8,
                    winapi::um::winuser::SW_HIDE,
                ) as winapi::shared::minwindef::DWORD
            }.into();

            if result.is_err() {
                self.current_error = Some(format!("{}", result));
                return Err(std::io::ErrorKind::Other.into());
            }
        }

        self.progress = 1.0;
        Ok(())
    }
}

pub fn start_gui() -> std::io::Result<()> {
    let window = WindowDesc::new(ui_builder)
        .title(format!("Apex Optimizer - v{}", env!("CARGO_PKG_VERSION")))
        .window_size((700., 320.))
        .resizable(false);

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

    // TODO: Add videomode selection scrolling RadioGroup

    root.add_child(Button::new("Apply").fix_size(100., 30.).on_click(|_, state: &mut OptimizerOptions, _| {
        info!("State: {:?}", state);

        let _ = state.apply();
    }).padding(20.));

    // TODO: Add button for `pretend` and `backup` options

    root.add_child(ProgressBar::new().expand_width().padding((30., 5.)).lens(OptimizerOptions::progress));

    root.center()
}
