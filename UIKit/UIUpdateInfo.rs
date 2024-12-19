//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiupdateinfo?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIUpdateInfo;
);

unsafe impl NSObjectProtocol for UIUpdateInfo {}

extern_methods!(
    unsafe impl UIUpdateInfo {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[method_id(@__retain_semantics Other currentUpdateInfoForWindowScene:)]
        pub unsafe fn currentUpdateInfoForWindowScene(
            window_scene: &UIWindowScene,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other currentUpdateInfoForView:)]
        pub unsafe fn currentUpdateInfoForView(view: &UIView) -> Option<Retained<Self>>;

        #[method(modelTime)]
        pub unsafe fn modelTime(&self) -> NSTimeInterval;

        #[method(completionDeadlineTime)]
        pub unsafe fn completionDeadlineTime(&self) -> NSTimeInterval;

        #[method(estimatedPresentationTime)]
        pub unsafe fn estimatedPresentationTime(&self) -> NSTimeInterval;

        #[method(isImmediatePresentationExpected)]
        pub unsafe fn isImmediatePresentationExpected(&self) -> bool;

        #[method(isLowLatencyEventDispatchConfirmed)]
        pub unsafe fn isLowLatencyEventDispatchConfirmed(&self) -> bool;

        #[method(isPerformingLowLatencyPhases)]
        pub unsafe fn isPerformingLowLatencyPhases(&self) -> bool;
    }
);
