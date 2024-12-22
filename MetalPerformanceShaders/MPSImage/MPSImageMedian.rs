//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// The MPSImageMedian applies a median filter to an image.  A median filter finds the
    /// median color value for each channel within a kernelDiameter x kernelDiameter
    /// window surrounding the pixel of interest.  It is a common means of noise reduction
    /// and also as a smoothing filter with edge preserving qualities.
    ///
    /// NOTE: The MPSImageMedian filter currently only supports images with
    /// <
    /// = 8 bits/channel.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagemedian?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageMedian;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageMedian {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageMedian {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageMedian {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageMedian {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageMedian {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageMedian {
        /// The diameter in pixels of the filter window.
        ///
        /// The median filter is applied to a kernelDiameter x kernelDiameter window
        /// of pixels centered on the corresponding source pixel for each destination
        /// pixel.  The kernel diameter must be an odd number.
        #[method(kernelDiameter)]
        pub unsafe fn kernelDiameter(&self) -> NSUInteger;

        /// Initialize a filter for a particular kernel size and device
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `kernelDiameter`: Diameter of the median filter. Must be an odd number.
        ///
        /// Returns: A valid object or nil, if failure.
        #[method_id(@__retain_semantics Init initWithDevice:kernelDiameter:)]
        pub unsafe fn initWithDevice_kernelDiameter(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_diameter: NSUInteger,
        ) -> Retained<Self>;

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// The maximum diameter in pixels of the filter window supported by the median filter.
        #[method(maxKernelDiameter)]
        pub unsafe fn maxKernelDiameter() -> NSUInteger;

        /// The minimum diameter in pixels of the filter window supported by the median filter.
        #[method(minKernelDiameter)]
        pub unsafe fn minKernelDiameter() -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageMedian {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageMedian {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
