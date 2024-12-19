//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerregion?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerRegion;
);

unsafe impl NSCopying for UIPointerRegion {}

unsafe impl CopyingHelper for UIPointerRegion {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPointerRegion {}

extern_methods!(
    unsafe impl UIPointerRegion {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(rect)]
        pub unsafe fn rect(&self) -> CGRect;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<ProtocolObject<dyn NSObjectProtocol>>>;

        #[cfg(feature = "UIGeometry")]
        #[method(latchingAxes)]
        pub unsafe fn latchingAxes(&self) -> UIAxis;

        #[cfg(feature = "UIGeometry")]
        #[method(setLatchingAxes:)]
        pub unsafe fn setLatchingAxes(&self, latching_axes: UIAxis);

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other regionWithRect:identifier:)]
        pub unsafe fn regionWithRect_identifier(
            rect: CGRect,
            identifier: Option<&ProtocolObject<dyn NSObjectProtocol>>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
