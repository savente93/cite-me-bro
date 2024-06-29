#![allow(dead_code)]
pub mod ops;
pub mod parsing;
pub mod styles;
pub mod utils;

pub use crate::ops::bibligraphy::Bibliography;
pub use styles::ReferenceStyle;

pub static VERSION: &str = "1.0";
