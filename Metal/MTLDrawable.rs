//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtldrawablepresentedhandler?language=objc)
#[cfg(feature = "block2")]
pub type MTLDrawablePresentedHandler =
    *mut block2::Block<dyn Fn(NonNull<ProtocolObject<dyn MTLDrawable>>)>;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtldrawable?language=objc)
    pub unsafe trait MTLDrawable: NSObjectProtocol {
        #[method(present)]
        fn present(&self);

        #[method(presentAtTime:)]
        unsafe fn presentAtTime(&self, presentation_time: CFTimeInterval);

        #[method(presentAfterMinimumDuration:)]
        unsafe fn presentAfterMinimumDuration(&self, duration: CFTimeInterval);

        #[cfg(feature = "block2")]
        #[method(addPresentedHandler:)]
        unsafe fn addPresentedHandler(&self, block: MTLDrawablePresentedHandler);

        #[method(presentedTime)]
        unsafe fn presentedTime(&self) -> CFTimeInterval;

        #[method(drawableID)]
        fn drawableID(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLDrawable {}
);
