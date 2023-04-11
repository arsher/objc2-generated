//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CARenderer")]
    pub struct CARenderer;

    #[cfg(feature = "CoreAnimation_CARenderer")]
    unsafe impl ClassType for CARenderer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CARenderer")]
unsafe impl NSObjectProtocol for CARenderer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CARenderer")]
    unsafe impl CARenderer {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated = "+rendererWithMTLTexture"]
        #[method_id(@__retain_semantics Other rendererWithCGLContext:options:)]
        pub unsafe fn rendererWithCGLContext_options(
            ctx: NonNull<c_void>,
            dict: Option<&NSDictionary>,
        ) -> Id<CARenderer>;

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer(&self) -> Option<Id<CALayer>>;

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[method(setLayer:)]
        pub fn setLayer(&self, layer: Option<&CALayer>);

        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub fn setBounds(&self, bounds: CGRect);

        #[method(updateBounds)]
        pub fn updateBounds(&self) -> CGRect;

        #[method(addUpdateRect:)]
        pub fn addUpdateRect(&self, r: CGRect);

        #[method(render)]
        pub fn render(&self);

        #[method(nextFrameTime)]
        pub fn nextFrameTime(&self) -> CFTimeInterval;

        #[method(endFrame)]
        pub fn endFrame(&self);
    }
);

extern_static!(kCARendererColorSpace: &'static NSString);

extern_static!(kCARendererMetalCommandQueue: &'static NSString);
