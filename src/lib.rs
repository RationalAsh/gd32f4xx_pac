//! Multi device hardware abstraction on top of the peripheral access API
//! for the STMicro STM32F4 series microcontrollers.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
#![no_std]
#![allow(non_camel_case_types)]

use enumflags2::{BitFlag, BitFlags};

pub use embedded_hal as hal;
pub use embedded_hal_02 as hal_02;

pub use nb;
pub use nb::block;
