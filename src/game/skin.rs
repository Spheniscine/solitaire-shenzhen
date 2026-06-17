use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum HonorSkin {
    #[default]
    Mythical,
    Abstract,
    Greek,
    Eastern,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum SuitSkin {
    #[default]
    Animals,
    Shapes,
    Western,
    Eastern,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorMode {
    #[default] Dark, Light
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Default)]
pub struct Skin {
    pub honors: HonorSkin,
    pub suits: SuitSkin,
}