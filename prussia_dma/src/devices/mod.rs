#![warn(missing_docs)]

mod gif;
mod ipu;
mod sif;
mod spram;
mod traits;
mod vif;

pub use crate::devices::gif::Gif;
pub use crate::devices::ipu::{IpuFrom, IpuTo};
pub use crate::devices::sif::{Sif0, Sif1, Sif2};
pub use crate::devices::spram::{SpramFrom, SpramTo};
pub use crate::devices::traits::*;
pub use crate::devices::vif::{Vif0, Vif1};
