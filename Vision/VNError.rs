//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnerrordomain?language=objc)
    pub static VNErrorDomain: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNErrorCode(pub NSInteger);
impl VNErrorCode {
    pub const VNErrorTuriCoreErrorCode: Self = Self(-1);
    pub const VNErrorOK: Self = Self(0);
    pub const VNErrorRequestCancelled: Self = Self(1);
    pub const VNErrorInvalidFormat: Self = Self(2);
    pub const VNErrorOperationFailed: Self = Self(3);
    pub const VNErrorOutOfBoundsError: Self = Self(4);
    pub const VNErrorInvalidOption: Self = Self(5);
    pub const VNErrorIOError: Self = Self(6);
    pub const VNErrorMissingOption: Self = Self(7);
    pub const VNErrorNotImplemented: Self = Self(8);
    pub const VNErrorInternalError: Self = Self(9);
    pub const VNErrorOutOfMemory: Self = Self(10);
    pub const VNErrorUnknownError: Self = Self(11);
    pub const VNErrorInvalidOperation: Self = Self(12);
    pub const VNErrorInvalidImage: Self = Self(13);
    pub const VNErrorInvalidArgument: Self = Self(14);
    pub const VNErrorInvalidModel: Self = Self(15);
    pub const VNErrorUnsupportedRevision: Self = Self(16);
    pub const VNErrorDataUnavailable: Self = Self(17);
    pub const VNErrorTimeStampNotFound: Self = Self(18);
    pub const VNErrorUnsupportedRequest: Self = Self(19);
    pub const VNErrorTimeout: Self = Self(20);
    pub const VNErrorUnsupportedComputeStage: Self = Self(21);
    pub const VNErrorUnsupportedComputeDevice: Self = Self(22);
}

unsafe impl Encode for VNErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
