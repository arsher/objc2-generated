//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "UIGraphicsRenderer", feature = "block2"))]
pub type UIGraphicsDrawingActions = *mut block2::Block<dyn Fn(NonNull<UIGraphicsRendererContext>)>;

extern_methods!(
    /// UIGraphicsRendererProtected
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsRenderer {
        #[method(rendererContextClass)]
        pub unsafe fn rendererContextClass() -> &'static AnyClass;

        #[cfg(feature = "block2")]
        #[method(runDrawingActions:completionActions:error:_)]
        pub unsafe fn runDrawingActions_completionActions_error(
            &self,
            drawing_actions: UIGraphicsDrawingActions,
            completion_actions: UIGraphicsDrawingActions,
        ) -> Result<(), Id<NSError>>;
    }
);
