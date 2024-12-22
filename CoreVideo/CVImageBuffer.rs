//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercgcolorspacekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferCGColorSpaceKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercleanaperturekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferCleanApertureKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercleanaperturewidthkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferCleanApertureWidthKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercleanapertureheightkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferCleanApertureHeightKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercleanaperturehorizontaloffsetkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercleanapertureverticaloffsetkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferpreferredcleanaperturekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferPreferredCleanApertureKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferfieldcountkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferFieldCountKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferfielddetailkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferFieldDetailKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferfielddetailtemporaltopfirst?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferFieldDetailTemporalTopFirst: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferfielddetailtemporalbottomfirst?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferFieldDetailTemporalBottomFirst: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferfielddetailspatialfirstlineearly?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferFieldDetailSpatialFirstLineEarly: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferfielddetailspatialfirstlinelate?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferFieldDetailSpatialFirstLineLate: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferpixelaspectratiokey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferPixelAspectRatioKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferpixelaspectratiohorizontalspacingkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferPixelAspectRatioHorizontalSpacingKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferpixelaspectratioverticalspacingkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferPixelAspectRatioVerticalSpacingKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferdisplaydimensionskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferDisplayDimensionsKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferdisplaywidthkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferDisplayWidthKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferdisplayheightkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferDisplayHeightKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffergammalevelkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferGammaLevelKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffericcprofilekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferICCProfileKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrixkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrixKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrix_itu_r_709_2?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrix_itu_r_601_4?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrix_smpte_240m_1995?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrix_dci_p3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrix_DCI_P3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrix_p3_d65?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrix_P3_D65: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferycbcrmatrix_itu_r_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimarieskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimariesKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_itu_r_709_2?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_ITU_R_709_2: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_ebu_3213?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_EBU_3213: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_smpte_c?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_SMPTE_C: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_p22?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_P22: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_dci_p3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_DCI_P3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_p3_d65?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_P3_D65: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercolorprimaries_itu_r_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferColorPrimaries_ITU_R_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunctionkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunctionKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_itu_r_709_2?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_ITU_R_709_2: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_smpte_240m_1995?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_SMPTE_240M_1995: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_usegamma?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_UseGamma: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_ebu_3213?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_EBU_3213: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_smpte_c?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_SMPTE_C: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_srgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_sRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_itu_r_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_ITU_R_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_smpte_st_428_1?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_SMPTE_ST_428_1: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_smpte_st_2084_pq?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_itu_r_2100_hlg?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_ITU_R_2100_HLG: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffertransferfunction_linear?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferTransferFunction_Linear: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocationtopfieldkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocationTopFieldKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocationbottomfieldkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocationBottomFieldKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_left?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_Left: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_center?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_Center: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_topleft?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_TopLeft: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_top?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_Top: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_bottomleft?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_BottomLeft: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_bottom?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_Bottom: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromalocation_dv420?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaLocation_DV420: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromasubsamplingkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaSubsamplingKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromasubsampling_420?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaSubsampling_420: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromasubsampling_422?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaSubsampling_422: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferchromasubsampling_411?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferChromaSubsampling_411: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferalphachannelisopaque?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferAlphaChannelIsOpaque: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferalphachannelmodekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferAlphaChannelModeKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferalphachannelmode_straightalpha?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferAlphaChannelMode_StraightAlpha: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferalphachannelmode_premultipliedalpha?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferAlphaChannelMode_PremultipliedAlpha: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferpostdecodeprocessingsequencemetadatakey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferPostDecodeProcessingSequenceMetadataKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferpostdecodeprocessingframemetadatakey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferPostDecodeProcessingFrameMetadataKey: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVYCbCrMatrixGetIntegerCodePointForString(y_cb_cr_matrix_string: CFStringRef) -> c_int;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVColorPrimariesGetIntegerCodePointForString(
        color_primaries_string: CFStringRef,
    ) -> c_int;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVTransferFunctionGetIntegerCodePointForString(
        transfer_function_string: CFStringRef,
    ) -> c_int;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVYCbCrMatrixGetStringForIntegerCodePoint(
        y_cb_cr_matrix_code_point: c_int,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVColorPrimariesGetStringForIntegerCodePoint(
        color_primaries_code_point: c_int,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVTransferFunctionGetStringForIntegerCodePoint(
        transfer_function_code_point: c_int,
    ) -> CFStringRef;
}

/// Base type for all CoreVideo image buffers
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvimagebufferref?language=objc)
#[cfg(feature = "CVBuffer")]
pub type CVImageBufferRef = CVBufferRef;

extern "C-unwind" {
    /// Returns the full encoded dimensions of a CVImageBuffer.  For example, for an NTSC DV frame this would be 720x480
    ///
    /// Parameter `imageBuffer`: A CVImageBuffer that you wish to retrieve the encoded size from.
    ///
    /// Returns: A CGSize returning the full encoded size of the buffer
    /// Returns zero size if called with a non-CVImageBufferRef type or NULL.
    #[cfg(all(feature = "CVBuffer", feature = "objc2-core-foundation"))]
    pub fn CVImageBufferGetEncodedSize(image_buffer: CVImageBufferRef) -> CGSize;
}

extern "C-unwind" {
    /// Returns the nominal output display size (in square pixels) of a CVImageBuffer.
    /// For example, for an NTSC DV frame this would be 640x480
    ///
    /// Parameter `imageBuffer`: A CVImageBuffer that you wish to retrieve the display size from.
    ///
    /// Returns: A CGSize returning the nominal display size of the buffer
    /// Returns zero size if called with a non-CVImageBufferRef type or NULL.
    #[cfg(all(feature = "CVBuffer", feature = "objc2-core-foundation"))]
    pub fn CVImageBufferGetDisplaySize(image_buffer: CVImageBufferRef) -> CGSize;
}

extern "C-unwind" {
    /// Returns the source rectangle of a CVImageBuffer that represents the clean aperture
    /// of the buffer in encoded pixels.    For example, an NTSC DV frame would return a CGRect with an
    /// origin of 8,0 and a size of 704,480.
    /// Note that the origin of this rect always the lower left    corner.   This is the same coordinate system as
    /// used by CoreImage.
    ///
    /// Parameter `imageBuffer`: A CVImageBuffer that you wish to retrieve the display size from.
    ///
    /// Returns: A CGSize returning the nominal display size of the buffer
    /// Returns zero rect if called with a non-CVImageBufferRef type or NULL.
    #[cfg(all(feature = "CVBuffer", feature = "objc2-core-foundation"))]
    pub fn CVImageBufferGetCleanRect(image_buffer: CVImageBufferRef) -> CGRect;
}

extern "C-unwind" {
    /// Returns whether the image is flipped vertically or not.
    ///
    /// Parameter `imageBuffer`: target
    ///
    /// Returns: True if 0,0 in the texture is upper left, false if 0,0 is lower left.
    #[cfg(feature = "CVBuffer")]
    pub fn CVImageBufferIsFlipped(image_buffer: CVImageBufferRef) -> Boolean;
}

extern "C-unwind" {
    /// Returns the color space of a CVImageBuffer.
    ///
    /// Parameter `imageBuffer`: A CVImageBuffer that you wish to retrieve the color space from.
    ///
    /// Returns: A CGColorSpaceRef representing the color space of the buffer.
    /// Returns NULL if called with a non-CVImageBufferRef type or NULL.
    #[cfg(all(feature = "CVBuffer", feature = "objc2-core-graphics"))]
    pub fn CVImageBufferGetColorSpace(image_buffer: CVImageBufferRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    /// Attempts to synthesize a CGColorSpace from an image buffer's attachments.
    ///
    /// Parameter `attachments`: A CFDictionary of attachments for an image buffer, obtained using CVBufferCopyAttachments().
    ///
    /// Returns: A CGColorSpaceRef representing the color space of the buffer.
    /// Returns NULL if the attachments dictionary does not contain the information required to synthesize a CGColorSpace.
    ///
    /// To generate a CGColorSpace, the attachments dictionary should include values for either:
    /// 1. kCVImageBufferICCProfile
    /// 2. kCVImageBufferColorPrimariesKey, kCVImageBufferTransferFunctionKey, and kCVImageBufferYCbCrMatrixKey (and possibly kCVImageBufferGammaLevelKey)
    /// The client is responsible for releasing the CGColorSpaceRef when it is done with it (CGColorSpaceRelease() or CFRelease())
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
    pub fn CVImageBufferCreateColorSpaceFromAttachments(
        attachments: CFDictionaryRef,
    ) -> CGColorSpaceRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffermasteringdisplaycolorvolumekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferMasteringDisplayColorVolumeKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffercontentlightlevelinfokey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferContentLightLevelInfoKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferambientviewingenvironmentkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferAmbientViewingEnvironmentKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebuffersceneilluminationkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferSceneIlluminationKey: CFStringRef;
}

extern "C" {
    /// Specifies region of interest that image statistics cover.
    ///
    /// This value should be a CGRect dictionary created by CGRectCreateDictionaryRepresentation(). The origin in the CGRect represents the x,y coordinate within the CVPixelBuffer where region of interest is located.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferregionofinterestkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferRegionOfInterestKey: CFStringRef;
}

extern "C" {
    /// Indicates that the transfer function or gamma of the content is a log format and identifies the specific log curve.
    ///
    /// The value is a CFString holding fully specified reverse DNS identifier.
    /// Content captured in Apple Log will have this key set to kCVImageBufferLogTransferFunction_AppleLog.
    ///
    /// Indicates the Apple Log identifier.
    ///
    /// You can download the Apple Log Profile White Paper from the Apple Developer Downloads website.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferlogtransferfunctionkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferLogTransferFunctionKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvimagebufferlogtransferfunction_applelog?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVImageBufferLogTransferFunction_AppleLog: CFStringRef;
}
