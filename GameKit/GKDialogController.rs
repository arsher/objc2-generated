//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkviewcontroller?language=objc)
    pub unsafe trait GKViewController {}

    unsafe impl ProtocolType for dyn GKViewController {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkdialogcontroller?language=objc)
    #[unsafe(super(NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct GKDialogController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKDialogController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKDialogController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Retained<NSWindow>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[method(presentViewController:)]
        pub unsafe fn presentViewController(&self, view_controller: &NSViewController) -> bool;

        #[method(dismiss:)]
        pub unsafe fn dismiss(&self, sender: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// SharedDialogController
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics Other sharedDialogController)]
        pub unsafe fn sharedDialogController(mtm: MainThreadMarker)
            -> Retained<GKDialogController>;
    }
);
