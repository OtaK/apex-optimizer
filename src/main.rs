mod fragments;

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
enum OptimizationLevel {
    Aggressive = 0,
    Conservative = 1,
    Safe = 2,
    Default = 3
}

impl From<usize> for OptimizationLevel {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::Aggressive,
            1 => Self::Conservative,
            2 => Self::Safe,
            _ => Self::Default,
        }
    }
}

fn main() {
    let theme = dialoguer::theme::ColorfulTheme::default();
    let mut prompt = dialoguer::Select::with_theme(&theme);
    prompt.with_prompt("Please select a level of optimization: ");
    prompt.items(&[
        "Aggressive - autoexec + ugly af videoconfig",
        "Conservative - ugly af videoconfig",
        "Safe - safe videoconfig with a few optims here and there",
        "Default - Deletes the custom videoconfig"
    ]);

    if let Ok(level) = prompt.interact().map(OptimizationLevel::from) {
        println!("level selected: {:?}", level);
    }
}
