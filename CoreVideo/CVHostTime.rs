//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;

use crate::*;

extern "C-unwind" {
    /// Retrieve the current value of the host time base.
    ///
    /// On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
    /// may be used interchangeably.
    ///
    /// Returns: The current host time.
    pub fn CVGetCurrentHostTime() -> u64;
}

extern "C-unwind" {
    /// Retrieve the frequency of the host time base.
    ///
    /// On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
    /// may be used interchangeably.
    ///
    /// Returns: The current host frequency.
    pub fn CVGetHostClockFrequency() -> c_double;
}

extern "C-unwind" {
    /// Retrieve the smallest possible increment in the host time base.
    ///
    /// Returns: The smallest valid increment in the host time base.
    pub fn CVGetHostClockMinimumTimeDelta() -> u32;
}
