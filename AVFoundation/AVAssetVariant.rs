//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariant?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariant;
);

unsafe impl Send for AVAssetVariant {}

unsafe impl Sync for AVAssetVariant {}

unsafe impl NSObjectProtocol for AVAssetVariant {}

extern_methods!(
    unsafe impl AVAssetVariant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(peakBitRate)]
        pub unsafe fn peakBitRate(&self) -> c_double;

        #[method(averageBitRate)]
        pub unsafe fn averageBitRate(&self) -> c_double;

        #[method_id(@__retain_semantics Other videoAttributes)]
        pub unsafe fn videoAttributes(&self) -> Option<Retained<AVAssetVariantVideoAttributes>>;

        #[method_id(@__retain_semantics Other audioAttributes)]
        pub unsafe fn audioAttributes(&self) -> Option<Retained<AVAssetVariantAudioAttributes>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantvideoattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantVideoAttributes;
);

unsafe impl NSObjectProtocol for AVAssetVariantVideoAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantVideoAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other videoRange)]
        pub unsafe fn videoRange(&self) -> Retained<AVVideoRange>;

        #[method_id(@__retain_semantics Other codecTypes)]
        pub unsafe fn codecTypes(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(presentationSize)]
        pub unsafe fn presentationSize(&self) -> CGSize;

        #[method(nominalFrameRate)]
        pub unsafe fn nominalFrameRate(&self) -> c_double;

        #[method_id(@__retain_semantics Other videoLayoutAttributes)]
        pub unsafe fn videoLayoutAttributes(
            &self,
        ) -> Retained<NSArray<AVAssetVariantVideoLayoutAttributes>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantvideolayoutattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantVideoLayoutAttributes;
);

unsafe impl Send for AVAssetVariantVideoLayoutAttributes {}

unsafe impl Sync for AVAssetVariantVideoLayoutAttributes {}

unsafe impl NSObjectProtocol for AVAssetVariantVideoLayoutAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantVideoLayoutAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(stereoViewComponents)]
        pub unsafe fn stereoViewComponents(&self) -> CMStereoViewComponents;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantaudioattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantAudioAttributes;
);

unsafe impl Send for AVAssetVariantAudioAttributes {}

unsafe impl Sync for AVAssetVariantAudioAttributes {}

unsafe impl NSObjectProtocol for AVAssetVariantAudioAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantAudioAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other formatIDs)]
        pub unsafe fn formatIDs(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other renditionSpecificAttributesForMediaOption:)]
        pub unsafe fn renditionSpecificAttributesForMediaOption(
            &self,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Option<Retained<AVAssetVariantAudioRenditionSpecificAttributes>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantaudiorenditionspecificattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantAudioRenditionSpecificAttributes;
);

unsafe impl Send for AVAssetVariantAudioRenditionSpecificAttributes {}

unsafe impl Sync for AVAssetVariantAudioRenditionSpecificAttributes {}

unsafe impl NSObjectProtocol for AVAssetVariantAudioRenditionSpecificAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantAudioRenditionSpecificAttributes {
        #[method(channelCount)]
        pub unsafe fn channelCount(&self) -> NSInteger;

        #[method(isBinaural)]
        pub unsafe fn isBinaural(&self) -> bool;

        #[method(isImmersive)]
        pub unsafe fn isImmersive(&self) -> bool;

        #[method(isDownmix)]
        pub unsafe fn isDownmix(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAssetVariantAudioRenditionSpecificAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantqualifier?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantQualifier;
);

unsafe impl Send for AVAssetVariantQualifier {}

unsafe impl Sync for AVAssetVariantQualifier {}

unsafe impl NSCopying for AVAssetVariantQualifier {}

unsafe impl CopyingHelper for AVAssetVariantQualifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVAssetVariantQualifier {}

extern_methods!(
    unsafe impl AVAssetVariantQualifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetVariantQualifierWithPredicate:)]
        pub unsafe fn assetVariantQualifierWithPredicate(predicate: &NSPredicate)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetVariantQualifierWithVariant:)]
        pub unsafe fn assetVariantQualifierWithVariant(variant: &AVAssetVariant) -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetVariantQualifierForMinimumValueInKeyPath:)]
        pub unsafe fn assetVariantQualifierForMinimumValueInKeyPath(
            key_path: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other assetVariantQualifierForMaximumValueInKeyPath:)]
        pub unsafe fn assetVariantQualifierForMaximumValueInKeyPath(
            key_path: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other predicateForChannelCount:mediaSelectionOption:operatorType:)]
        pub unsafe fn predicateForChannelCount_mediaSelectionOption_operatorType(
            channel_count: NSInteger,
            media_selection_option: &AVMediaSelectionOption,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other predicateForBinauralAudio:mediaSelectionOption:)]
        pub unsafe fn predicateForBinauralAudio_mediaSelectionOption(
            is_binaural_audio: bool,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other predicateForImmersiveAudio:mediaSelectionOption:)]
        pub unsafe fn predicateForImmersiveAudio_mediaSelectionOption(
            is_immersive_audio: bool,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other predicateForDownmixAudio:mediaSelectionOption:)]
        pub unsafe fn predicateForDownmixAudio_mediaSelectionOption(
            is_downmix_audio: bool,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other predicateForPresentationWidth:operatorType:)]
        pub unsafe fn predicateForPresentationWidth_operatorType(
            width: CGFloat,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other predicateForPresentationHeight:operatorType:)]
        pub unsafe fn predicateForPresentationHeight_operatorType(
            height: CGFloat,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other predicateForAudioSampleRate:mediaSelectionOption:operatorType:)]
        pub unsafe fn predicateForAudioSampleRate_mediaSelectionOption_operatorType(
            sample_rate: c_double,
            media_selection_option: &AVMediaSelectionOption,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;
    }
);
