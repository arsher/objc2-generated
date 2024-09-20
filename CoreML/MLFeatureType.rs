//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLFeatureType(pub NSInteger);
impl MLFeatureType {
    #[doc(alias = "MLFeatureTypeInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MLFeatureTypeInt64")]
    pub const Int64: Self = Self(1);
    #[doc(alias = "MLFeatureTypeDouble")]
    pub const Double: Self = Self(2);
    #[doc(alias = "MLFeatureTypeString")]
    pub const String: Self = Self(3);
    #[doc(alias = "MLFeatureTypeImage")]
    pub const Image: Self = Self(4);
    #[doc(alias = "MLFeatureTypeMultiArray")]
    pub const MultiArray: Self = Self(5);
    #[doc(alias = "MLFeatureTypeDictionary")]
    pub const Dictionary: Self = Self(6);
    #[doc(alias = "MLFeatureTypeSequence")]
    pub const Sequence: Self = Self(7);
    #[doc(alias = "MLFeatureTypeState")]
    pub const State: Self = Self(8);
}

unsafe impl Encode for MLFeatureType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MLFeatureType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
