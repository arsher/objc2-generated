//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowscenestandardplacement?language=objc)
    #[unsafe(super(UIWindowScenePlacement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowScenePlacement")]
    pub struct UIWindowSceneStandardPlacement;
);

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSCopying for UIWindowSceneStandardPlacement {}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl CopyingHelper for UIWindowSceneStandardPlacement {
    type Result = Self;
}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSObjectProtocol for UIWindowSceneStandardPlacement {}

extern_methods!(
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowSceneStandardPlacement {
        #[method_id(@__retain_semantics Other standardPlacement)]
        pub unsafe fn standardPlacement() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowScenePlacement`
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowSceneStandardPlacement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
