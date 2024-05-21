//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(not(target_os = "watchos"))]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageOrientation(pub NSInteger);
impl UIImageOrientation {
    #[doc(alias = "UIImageOrientationUp")]
    pub const Up: Self = Self(0);
    #[doc(alias = "UIImageOrientationDown")]
    pub const Down: Self = Self(1);
    #[doc(alias = "UIImageOrientationLeft")]
    pub const Left: Self = Self(2);
    #[doc(alias = "UIImageOrientationRight")]
    pub const Right: Self = Self(3);
    #[doc(alias = "UIImageOrientationUpMirrored")]
    pub const UpMirrored: Self = Self(4);
    #[doc(alias = "UIImageOrientationDownMirrored")]
    pub const DownMirrored: Self = Self(5);
    #[doc(alias = "UIImageOrientationLeftMirrored")]
    pub const LeftMirrored: Self = Self(6);
    #[doc(alias = "UIImageOrientationRightMirrored")]
    pub const RightMirrored: Self = Self(7);
}

unsafe impl Encode for UIImageOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageResizingMode(pub NSInteger);
impl UIImageResizingMode {
    #[doc(alias = "UIImageResizingModeTile")]
    pub const Tile: Self = Self(0);
    #[doc(alias = "UIImageResizingModeStretch")]
    pub const Stretch: Self = Self(1);
}

unsafe impl Encode for UIImageResizingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageResizingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImageRenderingMode(pub NSInteger);
impl UIImageRenderingMode {
    #[doc(alias = "UIImageRenderingModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIImageRenderingModeAlwaysOriginal")]
    pub const AlwaysOriginal: Self = Self(1);
    #[doc(alias = "UIImageRenderingModeAlwaysTemplate")]
    pub const AlwaysTemplate: Self = Self(2);
}

unsafe impl Encode for UIImageRenderingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImageRenderingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIImage;

    unsafe impl ClassType for UIImage {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for UIImage {}

unsafe impl Sync for UIImage {}

unsafe impl NSCoding for UIImage {}

unsafe impl NSObjectProtocol for UIImage {}

unsafe impl NSSecureCoding for UIImage {}

extern_methods!(
    unsafe impl UIImage {
        #[method_id(@__retain_semantics Other systemImageNamed:)]
        pub unsafe fn systemImageNamed(name: &NSString) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other systemImageNamed:withConfiguration:)]
        pub unsafe fn systemImageNamed_withConfiguration(
            name: &NSString,
            configuration: Option<&UIImageConfiguration>,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other systemImageNamed:compatibleWithTraitCollection:)]
        pub unsafe fn systemImageNamed_compatibleWithTraitCollection(
            name: &NSString,
            trait_collection: Option<&UITraitCollection>,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other systemImageNamed:variableValue:withConfiguration:)]
        pub unsafe fn systemImageNamed_variableValue_withConfiguration(
            name: &NSString,
            value: c_double,
            configuration: Option<&UIImageConfiguration>,
        ) -> Option<Id<UIImage>>;

        #[method_id(@__retain_semantics Other imageNamed:)]
        pub unsafe fn imageNamed(name: &NSString) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other imageNamed:inBundle:withConfiguration:)]
        pub unsafe fn imageNamed_inBundle_withConfiguration(
            name: &NSString,
            bundle: Option<&NSBundle>,
            configuration: Option<&UIImageConfiguration>,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other imageNamed:inBundle:compatibleWithTraitCollection:)]
        pub unsafe fn imageNamed_inBundle_compatibleWithTraitCollection(
            name: &NSString,
            bundle: Option<&NSBundle>,
            trait_collection: Option<&UITraitCollection>,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other imageNamed:inBundle:variableValue:withConfiguration:)]
        pub unsafe fn imageNamed_inBundle_variableValue_withConfiguration(
            name: &NSString,
            bundle: Option<&NSBundle>,
            value: c_double,
            configuration: Option<&UIImageConfiguration>,
        ) -> Option<Id<UIImage>>;

        #[method_id(@__retain_semantics Other imageWithContentsOfFile:)]
        pub unsafe fn imageWithContentsOfFile(path: &NSString) -> Option<Id<UIImage>>;

        #[method_id(@__retain_semantics Other imageWithData:)]
        pub unsafe fn imageWithData(data: &NSData) -> Option<Id<UIImage>>;

        #[method_id(@__retain_semantics Other imageWithData:scale:)]
        pub unsafe fn imageWithData_scale(data: &NSData, scale: CGFloat) -> Option<Id<UIImage>>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other imageWithCIImage:)]
        pub unsafe fn imageWithCIImage(ci_image: &CIImage) -> Id<UIImage>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other imageWithCIImage:scale:orientation:)]
        pub unsafe fn imageWithCIImage_scale_orientation(
            ci_image: &CIImage,
            scale: CGFloat,
            orientation: UIImageOrientation,
        ) -> Id<UIImage>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithData:scale:)]
        pub unsafe fn initWithData_scale(
            this: Allocated<Self>,
            data: &NSData,
            scale: CGFloat,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithCIImage:)]
        pub unsafe fn initWithCIImage(this: Allocated<Self>, ci_image: &CIImage) -> Id<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithCIImage:scale:orientation:)]
        pub unsafe fn initWithCIImage_scale_orientation(
            this: Allocated<Self>,
            ci_image: &CIImage,
            scale: CGFloat,
            orientation: UIImageOrientation,
        ) -> Id<Self>;

        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other CIImage)]
        pub unsafe fn CIImage(&self) -> Option<Id<CIImage>>;

        #[method(imageOrientation)]
        pub unsafe fn imageOrientation(&self) -> UIImageOrientation;

        #[method(scale)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[method(isSymbolImage)]
        pub unsafe fn isSymbolImage(&self) -> bool;

        #[method_id(@__retain_semantics Other animatedImageNamed:duration:)]
        pub unsafe fn animatedImageNamed_duration(
            name: &NSString,
            duration: NSTimeInterval,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other animatedResizableImageNamed:capInsets:duration:)]
        pub unsafe fn animatedResizableImageNamed_capInsets_duration(
            name: &NSString,
            cap_insets: UIEdgeInsets,
            duration: NSTimeInterval,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other animatedResizableImageNamed:capInsets:resizingMode:duration:)]
        pub unsafe fn animatedResizableImageNamed_capInsets_resizingMode_duration(
            name: &NSString,
            cap_insets: UIEdgeInsets,
            resizing_mode: UIImageResizingMode,
            duration: NSTimeInterval,
        ) -> Option<Id<UIImage>>;

        #[method_id(@__retain_semantics Other animatedImageWithImages:duration:)]
        pub unsafe fn animatedImageWithImages_duration(
            images: &NSArray<UIImage>,
            duration: NSTimeInterval,
        ) -> Option<Id<UIImage>>;

        #[method_id(@__retain_semantics Other images)]
        pub unsafe fn images(&self) -> Option<Id<NSArray<UIImage>>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: CGPoint);

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: CGRect);

        #[method(drawAsPatternInRect:)]
        pub unsafe fn drawAsPatternInRect(&self, rect: CGRect);

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other resizableImageWithCapInsets:)]
        pub unsafe fn resizableImageWithCapInsets(&self, cap_insets: UIEdgeInsets) -> Id<UIImage>;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other resizableImageWithCapInsets:resizingMode:)]
        pub unsafe fn resizableImageWithCapInsets_resizingMode(
            &self,
            cap_insets: UIEdgeInsets,
            resizing_mode: UIImageResizingMode,
        ) -> Id<UIImage>;

        #[cfg(feature = "UIGeometry")]
        #[method(capInsets)]
        pub unsafe fn capInsets(&self) -> UIEdgeInsets;

        #[method(resizingMode)]
        pub unsafe fn resizingMode(&self) -> UIImageResizingMode;

        #[cfg(feature = "UIGeometry")]
        #[method_id(@__retain_semantics Other imageWithAlignmentRectInsets:)]
        pub unsafe fn imageWithAlignmentRectInsets(
            &self,
            alignment_insets: UIEdgeInsets,
        ) -> Id<UIImage>;

        #[cfg(feature = "UIGeometry")]
        #[method(alignmentRectInsets)]
        pub unsafe fn alignmentRectInsets(&self) -> UIEdgeInsets;

        #[method_id(@__retain_semantics Other imageWithRenderingMode:)]
        pub unsafe fn imageWithRenderingMode(
            &self,
            rendering_mode: UIImageRenderingMode,
        ) -> Id<UIImage>;

        #[method(renderingMode)]
        pub unsafe fn renderingMode(&self) -> UIImageRenderingMode;

        #[cfg(all(feature = "UIGraphicsImageRenderer", feature = "UIGraphicsRenderer"))]
        #[method_id(@__retain_semantics Other imageRendererFormat)]
        pub unsafe fn imageRendererFormat(&self) -> Id<UIGraphicsImageRendererFormat>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitCollection)]
        pub unsafe fn traitCollection(&self) -> Id<UITraitCollection>;

        #[cfg(feature = "UIImageAsset")]
        #[method_id(@__retain_semantics Other imageAsset)]
        pub unsafe fn imageAsset(&self) -> Option<Id<UIImageAsset>>;

        #[method_id(@__retain_semantics Other imageFlippedForRightToLeftLayoutDirection)]
        pub unsafe fn imageFlippedForRightToLeftLayoutDirection(&self) -> Id<UIImage>;

        #[method(flipsForRightToLeftLayoutDirection)]
        pub unsafe fn flipsForRightToLeftLayoutDirection(&self) -> bool;

        #[method_id(@__retain_semantics Other imageWithHorizontallyFlippedOrientation)]
        pub unsafe fn imageWithHorizontallyFlippedOrientation(&self) -> Id<UIImage>;

        #[method(baselineOffsetFromBottom)]
        pub unsafe fn baselineOffsetFromBottom(&self) -> CGFloat;

        #[method(hasBaseline)]
        pub unsafe fn hasBaseline(&self) -> bool;

        #[method_id(@__retain_semantics Other imageWithBaselineOffsetFromBottom:)]
        pub unsafe fn imageWithBaselineOffsetFromBottom(
            &self,
            baseline_offset: CGFloat,
        ) -> Id<UIImage>;

        #[method_id(@__retain_semantics Other imageWithoutBaseline)]
        pub unsafe fn imageWithoutBaseline(&self) -> Id<UIImage>;

        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Id<UIImageConfiguration>>;

        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other imageWithConfiguration:)]
        pub unsafe fn imageWithConfiguration(
            &self,
            configuration: &UIImageConfiguration,
        ) -> Id<UIImage>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Id<UIImageSymbolConfiguration>>;

        #[cfg(all(
            feature = "UIImageConfiguration",
            feature = "UIImageSymbolConfiguration"
        ))]
        #[method_id(@__retain_semantics Other imageByApplyingSymbolConfiguration:)]
        pub unsafe fn imageByApplyingSymbolConfiguration(
            &self,
            configuration: &UIImageSymbolConfiguration,
        ) -> Option<Id<UIImage>>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other imageWithTintColor:)]
        pub unsafe fn imageWithTintColor(&self, color: &UIColor) -> Id<UIImage>;

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other imageWithTintColor:renderingMode:)]
        pub unsafe fn imageWithTintColor_renderingMode(
            &self,
            color: &UIColor,
            rendering_mode: UIImageRenderingMode,
        ) -> Id<UIImage>;

        #[method_id(@__retain_semantics Other imageByPreparingForDisplay)]
        pub unsafe fn imageByPreparingForDisplay(&self) -> Option<Id<UIImage>>;

        #[cfg(feature = "block2")]
        #[method(prepareForDisplayWithCompletionHandler:)]
        pub unsafe fn prepareForDisplayWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut UIImage)>,
        );

        #[method_id(@__retain_semantics Other imageByPreparingThumbnailOfSize:)]
        pub unsafe fn imageByPreparingThumbnailOfSize(&self, size: CGSize) -> Option<Id<UIImage>>;

        #[cfg(feature = "block2")]
        #[method(prepareThumbnailOfSize:completionHandler:)]
        pub unsafe fn prepareThumbnailOfSize_completionHandler(
            &self,
            size: CGSize,
            completion_handler: &block2::Block<dyn Fn(*mut UIImage)>,
        );

        #[method(isHighDynamicRange)]
        pub unsafe fn isHighDynamicRange(&self) -> bool;

        #[method_id(@__retain_semantics Other imageRestrictedToStandardDynamicRange)]
        pub unsafe fn imageRestrictedToStandardDynamicRange(&self) -> Id<UIImage>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIImage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// PreconfiguredSystemImages
    unsafe impl UIImage {
        #[method_id(@__retain_semantics Other actionsImage)]
        pub unsafe fn actionsImage() -> Id<UIImage>;

        #[method_id(@__retain_semantics Other addImage)]
        pub unsafe fn addImage() -> Id<UIImage>;

        #[method_id(@__retain_semantics Other removeImage)]
        pub unsafe fn removeImage() -> Id<UIImage>;

        #[method_id(@__retain_semantics Other checkmarkImage)]
        pub unsafe fn checkmarkImage() -> Id<UIImage>;

        #[method_id(@__retain_semantics Other strokedCheckmarkImage)]
        pub unsafe fn strokedCheckmarkImage() -> Id<UIImage>;
    }
);

extern_methods!(
    /// NSItemProvider
    unsafe impl UIImage {}
);

unsafe impl NSItemProviderReading for UIImage {}

unsafe impl NSItemProviderWriting for UIImage {}

extern_methods!(
    /// UIImage
    #[cfg(feature = "NSTextAttachment")]
    unsafe impl NSTextAttachment {
        #[method_id(@__retain_semantics Other textAttachmentWithImage:)]
        pub unsafe fn textAttachmentWithImage(image: &UIImage) -> Id<NSTextAttachment>;
    }
);

extern_methods!(
    /// UIImageDeprecated
    unsafe impl UIImage {
        #[method_id(@__retain_semantics Other stretchableImageWithLeftCapWidth:topCapHeight:)]
        pub unsafe fn stretchableImageWithLeftCapWidth_topCapHeight(
            &self,
            left_cap_width: NSInteger,
            top_cap_height: NSInteger,
        ) -> Id<UIImage>;

        #[method(leftCapWidth)]
        pub unsafe fn leftCapWidth(&self) -> NSInteger;

        #[method(topCapHeight)]
        pub unsafe fn topCapHeight(&self) -> NSInteger;
    }
);

extern_category!(
    /// Category "UIKitAdditions" on [`CIImage`].
    #[doc(alias = "UIKitAdditions")]
    pub unsafe trait CIImageUIKitAdditions {
        #[method_id(@__retain_semantics Init initWithImage:)]
        unsafe fn initWithImage(this: Allocated<Self>, image: &UIImage) -> Option<Id<Self>>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Init initWithImage:options:)]
        unsafe fn initWithImage_options(
            this: Allocated<Self>,
            image: &UIImage,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Id<Self>>;
    }

    #[cfg(feature = "objc2-core-image")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl CIImageUIKitAdditions for CIImage {}
);

extern "C" {
    pub fn UIImagePNGRepresentation(image: &UIImage) -> *mut NSData;
}

extern "C" {
    pub fn UIImageJPEGRepresentation(image: &UIImage, compression_quality: CGFloat) -> *mut NSData;
}

extern "C" {
    pub fn UIImageHEICRepresentation(image: &UIImage) -> *mut NSData;
}
