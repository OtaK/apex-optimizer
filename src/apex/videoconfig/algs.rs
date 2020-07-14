use super::VideoConfig;

impl VideoConfig {
    pub(crate) fn defaults_algs() -> Self {
        let mut ret = Self::default();
        ret.csm_enabled = false;
        ret
    }
}
