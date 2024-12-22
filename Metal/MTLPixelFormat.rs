//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlpixelformat?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLPixelFormat(pub NSUInteger);
impl MTLPixelFormat {
    #[doc(alias = "MTLPixelFormatInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MTLPixelFormatA8Unorm")]
    pub const A8Unorm: Self = Self(1);
    #[doc(alias = "MTLPixelFormatR8Unorm")]
    pub const R8Unorm: Self = Self(10);
    #[doc(alias = "MTLPixelFormatR8Unorm_sRGB")]
    pub const R8Unorm_sRGB: Self = Self(11);
    #[doc(alias = "MTLPixelFormatR8Snorm")]
    pub const R8Snorm: Self = Self(12);
    #[doc(alias = "MTLPixelFormatR8Uint")]
    pub const R8Uint: Self = Self(13);
    #[doc(alias = "MTLPixelFormatR8Sint")]
    pub const R8Sint: Self = Self(14);
    #[doc(alias = "MTLPixelFormatR16Unorm")]
    pub const R16Unorm: Self = Self(20);
    #[doc(alias = "MTLPixelFormatR16Snorm")]
    pub const R16Snorm: Self = Self(22);
    #[doc(alias = "MTLPixelFormatR16Uint")]
    pub const R16Uint: Self = Self(23);
    #[doc(alias = "MTLPixelFormatR16Sint")]
    pub const R16Sint: Self = Self(24);
    #[doc(alias = "MTLPixelFormatR16Float")]
    pub const R16Float: Self = Self(25);
    #[doc(alias = "MTLPixelFormatRG8Unorm")]
    pub const RG8Unorm: Self = Self(30);
    #[doc(alias = "MTLPixelFormatRG8Unorm_sRGB")]
    pub const RG8Unorm_sRGB: Self = Self(31);
    #[doc(alias = "MTLPixelFormatRG8Snorm")]
    pub const RG8Snorm: Self = Self(32);
    #[doc(alias = "MTLPixelFormatRG8Uint")]
    pub const RG8Uint: Self = Self(33);
    #[doc(alias = "MTLPixelFormatRG8Sint")]
    pub const RG8Sint: Self = Self(34);
    #[doc(alias = "MTLPixelFormatB5G6R5Unorm")]
    pub const B5G6R5Unorm: Self = Self(40);
    #[doc(alias = "MTLPixelFormatA1BGR5Unorm")]
    pub const A1BGR5Unorm: Self = Self(41);
    #[doc(alias = "MTLPixelFormatABGR4Unorm")]
    pub const ABGR4Unorm: Self = Self(42);
    #[doc(alias = "MTLPixelFormatBGR5A1Unorm")]
    pub const BGR5A1Unorm: Self = Self(43);
    #[doc(alias = "MTLPixelFormatR32Uint")]
    pub const R32Uint: Self = Self(53);
    #[doc(alias = "MTLPixelFormatR32Sint")]
    pub const R32Sint: Self = Self(54);
    #[doc(alias = "MTLPixelFormatR32Float")]
    pub const R32Float: Self = Self(55);
    #[doc(alias = "MTLPixelFormatRG16Unorm")]
    pub const RG16Unorm: Self = Self(60);
    #[doc(alias = "MTLPixelFormatRG16Snorm")]
    pub const RG16Snorm: Self = Self(62);
    #[doc(alias = "MTLPixelFormatRG16Uint")]
    pub const RG16Uint: Self = Self(63);
    #[doc(alias = "MTLPixelFormatRG16Sint")]
    pub const RG16Sint: Self = Self(64);
    #[doc(alias = "MTLPixelFormatRG16Float")]
    pub const RG16Float: Self = Self(65);
    #[doc(alias = "MTLPixelFormatRGBA8Unorm")]
    pub const RGBA8Unorm: Self = Self(70);
    #[doc(alias = "MTLPixelFormatRGBA8Unorm_sRGB")]
    pub const RGBA8Unorm_sRGB: Self = Self(71);
    #[doc(alias = "MTLPixelFormatRGBA8Snorm")]
    pub const RGBA8Snorm: Self = Self(72);
    #[doc(alias = "MTLPixelFormatRGBA8Uint")]
    pub const RGBA8Uint: Self = Self(73);
    #[doc(alias = "MTLPixelFormatRGBA8Sint")]
    pub const RGBA8Sint: Self = Self(74);
    #[doc(alias = "MTLPixelFormatBGRA8Unorm")]
    pub const BGRA8Unorm: Self = Self(80);
    #[doc(alias = "MTLPixelFormatBGRA8Unorm_sRGB")]
    pub const BGRA8Unorm_sRGB: Self = Self(81);
    #[doc(alias = "MTLPixelFormatRGB10A2Unorm")]
    pub const RGB10A2Unorm: Self = Self(90);
    #[doc(alias = "MTLPixelFormatRGB10A2Uint")]
    pub const RGB10A2Uint: Self = Self(91);
    #[doc(alias = "MTLPixelFormatRG11B10Float")]
    pub const RG11B10Float: Self = Self(92);
    #[doc(alias = "MTLPixelFormatRGB9E5Float")]
    pub const RGB9E5Float: Self = Self(93);
    #[doc(alias = "MTLPixelFormatBGR10A2Unorm")]
    pub const BGR10A2Unorm: Self = Self(94);
    #[doc(alias = "MTLPixelFormatBGR10_XR")]
    pub const BGR10_XR: Self = Self(554);
    #[doc(alias = "MTLPixelFormatBGR10_XR_sRGB")]
    pub const BGR10_XR_sRGB: Self = Self(555);
    #[doc(alias = "MTLPixelFormatRG32Uint")]
    pub const RG32Uint: Self = Self(103);
    #[doc(alias = "MTLPixelFormatRG32Sint")]
    pub const RG32Sint: Self = Self(104);
    #[doc(alias = "MTLPixelFormatRG32Float")]
    pub const RG32Float: Self = Self(105);
    #[doc(alias = "MTLPixelFormatRGBA16Unorm")]
    pub const RGBA16Unorm: Self = Self(110);
    #[doc(alias = "MTLPixelFormatRGBA16Snorm")]
    pub const RGBA16Snorm: Self = Self(112);
    #[doc(alias = "MTLPixelFormatRGBA16Uint")]
    pub const RGBA16Uint: Self = Self(113);
    #[doc(alias = "MTLPixelFormatRGBA16Sint")]
    pub const RGBA16Sint: Self = Self(114);
    #[doc(alias = "MTLPixelFormatRGBA16Float")]
    pub const RGBA16Float: Self = Self(115);
    #[doc(alias = "MTLPixelFormatBGRA10_XR")]
    pub const BGRA10_XR: Self = Self(552);
    #[doc(alias = "MTLPixelFormatBGRA10_XR_sRGB")]
    pub const BGRA10_XR_sRGB: Self = Self(553);
    #[doc(alias = "MTLPixelFormatRGBA32Uint")]
    pub const RGBA32Uint: Self = Self(123);
    #[doc(alias = "MTLPixelFormatRGBA32Sint")]
    pub const RGBA32Sint: Self = Self(124);
    #[doc(alias = "MTLPixelFormatRGBA32Float")]
    pub const RGBA32Float: Self = Self(125);
    #[doc(alias = "MTLPixelFormatBC1_RGBA")]
    pub const BC1_RGBA: Self = Self(130);
    #[doc(alias = "MTLPixelFormatBC1_RGBA_sRGB")]
    pub const BC1_RGBA_sRGB: Self = Self(131);
    #[doc(alias = "MTLPixelFormatBC2_RGBA")]
    pub const BC2_RGBA: Self = Self(132);
    #[doc(alias = "MTLPixelFormatBC2_RGBA_sRGB")]
    pub const BC2_RGBA_sRGB: Self = Self(133);
    #[doc(alias = "MTLPixelFormatBC3_RGBA")]
    pub const BC3_RGBA: Self = Self(134);
    #[doc(alias = "MTLPixelFormatBC3_RGBA_sRGB")]
    pub const BC3_RGBA_sRGB: Self = Self(135);
    #[doc(alias = "MTLPixelFormatBC4_RUnorm")]
    pub const BC4_RUnorm: Self = Self(140);
    #[doc(alias = "MTLPixelFormatBC4_RSnorm")]
    pub const BC4_RSnorm: Self = Self(141);
    #[doc(alias = "MTLPixelFormatBC5_RGUnorm")]
    pub const BC5_RGUnorm: Self = Self(142);
    #[doc(alias = "MTLPixelFormatBC5_RGSnorm")]
    pub const BC5_RGSnorm: Self = Self(143);
    #[doc(alias = "MTLPixelFormatBC6H_RGBFloat")]
    pub const BC6H_RGBFloat: Self = Self(150);
    #[doc(alias = "MTLPixelFormatBC6H_RGBUfloat")]
    pub const BC6H_RGBUfloat: Self = Self(151);
    #[doc(alias = "MTLPixelFormatBC7_RGBAUnorm")]
    pub const BC7_RGBAUnorm: Self = Self(152);
    #[doc(alias = "MTLPixelFormatBC7_RGBAUnorm_sRGB")]
    pub const BC7_RGBAUnorm_sRGB: Self = Self(153);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGB_2BPP")]
    pub const PVRTC_RGB_2BPP: Self = Self(160);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGB_2BPP_sRGB")]
    pub const PVRTC_RGB_2BPP_sRGB: Self = Self(161);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGB_4BPP")]
    pub const PVRTC_RGB_4BPP: Self = Self(162);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGB_4BPP_sRGB")]
    pub const PVRTC_RGB_4BPP_sRGB: Self = Self(163);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGBA_2BPP")]
    pub const PVRTC_RGBA_2BPP: Self = Self(164);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGBA_2BPP_sRGB")]
    pub const PVRTC_RGBA_2BPP_sRGB: Self = Self(165);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGBA_4BPP")]
    pub const PVRTC_RGBA_4BPP: Self = Self(166);
    #[deprecated = "Usage of ASTC/ETC2/BC formats is recommended instead."]
    #[doc(alias = "MTLPixelFormatPVRTC_RGBA_4BPP_sRGB")]
    pub const PVRTC_RGBA_4BPP_sRGB: Self = Self(167);
    #[doc(alias = "MTLPixelFormatEAC_R11Unorm")]
    pub const EAC_R11Unorm: Self = Self(170);
    #[doc(alias = "MTLPixelFormatEAC_R11Snorm")]
    pub const EAC_R11Snorm: Self = Self(172);
    #[doc(alias = "MTLPixelFormatEAC_RG11Unorm")]
    pub const EAC_RG11Unorm: Self = Self(174);
    #[doc(alias = "MTLPixelFormatEAC_RG11Snorm")]
    pub const EAC_RG11Snorm: Self = Self(176);
    #[doc(alias = "MTLPixelFormatEAC_RGBA8")]
    pub const EAC_RGBA8: Self = Self(178);
    #[doc(alias = "MTLPixelFormatEAC_RGBA8_sRGB")]
    pub const EAC_RGBA8_sRGB: Self = Self(179);
    #[doc(alias = "MTLPixelFormatETC2_RGB8")]
    pub const ETC2_RGB8: Self = Self(180);
    #[doc(alias = "MTLPixelFormatETC2_RGB8_sRGB")]
    pub const ETC2_RGB8_sRGB: Self = Self(181);
    #[doc(alias = "MTLPixelFormatETC2_RGB8A1")]
    pub const ETC2_RGB8A1: Self = Self(182);
    #[doc(alias = "MTLPixelFormatETC2_RGB8A1_sRGB")]
    pub const ETC2_RGB8A1_sRGB: Self = Self(183);
    #[doc(alias = "MTLPixelFormatASTC_4x4_sRGB")]
    pub const ASTC_4x4_sRGB: Self = Self(186);
    #[doc(alias = "MTLPixelFormatASTC_5x4_sRGB")]
    pub const ASTC_5x4_sRGB: Self = Self(187);
    #[doc(alias = "MTLPixelFormatASTC_5x5_sRGB")]
    pub const ASTC_5x5_sRGB: Self = Self(188);
    #[doc(alias = "MTLPixelFormatASTC_6x5_sRGB")]
    pub const ASTC_6x5_sRGB: Self = Self(189);
    #[doc(alias = "MTLPixelFormatASTC_6x6_sRGB")]
    pub const ASTC_6x6_sRGB: Self = Self(190);
    #[doc(alias = "MTLPixelFormatASTC_8x5_sRGB")]
    pub const ASTC_8x5_sRGB: Self = Self(192);
    #[doc(alias = "MTLPixelFormatASTC_8x6_sRGB")]
    pub const ASTC_8x6_sRGB: Self = Self(193);
    #[doc(alias = "MTLPixelFormatASTC_8x8_sRGB")]
    pub const ASTC_8x8_sRGB: Self = Self(194);
    #[doc(alias = "MTLPixelFormatASTC_10x5_sRGB")]
    pub const ASTC_10x5_sRGB: Self = Self(195);
    #[doc(alias = "MTLPixelFormatASTC_10x6_sRGB")]
    pub const ASTC_10x6_sRGB: Self = Self(196);
    #[doc(alias = "MTLPixelFormatASTC_10x8_sRGB")]
    pub const ASTC_10x8_sRGB: Self = Self(197);
    #[doc(alias = "MTLPixelFormatASTC_10x10_sRGB")]
    pub const ASTC_10x10_sRGB: Self = Self(198);
    #[doc(alias = "MTLPixelFormatASTC_12x10_sRGB")]
    pub const ASTC_12x10_sRGB: Self = Self(199);
    #[doc(alias = "MTLPixelFormatASTC_12x12_sRGB")]
    pub const ASTC_12x12_sRGB: Self = Self(200);
    #[doc(alias = "MTLPixelFormatASTC_4x4_LDR")]
    pub const ASTC_4x4_LDR: Self = Self(204);
    #[doc(alias = "MTLPixelFormatASTC_5x4_LDR")]
    pub const ASTC_5x4_LDR: Self = Self(205);
    #[doc(alias = "MTLPixelFormatASTC_5x5_LDR")]
    pub const ASTC_5x5_LDR: Self = Self(206);
    #[doc(alias = "MTLPixelFormatASTC_6x5_LDR")]
    pub const ASTC_6x5_LDR: Self = Self(207);
    #[doc(alias = "MTLPixelFormatASTC_6x6_LDR")]
    pub const ASTC_6x6_LDR: Self = Self(208);
    #[doc(alias = "MTLPixelFormatASTC_8x5_LDR")]
    pub const ASTC_8x5_LDR: Self = Self(210);
    #[doc(alias = "MTLPixelFormatASTC_8x6_LDR")]
    pub const ASTC_8x6_LDR: Self = Self(211);
    #[doc(alias = "MTLPixelFormatASTC_8x8_LDR")]
    pub const ASTC_8x8_LDR: Self = Self(212);
    #[doc(alias = "MTLPixelFormatASTC_10x5_LDR")]
    pub const ASTC_10x5_LDR: Self = Self(213);
    #[doc(alias = "MTLPixelFormatASTC_10x6_LDR")]
    pub const ASTC_10x6_LDR: Self = Self(214);
    #[doc(alias = "MTLPixelFormatASTC_10x8_LDR")]
    pub const ASTC_10x8_LDR: Self = Self(215);
    #[doc(alias = "MTLPixelFormatASTC_10x10_LDR")]
    pub const ASTC_10x10_LDR: Self = Self(216);
    #[doc(alias = "MTLPixelFormatASTC_12x10_LDR")]
    pub const ASTC_12x10_LDR: Self = Self(217);
    #[doc(alias = "MTLPixelFormatASTC_12x12_LDR")]
    pub const ASTC_12x12_LDR: Self = Self(218);
    #[doc(alias = "MTLPixelFormatASTC_4x4_HDR")]
    pub const ASTC_4x4_HDR: Self = Self(222);
    #[doc(alias = "MTLPixelFormatASTC_5x4_HDR")]
    pub const ASTC_5x4_HDR: Self = Self(223);
    #[doc(alias = "MTLPixelFormatASTC_5x5_HDR")]
    pub const ASTC_5x5_HDR: Self = Self(224);
    #[doc(alias = "MTLPixelFormatASTC_6x5_HDR")]
    pub const ASTC_6x5_HDR: Self = Self(225);
    #[doc(alias = "MTLPixelFormatASTC_6x6_HDR")]
    pub const ASTC_6x6_HDR: Self = Self(226);
    #[doc(alias = "MTLPixelFormatASTC_8x5_HDR")]
    pub const ASTC_8x5_HDR: Self = Self(228);
    #[doc(alias = "MTLPixelFormatASTC_8x6_HDR")]
    pub const ASTC_8x6_HDR: Self = Self(229);
    #[doc(alias = "MTLPixelFormatASTC_8x8_HDR")]
    pub const ASTC_8x8_HDR: Self = Self(230);
    #[doc(alias = "MTLPixelFormatASTC_10x5_HDR")]
    pub const ASTC_10x5_HDR: Self = Self(231);
    #[doc(alias = "MTLPixelFormatASTC_10x6_HDR")]
    pub const ASTC_10x6_HDR: Self = Self(232);
    #[doc(alias = "MTLPixelFormatASTC_10x8_HDR")]
    pub const ASTC_10x8_HDR: Self = Self(233);
    #[doc(alias = "MTLPixelFormatASTC_10x10_HDR")]
    pub const ASTC_10x10_HDR: Self = Self(234);
    #[doc(alias = "MTLPixelFormatASTC_12x10_HDR")]
    pub const ASTC_12x10_HDR: Self = Self(235);
    #[doc(alias = "MTLPixelFormatASTC_12x12_HDR")]
    pub const ASTC_12x12_HDR: Self = Self(236);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to YUY2, YUYV, yuvs, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_REV_APPLE.   The component order, from lowest addressed byte to highest, is Y0, Cb, Y1, Cr.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatGBGR422")]
    pub const GBGR422: Self = Self(240);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatBGRG422")]
    pub const BGRG422: Self = Self(241);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatDepth16Unorm")]
    pub const Depth16Unorm: Self = Self(250);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatDepth32Float")]
    pub const Depth32Float: Self = Self(252);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatStencil8")]
    pub const Stencil8: Self = Self(253);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatDepth24Unorm_Stencil8")]
    pub const Depth24Unorm_Stencil8: Self = Self(255);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatDepth32Float_Stencil8")]
    pub const Depth32Float_Stencil8: Self = Self(260);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatX32_Stencil8")]
    pub const X32_Stencil8: Self = Self(261);
    /// A pixel format where the red and green channels are subsampled horizontally.  Two pixels are stored in 32 bits, with shared red and blue values, and unique green values.
    ///
    /// This format is equivalent to UYVY, 2vuy, or GL_RGB_422_APPLE/GL_UNSIGNED_SHORT_8_8_APPLE. The component order, from lowest addressed byte to highest, is Cb, Y0, Cr, Y1.  There is no implicit colorspace conversion from YUV to RGB, the shader will receive (Cr, Y, Cb, 1).  422 textures must have a width that is a multiple of 2, and can only be used for 2D non-mipmap textures.  When sampling, ClampToEdge is the only usable wrap mode.
    #[doc(alias = "MTLPixelFormatX24_Stencil8")]
    pub const X24_Stencil8: Self = Self(262);
}

unsafe impl Encode for MTLPixelFormat {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLPixelFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
