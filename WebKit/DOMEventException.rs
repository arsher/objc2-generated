//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domeventexception?language=objc)
    pub static DOMEventException: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/domeventexceptioncode?language=objc)
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DOMEventExceptionCode(pub c_uint);
impl DOMEventExceptionCode {
    #[deprecated]
    pub const DOM_UNSPECIFIED_EVENT_TYPE_ERR: Self = Self(0);
}

unsafe impl Encode for DOMEventExceptionCode {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for DOMEventExceptionCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
