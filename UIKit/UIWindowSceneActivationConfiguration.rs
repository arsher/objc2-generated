//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowsceneactivationconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIWindowSceneActivationConfiguration;
);

unsafe impl NSObjectProtocol for UIWindowSceneActivationConfiguration {}

extern_methods!(
    unsafe impl UIWindowSceneActivationConfiguration {
        #[method_id(@__retain_semantics Other userActivity)]
        pub unsafe fn userActivity(&self) -> Retained<NSUserActivity>;

        #[cfg(all(
            feature = "UISceneOptions",
            feature = "UIWindowSceneActivationRequestOptions"
        ))]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UIWindowSceneActivationRequestOptions>>;

        #[cfg(all(
            feature = "UISceneOptions",
            feature = "UIWindowSceneActivationRequestOptions"
        ))]
        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: Option<&UIWindowSceneActivationRequestOptions>);

        #[cfg(feature = "UITargetedPreview")]
        #[method_id(@__retain_semantics Other preview)]
        pub unsafe fn preview(&self, mtm: MainThreadMarker) -> Option<Retained<UITargetedPreview>>;

        #[cfg(feature = "UITargetedPreview")]
        #[method(setPreview:)]
        pub unsafe fn setPreview(&self, preview: Option<&UITargetedPreview>);

        #[method_id(@__retain_semantics Init initWithUserActivity:)]
        pub unsafe fn initWithUserActivity(
            this: Allocated<Self>,
            user_activity: &NSUserActivity,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
