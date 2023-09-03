use crate::commands::feature::build_feature;

pub enum Commands {
    Feature(Box<str>),
}

impl Drop for Commands {
    fn drop(&mut self) {
        match self {
            Self::Feature(feat) => build_feature(feat),
        }
    }
}
