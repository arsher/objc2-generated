//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// The MPSThreshold filter applies a fixed-level threshold to each pixel in the image.
    /// The threshold functions convert a single channel image to a binary image.
    /// If the input image is not a single channel image, convert the inputimage to a single channel
    /// luminance image using the linearGrayColorTransform and then apply the threshold.
    /// The ThresholdBinary function is:
    /// destinationPixelValue = sourcePixelValue > thresholdValue ? maximumValue : 0
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagethresholdbinary?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageThresholdBinary;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageThresholdBinary {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageThresholdBinary {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageThresholdBinary {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageThresholdBinary {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageThresholdBinary {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdBinary {
        /// initialize a MPSImageThresholdBinary filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `thresholdValue`: The threshold value to use
        ///
        /// Parameter `maximumValue`: The maximum value to use
        ///
        /// Parameter `transform`: This matrix is an array of 3 floats.
        /// The default if no transform is specifed is BT.601/JPEG: {0.299f, 0.587f, 0.114f};
        #[method_id(@__retain_semantics Init initWithDevice:thresholdValue:maximumValue:linearGrayColorTransform:)]
        pub unsafe fn initWithDevice_thresholdValue_maximumValue_linearGrayColorTransform(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            threshold_value: c_float,
            maximum_value: c_float,
            transform: *const c_float,
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

        /// The threshold value used to init the threshold filter
        #[method(thresholdValue)]
        pub unsafe fn thresholdValue(&self) -> c_float;

        /// The maximum value used to init the threshold filter
        #[method(maximumValue)]
        pub unsafe fn maximumValue(&self) -> c_float;

        /// The color transform used to init the threshold filter
        #[method(transform)]
        pub unsafe fn transform(&self) -> NonNull<c_float>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdBinary {
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
    unsafe impl MPSImageThresholdBinary {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The MPSImageThresholdBinaryInverse filter applies a fixed-level threshold to each pixel in the image.
    /// The threshold functions convert a single channel image to a binary image.
    /// If the input image is not a single channel image, convert the inputimage to a single channel
    /// luminance image using the linearGrayColorTransform and then apply the threshold.
    /// The ThresholdBinaryInverse function is:
    /// destinationPixelValue = sourcePixelValue > thresholdValue ? 0 : maximumValue
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagethresholdbinaryinverse?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageThresholdBinaryInverse;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageThresholdBinaryInverse {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageThresholdBinaryInverse {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageThresholdBinaryInverse {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageThresholdBinaryInverse {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageThresholdBinaryInverse {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdBinaryInverse {
        /// initialize a MPSImageThresholdBinaryInverse filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `thresholdValue`: The threshold value to use
        ///
        /// Parameter `maximumValue`: The maximum value to use
        ///
        /// Parameter `transform`: This matrix is an array of 3 floats.
        /// The default if no transform is specifed is BT.601/JPEG: {0.299f, 0.587f, 0.114f};
        #[method_id(@__retain_semantics Init initWithDevice:thresholdValue:maximumValue:linearGrayColorTransform:)]
        pub unsafe fn initWithDevice_thresholdValue_maximumValue_linearGrayColorTransform(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            threshold_value: c_float,
            maximum_value: c_float,
            transform: *const c_float,
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

        /// The threshold value used to init the threshold filter
        #[method(thresholdValue)]
        pub unsafe fn thresholdValue(&self) -> c_float;

        /// The maximum value used to init the threshold filter
        #[method(maximumValue)]
        pub unsafe fn maximumValue(&self) -> c_float;

        /// The color transform used to init the threshold filter
        #[method(transform)]
        pub unsafe fn transform(&self) -> NonNull<c_float>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdBinaryInverse {
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
    unsafe impl MPSImageThresholdBinaryInverse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The MPSImageThresholdTruncate filter applies a fixed-level threshold to each pixel in the image:
    /// The threshold functions convert a single channel image to a binary image.
    /// If the input image is not a single channel image, convert the inputimage to a single channel
    /// luminance image using the linearGrayColorTransform and then apply the threshold.
    /// The ThresholdTruncate function is:
    /// destinationPixelValue = sourcePixelValue > thresholdValue ? thresholdValue : sourcePixelValue
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagethresholdtruncate?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageThresholdTruncate;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageThresholdTruncate {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageThresholdTruncate {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageThresholdTruncate {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageThresholdTruncate {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageThresholdTruncate {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdTruncate {
        /// initialize a MPSImageThresholdTruncate filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `thresholdValue`: The threshold value to use
        ///
        /// Parameter `transform`: This matrix is an array of 3 floats.
        /// The default if no transform is specifed is BT.601/JPEG: {0.299f, 0.587f, 0.114f};
        #[method_id(@__retain_semantics Init initWithDevice:thresholdValue:linearGrayColorTransform:)]
        pub unsafe fn initWithDevice_thresholdValue_linearGrayColorTransform(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            threshold_value: c_float,
            transform: *const c_float,
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

        /// The threshold value used to init the threshold filter
        #[method(thresholdValue)]
        pub unsafe fn thresholdValue(&self) -> c_float;

        /// The color transform used to init the threshold filter
        #[method(transform)]
        pub unsafe fn transform(&self) -> NonNull<c_float>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdTruncate {
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
    unsafe impl MPSImageThresholdTruncate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The MPSImageThresholdToZero filter applies a fixed-level threshold to each pixel in the image.
    /// The threshold functions convert a single channel image to a binary image.
    /// If the input image is not a single channel image, convert the inputimage to a single channel
    /// luminance image using the linearGrayColorTransform and then apply the threshold.
    /// The ThresholdToZero function is:
    /// destinationPixelValue = sourcePixelValue > thresholdValue ? sourcePixelValue : 0
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagethresholdtozero?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageThresholdToZero;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageThresholdToZero {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageThresholdToZero {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageThresholdToZero {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageThresholdToZero {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageThresholdToZero {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdToZero {
        /// initialize a MPSImageThresholdToZero filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `thresholdValue`: The threshold value to use
        ///
        /// Parameter `transform`: This matrix is an array of 3 floats.
        /// The default if no transform is specifed is BT.601/JPEG: {0.299f, 0.587f, 0.114f};
        #[method_id(@__retain_semantics Init initWithDevice:thresholdValue:linearGrayColorTransform:)]
        pub unsafe fn initWithDevice_thresholdValue_linearGrayColorTransform(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            threshold_value: c_float,
            transform: *const c_float,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
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

        /// The threshold value used to init the threshold filter
        #[method(thresholdValue)]
        pub unsafe fn thresholdValue(&self) -> c_float;

        /// The color transform used to init the threshold filter
        #[method(transform)]
        pub unsafe fn transform(&self) -> NonNull<c_float>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdToZero {
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
    unsafe impl MPSImageThresholdToZero {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The MPSImageThresholdToZeroInverse filter applies a fixed-level threshold to each pixel in the image.
    /// The threshold functions convert a single channel image to a binary image.
    /// If the input image is not a single channel image, convert the inputimage to a single channel
    /// luminance image using the linearGrayColorTransform and then apply the threshold.
    /// The ThresholdToZeroINverse function is:
    /// destinationPixelValue = sourcePixelValue > thresholdValue ? 0 : sourcePixelValue
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagethresholdtozeroinverse?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageThresholdToZeroInverse;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageThresholdToZeroInverse {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageThresholdToZeroInverse {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageThresholdToZeroInverse {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageThresholdToZeroInverse {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageThresholdToZeroInverse {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdToZeroInverse {
        /// initialize a MPSImageThresholdToZeroInverse filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `thresholdValue`: The threshold value to use
        ///
        /// Parameter `transform`: This matrix is an array of 3 floats.
        /// The default if no transform is specifed is BT.601/JPEG: {0.299f, 0.587f, 0.114f};
        #[method_id(@__retain_semantics Init initWithDevice:thresholdValue:linearGrayColorTransform:)]
        pub unsafe fn initWithDevice_thresholdValue_linearGrayColorTransform(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            threshold_value: c_float,
            transform: *const c_float,
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

        /// The threshold value used to init the threshold filter
        #[method(thresholdValue)]
        pub unsafe fn thresholdValue(&self) -> c_float;

        /// The color transform used to init the threshold filter
        #[method(transform)]
        pub unsafe fn transform(&self) -> NonNull<c_float>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageThresholdToZeroInverse {
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
    unsafe impl MPSImageThresholdToZeroInverse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
