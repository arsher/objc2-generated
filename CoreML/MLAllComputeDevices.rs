//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "MLComputeDeviceProtocol")]
#[inline]
pub unsafe extern "C-unwind" fn MLAllComputeDevices(
) -> Retained<NSArray<ProtocolObject<dyn MLComputeDeviceProtocol>>> {
    extern "C-unwind" {
        fn MLAllComputeDevices() -> NonNull<NSArray<ProtocolObject<dyn MLComputeDeviceProtocol>>>;
    }
    let ret = unsafe { MLAllComputeDevices() };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}
