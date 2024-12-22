//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Keys to a dictionary that holds useful information about a model.
/// All are optional with the aim of being helpful to a developer or user
/// for descriptive purposes.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelmetadatakey?language=objc)
// NS_TYPED_ENUM
pub type MLModelMetadataKey = NSString;

extern "C" {
    /// A short description of what the model does and/or its purpose
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodeldescriptionkey?language=objc)
    pub static MLModelDescriptionKey: Option<&'static MLModelMetadataKey>;
}

extern "C" {
    /// A version number encoded as a string
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelversionstringkey?language=objc)
    pub static MLModelVersionStringKey: Option<&'static MLModelMetadataKey>;
}

extern "C" {
    /// The author of this model
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelauthorkey?language=objc)
    pub static MLModelAuthorKey: Option<&'static MLModelMetadataKey>;
}

extern "C" {
    /// License information for the model
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodellicensekey?language=objc)
    pub static MLModelLicenseKey: Option<&'static MLModelMetadataKey>;
}

extern "C" {
    /// Any additional pertinent information specified by the model creator
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelcreatordefinedkey?language=objc)
    pub static MLModelCreatorDefinedKey: Option<&'static MLModelMetadataKey>;
}
