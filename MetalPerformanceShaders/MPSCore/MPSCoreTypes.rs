//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpskerneloptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSKernelOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MPSKernelOptions: NSUInteger {
/// Use default options
        #[doc(alias = "MPSKernelOptionsNone")]
        const None = 0;
/// Most MPS functions will sanity check their arguments. This has a small but
/// non-zero CPU cost. Setting the MPSKernelOptionsSkipAPIValidation will skip these checks.
/// MPSKernelOptionsSkipAPIValidation does not skip checks for memory allocation failure.
/// Caution:  turning on MPSKernelOptionsSkipAPIValidation can result in undefined behavior
/// if the requested operation can not be completed for some reason. Most error states
/// will be passed through to Metal which may do nothing or abort the program if Metal
/// API validation is turned on.
        #[doc(alias = "MPSKernelOptionsSkipAPIValidation")]
        const SkipAPIValidation = 1<<0;
/// When possible, MPSKernels use a higher precision data representation internally than
/// the destination storage format to avoid excessive accumulation of computational
/// rounding error in the result. MPSKernelOptionsAllowReducedPrecision advises the
/// MPSKernel that the destination storage format already has too much precision for
/// what is ultimately required downstream, and the MPSKernel may use reduced precision
/// internally when it feels that a less precise result would yield better performance.
/// The expected performance win is often small, perhaps 0-20%. When enabled, the
/// precision of the result may vary by hardware and operating system.
        #[doc(alias = "MPSKernelOptionsAllowReducedPrecision")]
        const AllowReducedPrecision = 1<<1;
/// Some MPSKernels may automatically split up the work internally into multiple tiles.
/// This improves performance on larger textures and reduces the amount of memory needed by
/// MPS for temporary storage. However, if you are using your own tiling scheme to achieve
/// similar results, your tile sizes and MPS's choice of tile sizes may interfere with
/// one another causing MPS to subdivide your tiles for its own use inefficiently. Pass
/// MPSKernelOptionsDisableInternalTiling to force MPS to process your data tile as a
/// single chunk.
        #[doc(alias = "MPSKernelOptionsDisableInternalTiling")]
        const DisableInternalTiling = 1<<2;
/// Enabling this bit will cause various -encode... methods to call MTLCommandEncoder
/// push/popDebugGroup.  The debug string will be drawn from MPSKernel.label, if any
/// or the name of the class otherwise.
        #[doc(alias = "MPSKernelOptionsInsertDebugGroups")]
        const InsertDebugGroups = 1<<3;
/// Some parts of MPS can provide debug commentary and tuning advice when run.
/// Setting this bit to 1 will cause the commentary to be emitted to stderr. Otherwise,
/// the code is silent.  This is especially useful for debugging MPSNNGraph. This option
/// is on by default when the MPS_LOG_INFO environment variable is defined.  For
/// even more detailed output on a MPS object, you can use the po command in llvm
/// with MPS objects:
///
/// ```text
///     llvm>  po  <MPS object pointer>
/// ```
        #[doc(alias = "MPSKernelOptionsVerbose")]
        const Verbose = 1<<4;
    }
}

unsafe impl Encode for MPSKernelOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSKernelOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageedgemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSImageEdgeMode(pub NSUInteger);
impl MPSImageEdgeMode {
    /// Out of bound pixels are (0,0,0,1) for image with pixel format without alpha channel
    /// and (0,0,0,0) for image with pixel format that has an alpha channel
    #[doc(alias = "MPSImageEdgeModeZero")]
    pub const Zero: Self = Self(0);
    /// Out of bound pixels are clamped to nearest edge pixel
    #[doc(alias = "MPSImageEdgeModeClamp")]
    pub const Clamp: Self = Self(1);
    /// Out of bound pixels are mirrored wrt. the nearest edge pixel center - ie. the edge of the image is not repeated.
    /// NOTE: The only filter that currently supports this mode is
    /// MPSNNPad- using this with other filters results in undefined behavior.
    #[doc(alias = "MPSImageEdgeModeMirror")]
    pub const Mirror: Self = Self(2);
    /// Out of bound pixels are mirrored wrt. the nearest edge pixel nearest border - ie. the edge of the image is repeated.
    /// NOTE: The only filter that currently supports this mode is
    /// MPSNNPad- using this with other filters results in undefined behavior.
    #[doc(alias = "MPSImageEdgeModeMirrorWithEdge")]
    pub const MirrorWithEdge: Self = Self(3);
    /// Out of bound pixels are filled with a constant value defined by the filter.
    /// NOTE: The only filter that currently supports this mode is
    /// MPSNNPad- using this with other filters results in undefined behavior.
    #[doc(alias = "MPSImageEdgeModeConstant")]
    pub const Constant: Self = Self(4);
}

unsafe impl Encode for MPSImageEdgeMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSImageEdgeMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagefeaturechannelformat?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSImageFeatureChannelFormat(pub NSUInteger);
impl MPSImageFeatureChannelFormat {
    /// No format. This can mean  according to context invalid format or any format.  In the
    /// latter case, it is an invitation to MPS to pick a format.
    #[doc(alias = "MPSImageFeatureChannelFormatNone")]
    pub const None: Self = Self(0);
    /// uint8_t with value [0,255] encoding [0,1.0]
    #[doc(alias = "MPSImageFeatureChannelFormatUnorm8")]
    pub const Unorm8: Self = Self(1);
    /// uint16_t with value [0,65535] encoding [0,1.0]
    #[doc(alias = "MPSImageFeatureChannelFormatUnorm16")]
    pub const Unorm16: Self = Self(2);
    /// IEEE-754 16-bit floating-point value. "half precision" Representable normal range is +-[2**-14, 65504], 0, Infinity, NaN. 11 bits of precision + exponent.
    #[doc(alias = "MPSImageFeatureChannelFormatFloat16")]
    pub const Float16: Self = Self(3);
    /// IEEE-754 32-bit floating-point value.  "single precision" (standard float type in C) 24 bits of precision + exponent
    #[doc(alias = "MPSImageFeatureChannelFormatFloat32")]
    pub const Float32: Self = Self(4);
    /// Reserved for later expansion
    #[doc(alias = "MPSImageFeatureChannelFormat_reserved0")]
    pub const _reserved0: Self = Self(5);
    /// Reserved for later expansion
    #[doc(alias = "MPSImageFeatureChannelFormatCount")]
    pub const Count: Self = Self(6);
}

unsafe impl Encode for MPSImageFeatureChannelFormat {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSImageFeatureChannelFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsfloatdatatypebit?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSFloatDataTypeBit(pub u32);
impl MPSFloatDataTypeBit {
    pub const MPSFloatDataTypeSignBit: Self = Self(0x00800000);
    pub const MPSFloatDataTypeExponentBit: Self = Self(0x007C0000);
    pub const MPSFloatDataTypeMantissaBit: Self = Self(0x0003FC00);
}

unsafe impl Encode for MPSFloatDataTypeBit {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MPSFloatDataTypeBit {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsfloatdatatypeshift?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSFloatDataTypeShift(pub u32);
impl MPSFloatDataTypeShift {
    pub const MPSFloatDataTypeSignShift: Self = Self(23);
    pub const MPSFloatDataTypeExponentShift: Self = Self(18);
    pub const MPSFloatDataTypeMantissaShift: Self = Self(10);
}

unsafe impl Encode for MPSFloatDataTypeShift {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MPSFloatDataTypeShift {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsdatatype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSDataType(pub u32);
impl MPSDataType {
    #[doc(alias = "MPSDataTypeInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MPSDataTypeFloatBit")]
    pub const FloatBit: Self = Self(0x10000000);
    #[doc(alias = "MPSDataTypeFloat32")]
    pub const Float32: Self = Self(MPSDataType::FloatBit.0 | 32);
    #[doc(alias = "MPSDataTypeFloat16")]
    pub const Float16: Self = Self(MPSDataType::FloatBit.0 | 16);
    #[doc(alias = "MPSDataTypeComplexBit")]
    pub const ComplexBit: Self = Self(0x01000000);
    #[doc(alias = "MPSDataTypeComplexFloat32")]
    pub const ComplexFloat32: Self = Self(MPSDataType::FloatBit.0 | MPSDataType::ComplexBit.0 | 64);
    #[doc(alias = "MPSDataTypeComplexFloat16")]
    pub const ComplexFloat16: Self = Self(MPSDataType::FloatBit.0 | MPSDataType::ComplexBit.0 | 32);
    #[doc(alias = "MPSDataTypeSignedBit")]
    pub const SignedBit: Self = Self(0x20000000);
    #[doc(alias = "MPSDataTypeIntBit")]
    pub const IntBit: Self = Self(MPSDataType::SignedBit.0);
    #[doc(alias = "MPSDataTypeInt4")]
    pub const Int4: Self = Self(MPSDataType::SignedBit.0 | 4);
    #[doc(alias = "MPSDataTypeInt8")]
    pub const Int8: Self = Self(MPSDataType::SignedBit.0 | 8);
    #[doc(alias = "MPSDataTypeInt16")]
    pub const Int16: Self = Self(MPSDataType::SignedBit.0 | 16);
    #[doc(alias = "MPSDataTypeInt32")]
    pub const Int32: Self = Self(MPSDataType::SignedBit.0 | 32);
    #[doc(alias = "MPSDataTypeInt64")]
    pub const Int64: Self = Self(MPSDataType::SignedBit.0 | 64);
    #[doc(alias = "MPSDataTypeUInt4")]
    pub const UInt4: Self = Self(4);
    #[doc(alias = "MPSDataTypeUInt8")]
    pub const UInt8: Self = Self(8);
    #[doc(alias = "MPSDataTypeUInt16")]
    pub const UInt16: Self = Self(16);
    #[doc(alias = "MPSDataTypeUInt32")]
    pub const UInt32: Self = Self(32);
    #[doc(alias = "MPSDataTypeUInt64")]
    pub const UInt64: Self = Self(64);
    #[doc(alias = "MPSDataTypeAlternateEncodingBit")]
    pub const AlternateEncodingBit: Self = Self(0x80000000);
    #[doc(alias = "MPSDataTypeBool")]
    pub const Bool: Self = Self(MPSDataType::AlternateEncodingBit.0 | 8);
    #[doc(alias = "MPSDataTypeBFloat16")]
    pub const BFloat16: Self = Self(MPSDataType::AlternateEncodingBit.0 | MPSDataType::Float16.0);
    #[doc(alias = "MPSDataTypeNormalizedBit")]
    pub const NormalizedBit: Self = Self(0x40000000);
    #[doc(alias = "MPSDataTypeUnorm1")]
    pub const Unorm1: Self = Self(MPSDataType::NormalizedBit.0 | 1);
    #[doc(alias = "MPSDataTypeUnorm8")]
    pub const Unorm8: Self = Self(MPSDataType::NormalizedBit.0 | 8);
}

unsafe impl Encode for MPSDataType {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MPSDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsaliasingstrategy?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSAliasingStrategy(pub NSUInteger);
bitflags::bitflags! {
    impl MPSAliasingStrategy: NSUInteger {
        #[doc(alias = "MPSAliasingStrategyDefault")]
        const Default = 0;
        #[doc(alias = "MPSAliasingStrategyDontCare")]
        const DontCare = MPSAliasingStrategy::Default.0;
        #[doc(alias = "MPSAliasingStrategyShallAlias")]
        const ShallAlias = 1<<0;
        #[doc(alias = "MPSAliasingStrategyShallNotAlias")]
        const ShallNotAlias = 1<<1;
        #[doc(alias = "MPSAliasingStrategyAliasingReserved")]
        const AliasingReserved = MPSAliasingStrategy::ShallAlias.0|MPSAliasingStrategy::ShallNotAlias.0;
/// The view must alias the original.  Typical usage for views used for destination slicing.
        #[doc(alias = "MPSAliasingStrategyPreferTemporaryMemory")]
        const PreferTemporaryMemory = 1<<2;
/// The view must alias the original.  Typical usage for views used for destination slicing.
        #[doc(alias = "MPSAliasingStrategyPreferNonTemporaryMemory")]
        const PreferNonTemporaryMemory = 1<<3;
    }
}

unsafe impl Encode for MPSAliasingStrategy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSAliasingStrategy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A signed coordinate with x, y and z components
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsoffset?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSOffset {
    /// The horizontal component of the offset. Units: pixels
    pub x: NSInteger,
    /// The vertical component of the offset. Units: pixels
    pub y: NSInteger,
    /// The depth component of the offset. Units: pixels
    pub z: NSInteger,
}

unsafe impl Encode for MPSOffset {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <NSInteger>::ENCODING,
            <NSInteger>::ENCODING,
            <NSInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSOffset {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A position in an image
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsorigin?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSOrigin {
    /// The x coordinate of the position
    pub x: c_double,
    /// The y coordinate of the position
    pub y: c_double,
    /// The z coordinate of the position
    pub z: c_double,
}

unsafe impl Encode for MPSOrigin {
    const ENCODING: Encoding = Encoding::Struct(
        "MPSOrigin",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSOrigin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A size of a region in an image
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpssize?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSSize {
    /// The width of the region
    pub width: c_double,
    /// The height of the region
    pub height: c_double,
    /// The depth of the region
    pub depth: c_double,
}

unsafe impl Encode for MPSSize {
    const ENCODING: Encoding = Encoding::Struct(
        "MPSSize",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes a sub-region of an array dimension
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsdimensionslice?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSDimensionSlice {
    /// the position of the first element in the slice
    pub start: NSUInteger,
    /// the number of elements in the slice.
    pub length: NSUInteger,
}

unsafe impl Encode for MPSDimensionSlice {
    const ENCODING: Encoding = Encoding::Struct(
        "MPSDimensionSlice",
        &[<NSUInteger>::ENCODING, <NSUInteger>::ENCODING],
    );
}

unsafe impl RefEncode for MPSDimensionSlice {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A region of an image
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsregion?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSRegion {
    /// The top left corner of the region.  Units: pixels
    pub origin: MPSOrigin,
    /// The size of the region. Units: pixels
    pub size: MPSSize,
}

unsafe impl Encode for MPSRegion {
    const ENCODING: Encoding =
        Encoding::Struct("MPSRegion", &[<MPSOrigin>::ENCODING, <MPSSize>::ENCODING]);
}

unsafe impl RefEncode for MPSRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Transform matrix for explict control over resampling in MPSImageScale.
///
/// The MPSScaleTransform is equivalent to:
///
/// ```text
///           (CGAffineTransform) {
///                .a = scaleX,        .b = 0,
///                .c = 0,             .d = scaleY,
///                .tx = translateX,   .ty = translateY
///            }
/// ```
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsscaletransform?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSScaleTransform {
    /// horizontal scaling factor
    pub scaleX: c_double,
    /// vertical scaling factor
    pub scaleY: c_double,
    /// horizontal translation
    pub translateX: c_double,
    /// vertical translation
    pub translateY: c_double,
}

unsafe impl Encode for MPSScaleTransform {
    const ENCODING: Encoding = Encoding::Struct(
        "MPSScaleTransform",
        &[
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
            <c_double>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSScaleTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A unsigned coordinate with x, y and channel components
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagecoordinate?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSImageCoordinate {
    /// The horizontal component of the coordinate. Units: pixels
    pub x: NSUInteger,
    /// The vertical component of the coordinate. Units: pixels
    pub y: NSUInteger,
    /// The index of the channel or feature channel within the pixel
    pub channel: NSUInteger,
}

unsafe impl Encode for MPSImageCoordinate {
    const ENCODING: Encoding = Encoding::Struct(
        "MPSImageCoordinate",
        &[
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSImageCoordinate {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A rectangular subregion of a MPSImage
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageregion?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSImageRegion {
    /// The position of the top left corner of the subregion
    pub offset: MPSImageCoordinate,
    /// The size {pixels, pixels, channels} of the subregion
    pub size: MPSImageCoordinate,
}

unsafe impl Encode for MPSImageRegion {
    const ENCODING: Encoding = Encoding::Struct(
        "MPSImageRegion",
        &[
            <MPSImageCoordinate>::ENCODING,
            <MPSImageCoordinate>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSImageRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// This is a special constant to indicate no clipping is to be done.
    /// The entire image will be used.
    /// This is the default clipping rectangle or the input extent for MPSKernels.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsrectnoclip?language=objc)
    pub static MPSRectNoClip: MTLRegion;
}

extern_protocol!(
    /// A way of extending a NSCoder to enable the setting of MTLDevice for unarchived objects
    ///
    /// When a object is initialized by a NSCoder, it calls -initWithCoder:, which is
    /// missing the necessary MTLDevice to correctly initialize the MPSKernel, or MPSNNGraph.
    /// If the coder does not conform to MPSDeviceProvider, the system default device
    /// will be used.  If you would like to specify which device to use, subclass the
    /// NSCoder (NSKeyedUnarchiver, etc.) to conform to MPSDeviceProvider so that
    /// the device can be gotten from the NSCoder.
    ///
    /// See MPSKeyedUnarchiver for one implementation of this protocol. It reads files
    /// prepared with the NSKeyedArchiver and allows you to set the MTLDevice that the
    /// unarchived objects use.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsdeviceprovider?language=objc)
    pub unsafe trait MPSDeviceProvider {
        /// Return the device to use when making MPSKernel subclasses from the NSCoder
        #[method_id(@__retain_semantics Other mpsMTLDevice)]
        unsafe fn mpsMTLDevice(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;
    }

    unsafe impl ProtocolType for dyn MPSDeviceProvider {}
);

/// An array of NSNumbers where dimension lengths provided by the user goes from slowest moving to fastest moving dimension.
/// This is same order as MLMultiArray in coreML and most frameworks in Python.
///
/// ```text
///   A shape @[5, 4, 2] would mean fastest moving 0th dimension is one with size 2,
///   1st dimension is size 4 finally slowest moving 2nd dimension is size 5.
/// ```
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsshape?language=objc)
pub type MPSShape = NSArray<NSNumber>;

// TODO: pub fn MPSDataTypeBitsCount(t: MPSDataType,) -> usize;

// TODO: pub fn MPSSizeofMPSDataType(t: MPSDataType,) -> usize;
