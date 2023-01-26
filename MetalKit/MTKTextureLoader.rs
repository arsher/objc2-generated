//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Metal::*;
use crate::MetalKit::*;

typed_enum!(
    pub type MTKTextureLoaderError = NSString;
);

extern_static!(MTKTextureLoaderErrorDomain: &'static MTKTextureLoaderError);

extern_static!(MTKTextureLoaderErrorKey: &'static MTKTextureLoaderError);

typed_enum!(
    pub type MTKTextureLoaderOption = NSString;
);

extern_static!(MTKTextureLoaderOptionAllocateMipmaps: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderOptionGenerateMipmaps: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderOptionSRGB: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderOptionTextureUsage: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderOptionTextureCPUCacheMode: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderOptionTextureStorageMode: &'static MTKTextureLoaderOption);

typed_enum!(
    pub type MTKTextureLoaderCubeLayout = NSString;
);

extern_static!(MTKTextureLoaderOptionCubeLayout: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderCubeLayoutVertical: &'static MTKTextureLoaderCubeLayout);

typed_enum!(
    pub type MTKTextureLoaderOrigin = NSString;
);

extern_static!(MTKTextureLoaderOptionOrigin: &'static MTKTextureLoaderOption);

extern_static!(MTKTextureLoaderOriginTopLeft: &'static MTKTextureLoaderOrigin);

extern_static!(MTKTextureLoaderOriginBottomLeft: &'static MTKTextureLoaderOrigin);

extern_static!(MTKTextureLoaderOriginFlippedVertically: &'static MTKTextureLoaderOrigin);

pub type MTKTextureLoaderCallback =
    *mut Block<(*mut ProtocolObject<dyn MTLTexture>, *mut NSError), ()>;

pub type MTKTextureLoaderArrayCallback = *mut Block<
    (
        NonNull<NSArray<ProtocolObject<dyn MTLTexture>>>,
        *mut NSError,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalKit_MTKTextureLoader")]
    pub struct MTKTextureLoader;

    #[cfg(feature = "MetalKit_MTKTextureLoader")]
    unsafe impl ClassType for MTKTextureLoader {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetalKit_MTKTextureLoader")]
unsafe impl NSObjectProtocol for MTKTextureLoader {}

extern_methods!(
    #[cfg(feature = "MetalKit_MTKTextureLoader")]
    unsafe impl MTKTextureLoader {
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Option<Allocated<Self>>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSURL"))]
        #[method(newTextureWithContentsOfURL:options:completionHandler:)]
        pub unsafe fn newTextureWithContentsOfURL_options_completionHandler(
            &self,
            url: &NSURL,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderCallback,
        );

        #[cfg(all(
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(newTextureWithName:scaleFactor:bundle:options:completionHandler:)]
        pub unsafe fn newTextureWithName_scaleFactor_bundle_options_completionHandler(
            &self,
            name: &NSString,
            scale_factor: CGFloat,
            bundle: Option<&NSBundle>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderCallback,
        );

        #[cfg(all(
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(newTextureWithName:scaleFactor:displayGamut:bundle:options:completionHandler:)]
        pub unsafe fn newTextureWithName_scaleFactor_displayGamut_bundle_options_completionHandler(
            &self,
            name: &NSString,
            scale_factor: CGFloat,
            display_gamut: NSDisplayGamut,
            bundle: Option<&NSBundle>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderCallback,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSURL"
        ))]
        #[method(newTexturesWithContentsOfURLs:options:completionHandler:)]
        pub unsafe fn newTexturesWithContentsOfURLs_options_completionHandler(
            &self,
            ur_ls: &NSArray<NSURL>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderArrayCallback,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(newTexturesWithNames:scaleFactor:bundle:options:completionHandler:)]
        pub unsafe fn newTexturesWithNames_scaleFactor_bundle_options_completionHandler(
            &self,
            names: &NSArray<NSString>,
            scale_factor: CGFloat,
            bundle: Option<&NSBundle>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderArrayCallback,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(newTexturesWithNames:scaleFactor:displayGamut:bundle:options:completionHandler:)]
        pub unsafe fn newTexturesWithNames_scaleFactor_displayGamut_bundle_options_completionHandler(
            &self,
            names: &NSArray<NSString>,
            scale_factor: CGFloat,
            display_gamut: NSDisplayGamut,
            bundle: Option<&NSBundle>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderArrayCallback,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method(newTextureWithData:options:completionHandler:)]
        pub unsafe fn newTextureWithData_options_completionHandler(
            &self,
            data: &NSData,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderCallback,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "ModelIO_MDLTexture"))]
        #[method(newTextureWithMDLTexture:options:completionHandler:)]
        pub unsafe fn newTextureWithMDLTexture_options_completionHandler(
            &self,
            texture: &MDLTexture,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
            completion_handler: MTKTextureLoaderCallback,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics New newTextureWithContentsOfURL:options:error:_)]
        pub unsafe fn newTextureWithContentsOfURL_options_error(
            &self,
            url: &NSURL,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
        ) -> Result<Id<ProtocolObject<dyn MTLTexture>, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics New newTextureWithData:options:error:_)]
        pub unsafe fn newTextureWithData_options_error(
            &self,
            data: &NSData,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
        ) -> Result<Id<ProtocolObject<dyn MTLTexture>, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "ModelIO_MDLTexture"
        ))]
        #[method_id(@__retain_semantics New newTextureWithMDLTexture:options:error:_)]
        pub unsafe fn newTextureWithMDLTexture_options_error(
            &self,
            texture: &MDLTexture,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
        ) -> Result<Id<ProtocolObject<dyn MTLTexture>, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics New newTextureWithName:scaleFactor:bundle:options:error:_)]
        pub unsafe fn newTextureWithName_scaleFactor_bundle_options_error(
            &self,
            name: &NSString,
            scale_factor: CGFloat,
            bundle: Option<&NSBundle>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
        ) -> Result<Id<ProtocolObject<dyn MTLTexture>, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics New newTextureWithName:scaleFactor:displayGamut:bundle:options:error:_)]
        pub unsafe fn newTextureWithName_scaleFactor_displayGamut_bundle_options_error(
            &self,
            name: &NSString,
            scale_factor: CGFloat,
            display_gamut: NSDisplayGamut,
            bundle: Option<&NSBundle>,
            options: Option<&NSDictionary<MTKTextureLoaderOption, Object>>,
        ) -> Result<Id<ProtocolObject<dyn MTLTexture>, Shared>, Id<NSError, Shared>>;
    }
);
