//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// AVVideoSettings
    ///
    /// NSDictionary keys for configuring output video format
    ///
    ///
    /// A video settings dictionary may take one of two forms:
    ///
    /// 1. For compressed video output, use only the keys in this header, AVVideoSettings.h.
    /// 2. For uncompressed video output, start with kCVPixelBuffer* keys in
    /// <CoreVideo
    /// /CVPixelBuffer.h>.
    ///
    /// In addition to the keys in CVPixelBuffer.h, uncompressed video settings dictionaries may also contain the following keys:
    ///
    /// AVVideoPixelAspectRatioKey
    /// AVVideoCleanApertureKey
    /// AVVideoScalingModeKey
    /// AVVideoColorPropertiesKey
    /// AVVideoAllowWideColorKey
    ///
    /// It is an error to add any other AVVideoSettings.h keys to an uncompressed video settings dictionary.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodeckey?language=objc)
    pub static AVVideoCodecKey: Option<&'static NSString>;
}

/// The type of the strings used to specify a video codec type (for instance, as values for the AVVideoCodecKey key in a video settings dictionary).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectype?language=objc)
// NS_TYPED_ENUM
pub type AVVideoCodecType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypehevc?language=objc)
    pub static AVVideoCodecTypeHEVC: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeh264?language=objc)
    pub static AVVideoCodecTypeH264: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypejpeg?language=objc)
    pub static AVVideoCodecTypeJPEG: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypejpegxl?language=objc)
    pub static AVVideoCodecTypeJPEGXL: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeappleprores4444?language=objc)
    pub static AVVideoCodecTypeAppleProRes4444: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeappleprores4444xq?language=objc)
    pub static AVVideoCodecTypeAppleProRes4444XQ: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeappleprores422?language=objc)
    pub static AVVideoCodecTypeAppleProRes422: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeappleprores422hq?language=objc)
    pub static AVVideoCodecTypeAppleProRes422HQ: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeappleprores422lt?language=objc)
    pub static AVVideoCodecTypeAppleProRes422LT: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypeappleprores422proxy?language=objc)
    pub static AVVideoCodecTypeAppleProRes422Proxy: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodectypehevcwithalpha?language=objc)
    pub static AVVideoCodecTypeHEVCWithAlpha: Option<&'static AVVideoCodecType>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodechevc?language=objc)
    pub static AVVideoCodecHEVC: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodech264?language=objc)
    pub static AVVideoCodecH264: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodecjpeg?language=objc)
    pub static AVVideoCodecJPEG: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodecappleprores4444?language=objc)
    pub static AVVideoCodecAppleProRes4444: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocodecappleprores422?language=objc)
    pub static AVVideoCodecAppleProRes422: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideowidthkey?language=objc)
    pub static AVVideoWidthKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoheightkey?language=objc)
    pub static AVVideoHeightKey: Option<&'static NSString>;
}

extern "C" {
    /// The aspect ratio of the pixels in the video frame
    ///
    /// The value for this key is an NSDictionary containing AVVideoPixelAspectRatio*Key keys.  If no value is specified for this key, the default value for the codec is used.  Usually this is 1:1, meaning square pixels.
    ///
    /// Note that prior to macOS 10.9 and iOS 7.0, this key could only be specified as part of the dictionary given for AVVideoCompressionPropertiesKey.  As of macOS 10.9 and iOS 7.0, the top level of an AVVideoSettings dictionary is the preferred place to specify this key.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideopixelaspectratiokey?language=objc)
    pub static AVVideoPixelAspectRatioKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideopixelaspectratiohorizontalspacingkey?language=objc)
    pub static AVVideoPixelAspectRatioHorizontalSpacingKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideopixelaspectratioverticalspacingkey?language=objc)
    pub static AVVideoPixelAspectRatioVerticalSpacingKey: Option<&'static NSString>;
}

extern "C" {
    /// Defines the region within the video dimensions that will be displayed during playback
    ///
    /// The value for this key is an NSDictionary containing AVVideoCleanAperture*Key keys.  AVVideoCleanApertureWidthKey and AVVideoCleanApertureHeightKey define a clean rectangle which is centered on the video frame.  To offset this rectangle from center, use AVVideoCleanApertureHorizontalOffsetKey and AVVideoCleanApertureVerticalOffsetKey.  A positive value for AVVideoCleanApertureHorizontalOffsetKey moves the clean aperture region to the right, and a positive value for AVVideoCleanApertureVerticalOffsetKey moves the clean aperture region down.
    ///
    /// If no clean aperture region is specified, the entire frame will be displayed during playback.
    ///
    /// Note that prior to macOS 10.9 and iOS 7.0, this key could only be specified as part of the dictionary given for AVVideoCompressionPropertiesKey.  As of macOS 10.9 and iOS 7.0, the top level of an AVVideoSettings dictionary is the preferred place to specify this key.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocleanaperturekey?language=objc)
    pub static AVVideoCleanApertureKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocleanaperturewidthkey?language=objc)
    pub static AVVideoCleanApertureWidthKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocleanapertureheightkey?language=objc)
    pub static AVVideoCleanApertureHeightKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocleanaperturehorizontaloffsetkey?language=objc)
    pub static AVVideoCleanApertureHorizontalOffsetKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocleanapertureverticaloffsetkey?language=objc)
    pub static AVVideoCleanApertureVerticalOffsetKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoscalingmodekey?language=objc)
    pub static AVVideoScalingModeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoscalingmodefit?language=objc)
    pub static AVVideoScalingModeFit: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoscalingmoderesize?language=objc)
    pub static AVVideoScalingModeResize: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoscalingmoderesizeaspect?language=objc)
    pub static AVVideoScalingModeResizeAspect: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoscalingmoderesizeaspectfill?language=objc)
    pub static AVVideoScalingModeResizeAspectFill: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorpropertieskey?language=objc)
    pub static AVVideoColorPropertiesKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorprimarieskey?language=objc)
    pub static AVVideoColorPrimariesKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorprimaries_itu_r_709_2?language=objc)
    pub static AVVideoColorPrimaries_ITU_R_709_2: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorprimaries_ebu_3213?language=objc)
    pub static AVVideoColorPrimaries_EBU_3213: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorprimaries_smpte_c?language=objc)
    pub static AVVideoColorPrimaries_SMPTE_C: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorprimaries_p3_d65?language=objc)
    pub static AVVideoColorPrimaries_P3_D65: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocolorprimaries_itu_r_2020?language=objc)
    pub static AVVideoColorPrimaries_ITU_R_2020: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunctionkey?language=objc)
    pub static AVVideoTransferFunctionKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunction_itu_r_709_2?language=objc)
    pub static AVVideoTransferFunction_ITU_R_709_2: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunction_smpte_240m_1995?language=objc)
    pub static AVVideoTransferFunction_SMPTE_240M_1995: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunction_smpte_st_2084_pq?language=objc)
    pub static AVVideoTransferFunction_SMPTE_ST_2084_PQ: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunction_itu_r_2100_hlg?language=objc)
    pub static AVVideoTransferFunction_ITU_R_2100_HLG: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunction_linear?language=objc)
    pub static AVVideoTransferFunction_Linear: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideotransferfunction_iec_srgb?language=objc)
    pub static AVVideoTransferFunction_IEC_sRGB: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoycbcrmatrixkey?language=objc)
    pub static AVVideoYCbCrMatrixKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoycbcrmatrix_itu_r_709_2?language=objc)
    pub static AVVideoYCbCrMatrix_ITU_R_709_2: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoycbcrmatrix_itu_r_601_4?language=objc)
    pub static AVVideoYCbCrMatrix_ITU_R_601_4: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoycbcrmatrix_smpte_240m_1995?language=objc)
    pub static AVVideoYCbCrMatrix_SMPTE_240M_1995: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoycbcrmatrix_itu_r_2020?language=objc)
    pub static AVVideoYCbCrMatrix_ITU_R_2020: Option<&'static NSString>;
}

extern "C" {
    /// Indicates whether the client can process wide color
    ///
    /// Clients who wish to process wide color content should set the value of this key to
    /// `true`, or specify AVVideoColorPropertiesKey.
    ///
    /// The default value,
    /// `false`, permits implicit color conversions to occur to a non-wide gamut color space.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoallowwidecolorkey?language=objc)
    pub static AVVideoAllowWideColorKey: Option<&'static NSString>;
}

extern "C" {
    /// The value for this key is an instance of NSDictionary, containing properties to be passed down to the video encoder.
    ///
    /// Package the below keys in an instance of NSDictionary and use it as the value for AVVideoCompressionPropertiesKey in the top-level AVVideoSettings dictionary.  In addition to the keys listed below, you can also include keys from VideoToolbox/VTCompressionProperties.h.
    ///
    /// Most keys can only be used for certain encoders.  Look at individual keys for details.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideocompressionpropertieskey?language=objc)
    pub static AVVideoCompressionPropertiesKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoaveragebitratekey?language=objc)
    pub static AVVideoAverageBitRateKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoqualitykey?language=objc)
    pub static AVVideoQualityKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideomaxkeyframeintervalkey?language=objc)
    pub static AVVideoMaxKeyFrameIntervalKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideomaxkeyframeintervaldurationkey?language=objc)
    pub static AVVideoMaxKeyFrameIntervalDurationKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoappleprorawbitdepthkey?language=objc)
    pub static AVVideoAppleProRAWBitDepthKey: Option<&'static NSString>;
}

extern "C" {
    /// Enables or disables frame reordering.
    ///
    /// In order to achieve the best compression while maintaining image quality, some video encoders can reorder frames.  This means that the order in which the frames will be emitted and stored (the decode order) will be different from the order in which they are presented to the video encoder (the display order).
    ///
    /// Encoding using frame reordering requires more system resources than encoding without frame reordering, so encoding performance should be taken into account when deciding whether to enable frame reordering.  This is especially important when encoding video data from a real-time source, such as AVCaptureVideoDataOutput.  In this situation, using a value of
    /// `false`for AVVideoAllowFrameReorderingKey may yield the best results.
    ///
    /// The default is
    /// `true`, which means that the encoder decides whether to enable frame reordering.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoallowframereorderingkey?language=objc)
    pub static AVVideoAllowFrameReorderingKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelkey?language=objc)
    pub static AVVideoProfileLevelKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264baseline30?language=objc)
    pub static AVVideoProfileLevelH264Baseline30: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264baseline31?language=objc)
    pub static AVVideoProfileLevelH264Baseline31: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264baseline41?language=objc)
    pub static AVVideoProfileLevelH264Baseline41: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264baselineautolevel?language=objc)
    pub static AVVideoProfileLevelH264BaselineAutoLevel: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264main30?language=objc)
    pub static AVVideoProfileLevelH264Main30: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264main31?language=objc)
    pub static AVVideoProfileLevelH264Main31: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264main32?language=objc)
    pub static AVVideoProfileLevelH264Main32: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264main41?language=objc)
    pub static AVVideoProfileLevelH264Main41: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264mainautolevel?language=objc)
    pub static AVVideoProfileLevelH264MainAutoLevel: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264high40?language=objc)
    pub static AVVideoProfileLevelH264High40: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264high41?language=objc)
    pub static AVVideoProfileLevelH264High41: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoprofilelevelh264highautolevel?language=objc)
    pub static AVVideoProfileLevelH264HighAutoLevel: Option<&'static NSString>;
}

extern "C" {
    /// The entropy encoding mode for H.264 compression.
    ///
    /// If supported by an H.264 encoder, this property controls whether the encoder should use Context-based Adaptive Variable Length Coding (CAVLC) or Context-based Adaptive Binary Arithmetic Coding (CABAC).  CABAC generally gives better compression at the expense of higher computational overhead.  The default value is encoder-specific and may change depending on other encoder settings.  Care should be taken when using this property -- changes may result in a configuration which is not compatible with a requested Profile and Level.  Results in this case are undefined, and could include encode errors or a non-compliant output stream.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoh264entropymodekey?language=objc)
    pub static AVVideoH264EntropyModeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoh264entropymodecavlc?language=objc)
    pub static AVVideoH264EntropyModeCAVLC: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoh264entropymodecabac?language=objc)
    pub static AVVideoH264EntropyModeCABAC: Option<&'static NSString>;
}

extern "C" {
    /// Indicates the expected source frame rate, if known.
    ///
    /// The frame rate is measured in frames per second. This is not used to control the frame rate; it is provided as a hint to the video encoder so that it can set up internal configuration before compression begins. The actual frame rate will depend on frame durations and may vary. This should be set if an AutoLevel AVVideoProfileLevelKey is used, or if the source content has a high frame rate (higher than 30 fps). The encoder might have to drop frames to satisfy bit stream requirements if this key is not specified.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoexpectedsourceframeratekey?language=objc)
    pub static AVVideoExpectedSourceFrameRateKey: Option<&'static NSString>;
}

extern "C" {
    /// The desired average number of non-droppable frames to be encoded for each second of video.
    ///
    /// Some video encoders can produce a flexible mixture of non-droppable frames and droppable frames.  The difference between these types is that it is necessary for a video decoder to decode a non-droppable frame in order to successfully decode subsequent frames, whereas droppable frames are optional and can be skipped without impact on decode of subsequent frames.  Having a proportion of droppable frames in a sequence has advantages for temporal scalability: at playback time more or fewer frames may be decoded depending on the play rate.  This property requests that the encoder emit an overall proportion of non-droppable and droppable frames so that there are the specified number of non-droppable frames per second.
    ///
    /// For example, to specify that the encoder should include an average of 30 non-droppable frames for each second of video:
    ///
    /// [myVideoSettings setObject:
    /// @
    /// 30 forKey:AVVideoAverageNonDroppableFrameRateKey];
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoaveragenondroppableframeratekey?language=objc)
    pub static AVVideoAverageNonDroppableFrameRateKey: Option<&'static NSString>;
}

extern "C" {
    /// The value for this key is an instance of NSDictionary, containing properties to be passed down to the video decoder.
    ///
    /// Package the below keys in an instance of NSDictionary and use it as the value for AVVideoDecompressionPropertiesKey in the top-level AVVideoSettings dictionary.  In addition to the keys listed below, you can also include keys from VideoToolbox/VTDecompressionProperties.h.
    ///
    /// Most keys can only be used for certain decoders.  Look at individual keys for details.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideodecompressionpropertieskey?language=objc)
    pub static AVVideoDecompressionPropertiesKey: Option<&'static NSString>;
}

extern "C" {
    /// The video encoder specification includes options for choosing a specific video encoder.
    ///
    ///
    /// The value for this key is a dictionary containing kVTVideoEncoderSpecification_* keys specified in the VideoToolbox framework.  This key should be specified at the top level of an AVVideoSettings dictionary.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoencoderspecificationkey?language=objc)
    pub static AVVideoEncoderSpecificationKey: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoaperturemode?language=objc)
// NS_TYPED_ENUM
pub type AVVideoApertureMode = NSString;

extern "C" {
    /// Both pixel aspect ratio and clean aperture will be applied.
    ///
    /// An image's clean aperture is a region of video free from transition artifacts caused by the encoding of the signal.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoaperturemodecleanaperture?language=objc)
    pub static AVVideoApertureModeCleanAperture: Option<&'static AVVideoApertureMode>;
}

extern "C" {
    /// Only pixel aspect ratio will be applied.
    ///
    /// The image is not cropped to the clean aperture region, but it is scaled according to the pixel aspect ratio. Use this option when you want to see all the pixels in your video, including the edges.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoaperturemodeproductionaperture?language=objc)
    pub static AVVideoApertureModeProductionAperture: Option<&'static AVVideoApertureMode>;
}

extern "C" {
    /// Neither pixel aspect ratio nor clean aperture will be applied.
    ///
    /// The image is not cropped to the clean aperture region and is not scaled according to the pixel aspect ratio. The encoded dimensions of the image description are displayed.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoaperturemodeencodedpixels?language=objc)
    pub static AVVideoApertureModeEncodedPixels: Option<&'static AVVideoApertureMode>;
}
