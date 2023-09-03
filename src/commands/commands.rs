use crate::commands::feature::build_feature;

pub enum Commands {
    Feature(Box<str>),
}

impl Commands {
    pub(crate) fn run(&self) {
        match self {
            Self::Feature(feat) => build_feature(feat),
        }
    }
}

impl Drop for Commands {
    fn drop(&mut self) {
        self.run();
    }
}
