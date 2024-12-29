//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "objc2")]
extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icscannerbanddata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct ICScannerBandData;
);

#[cfg(feature = "objc2")]
unsafe impl NSObjectProtocol for ICScannerBandData {}

#[cfg(feature = "objc2")]
extern_methods!(
    #[cfg(feature = "objc2")]
    unsafe impl ICScannerBandData {
        /// Describes the full image width of the banded image.
        #[method(fullImageWidth)]
        pub unsafe fn fullImageWidth(&self) -> NSUInteger;

        /// Describes the full image height of the banded image.
        #[method(fullImageHeight)]
        pub unsafe fn fullImageHeight(&self) -> NSUInteger;

        /// Describes the number of bits per pixel for banded the image.
        #[method(bitsPerPixel)]
        pub unsafe fn bitsPerPixel(&self) -> NSUInteger;

        /// Describes the number of bits per component for the banded image.
        #[method(bitsPerComponent)]
        pub unsafe fn bitsPerComponent(&self) -> NSUInteger;

        /// Describes how many components are contained within the banded image.
        #[method(numComponents)]
        pub unsafe fn numComponents(&self) -> NSUInteger;

        /// Describes if the banded image data is reported in big endian.
        #[method(isBigEndian)]
        pub unsafe fn isBigEndian(&self) -> bool;

        #[cfg(feature = "ICScannerFunctionalUnits")]
        /// Type of pixel data that is contained in the band.
        #[method(pixelDataType)]
        pub unsafe fn pixelDataType(&self) -> ICScannerPixelDataType;

        /// Returns the path to the color profile matching the banded data.
        #[method_id(@__retain_semantics Other colorSyncProfilePath)]
        pub unsafe fn colorSyncProfilePath(&self) -> Option<Retained<NSString>>;

        /// Descries how many bytes are in each image band row.
        #[method(bytesPerRow)]
        pub unsafe fn bytesPerRow(&self) -> NSUInteger;

        /// Describes the start row of the image band.
        #[method(dataStartRow)]
        pub unsafe fn dataStartRow(&self) -> NSUInteger;

        /// Describes the number of rows contained in the image band.
        #[method(dataNumRows)]
        pub unsafe fn dataNumRows(&self) -> NSUInteger;

        /// Describes the actual data size of the image band buffer.
        #[method(dataSize)]
        pub unsafe fn dataSize(&self) -> NSUInteger;

        /// The pointer to the data buffer object.
        #[method_id(@__retain_semantics Other dataBuffer)]
        pub unsafe fn dataBuffer(&self) -> Option<Retained<NSData>>;
    }
);

#[cfg(feature = "objc2")]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2")]
    unsafe impl ICScannerBandData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);