//! # Art
//!
//! A library for modelling artistic concepts.

pub use self::kinds::*; // <--- This is known as re-exporting
pub use self::utils::*;

pub mod kinds {
    /// The primary colors according to the RYB model.
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB model.
    #[derive(PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::{PrimaryColor, SecondaryColor};

    /// Combines two paimary colors in equal amounts to create a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow {
            SecondaryColor::Orange
        } else if c1 == PrimaryColor::Red && c2 == PrimaryColor::Blue {
            SecondaryColor::Purple
        } else if c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Red {
            SecondaryColor::Orange
        } else if c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Blue {
            SecondaryColor::Green
        } else if c1 == PrimaryColor::Blue && c2 == PrimaryColor::Red {
            SecondaryColor::Purple
        } else {
            SecondaryColor::Green
        }
    }
}
