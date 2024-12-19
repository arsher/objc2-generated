//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicsdrawingactions?language=objc)
#[cfg(all(feature = "UIGraphicsRenderer", feature = "block2"))]
pub type UIGraphicsDrawingActions = *mut block2::Block<dyn Fn(NonNull<UIGraphicsRendererContext>)>;

extern_methods!(
    /// UIGraphicsRendererProtected
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsRenderer {
        #[method(rendererContextClass)]
        pub unsafe fn rendererContextClass() -> &'static AnyClass;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(prepareCGContext:withRendererContext:)]
        pub unsafe fn prepareCGContext_withRendererContext(
            context: CGContextRef,
            renderer_context: &UIGraphicsRendererContext,
        );

        #[cfg(feature = "block2")]
        #[method(runDrawingActions:completionActions:error:_)]
        pub unsafe fn runDrawingActions_completionActions_error(
            &self,
            drawing_actions: UIGraphicsDrawingActions,
            completion_actions: UIGraphicsDrawingActions,
        ) -> Result<(), Retained<NSError>>;
    }
);
