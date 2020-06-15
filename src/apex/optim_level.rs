#[derive(Debug, Copy, Clone, PartialEq, Eq, druid::Data)]
pub enum OptimizationLevel {
    Performance = 0,
    Safe = 1,
    ALGS = 2,
    Default = 10,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Default
    }
}

impl From<usize> for OptimizationLevel {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::Performance,
            1 => Self::Safe,
            2 => Self::ALGS,
            _ => Self::Default,
        }
    }
}
