//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandomdistribution?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSMatrixRandomDistribution(pub NSUInteger);
bitflags::bitflags! {
    impl MPSMatrixRandomDistribution: NSUInteger {
        #[doc(alias = "MPSMatrixRandomDistributionDefault")]
        const Default = 1;
        #[doc(alias = "MPSMatrixRandomDistributionUniform")]
        const Uniform = 2;
        #[doc(alias = "MPSMatrixRandomDistributionNormal")]
        const Normal = 3;
    }
}

unsafe impl Encode for MPSMatrixRandomDistribution {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSMatrixRandomDistribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Dependencies: This depends on Metal.framework
    ///
    /// Decribes properties of a distribution of random values.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandomdistributiondescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSMatrixRandomDistributionDescriptor;
);

unsafe impl NSCopying for MPSMatrixRandomDistributionDescriptor {}

unsafe impl CopyingHelper for MPSMatrixRandomDistributionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPSMatrixRandomDistributionDescriptor {}

extern_methods!(
    unsafe impl MPSMatrixRandomDistributionDescriptor {
        /// The type of distribution.
        #[method(distributionType)]
        pub unsafe fn distributionType(&self) -> MPSMatrixRandomDistribution;

        /// Setter for [`distributionType`][Self::distributionType].
        #[method(setDistributionType:)]
        pub unsafe fn setDistributionType(&self, distribution_type: MPSMatrixRandomDistribution);

        /// For distributions of values bounded below, this value describes the minimum.
        #[method(minimum)]
        pub unsafe fn minimum(&self) -> c_float;

        /// Setter for [`minimum`][Self::minimum].
        #[method(setMinimum:)]
        pub unsafe fn setMinimum(&self, minimum: c_float);

        /// For distributions of values bounded above, this value describes the maximum.
        #[method(maximum)]
        pub unsafe fn maximum(&self) -> c_float;

        /// Setter for [`maximum`][Self::maximum].
        #[method(setMaximum:)]
        pub unsafe fn setMaximum(&self, maximum: c_float);

        /// The value to use for distributions described by their mean.
        #[method(mean)]
        pub unsafe fn mean(&self) -> c_float;

        /// Setter for [`mean`][Self::mean].
        #[method(setMean:)]
        pub unsafe fn setMean(&self, mean: c_float);

        /// The value to use for distributions described by their standardDeviation.
        #[method(standardDeviation)]
        pub unsafe fn standardDeviation(&self) -> c_float;

        /// Setter for [`standardDeviation`][Self::standardDeviation].
        #[method(setStandardDeviation:)]
        pub unsafe fn setStandardDeviation(&self, standard_deviation: c_float);

        /// Make a descriptor for a uniform distribution of floating point values in
        /// the range [minimum, maximum).
        ///
        /// Parameter `minimum`: The lower bound of the range.
        ///
        /// Parameter `maximum`: The upper bound of the range.
        ///
        /// Returns: A valid MPSMatrixRandomDistribution object or nil, if failure.
        #[method_id(@__retain_semantics Other uniformDistributionDescriptorWithMinimum:maximum:)]
        pub unsafe fn uniformDistributionDescriptorWithMinimum_maximum(
            minimum: c_float,
            maximum: c_float,
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;

        /// Make a descriptor for a normal distribution of floating point values.
        ///
        /// Parameter `mean`: The mean of the distribution
        ///
        /// Parameter `standardDeviation`: The standard deviation of the distribution.
        ///
        /// Returns: A valid MPSMatrixRandomDistribution object or nil if failure.
        #[method_id(@__retain_semantics Other normalDistributionDescriptorWithMean:standardDeviation:)]
        pub unsafe fn normalDistributionDescriptorWithMean_standardDeviation(
            mean: c_float,
            standard_deviation: c_float,
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;

        /// Make a descriptor for a truncated normal distribution of floating point values.
        ///
        /// Parameter `mean`: The mean of the distribution
        ///
        /// Parameter `standardDeviation`: The standard deviation of the distribution.
        ///
        /// Parameter `minimum`: The lower bound of the distribution
        ///
        /// Parameter `maximum`: The upper bound of the distribution
        ///
        /// Returns: A valid MPSMatrixRandomDistribution object or nil if failure.
        #[method_id(@__retain_semantics Other normalDistributionDescriptorWithMean:standardDeviation:minimum:maximum:)]
        pub unsafe fn normalDistributionDescriptorWithMean_standardDeviation_minimum_maximum(
            mean: c_float,
            standard_deviation: c_float,
            minimum: c_float,
            maximum: c_float,
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;

        /// Make a descriptor for a default distribution.
        ///
        /// Returns: A valid MPSMatrixRandomDistribution object or nil, if failure.
        #[method_id(@__retain_semantics Other defaultDistributionDescriptor)]
        pub unsafe fn defaultDistributionDescriptor(
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSMatrixRandomDistributionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Kernels that implement random number generation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandom?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixRandom;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixRandom {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixRandom {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixRandom {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixRandom {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixRandom {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandom {
        #[cfg(feature = "MPSCoreTypes")]
        /// The type of the data which makes up the values of the result.
        /// Supported values are:
        /// MPSDataTypeUInt32
        /// MPSDataTypeFloat32
        ///
        /// Default is MPSDataTypeUInt32
        #[method(destinationDataType)]
        pub unsafe fn destinationDataType(&self) -> MPSDataType;

        /// The distribution from which to generate random values.
        ///
        /// Default is MPSMatrixRandomDistributionDefault
        #[method(distributionType)]
        pub unsafe fn distributionType(&self) -> MPSMatrixRandomDistribution;

        /// The starting index in the destination batch.
        #[method(batchStart)]
        pub unsafe fn batchStart(&self) -> NSUInteger;

        /// Setter for [`batchStart`][Self::batchStart].
        #[method(setBatchStart:)]
        pub unsafe fn setBatchStart(&self, batch_start: NSUInteger);

        /// The size of the batch to process.
        #[method(batchSize)]
        pub unsafe fn batchSize(&self) -> NSUInteger;

        /// Setter for [`batchSize`][Self::batchSize].
        #[method(setBatchSize:)]
        pub unsafe fn setBatchSize(&self, batch_size: NSUInteger);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSMatrix")]
        /// Encode a MPSMatrixRandom kernel into a command Buffer.
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer to receive the encoded filter
        ///
        /// Parameter `destinationVector`: A valid MPSVector to contain the result.
        #[method(encodeToCommandBuffer:destinationVector:)]
        pub unsafe fn encodeToCommandBuffer_destinationVector(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            destination_vector: &MPSVector,
        );

        #[cfg(feature = "MPSMatrix")]
        /// Encode a MPSMatrixRandom kernel into a command Buffer.
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer to receive the encoded filter
        ///
        /// Parameter `destinationMatrix`: A valid MPSMatrix to contain the result.
        #[method(encodeToCommandBuffer:destinationMatrix:)]
        pub unsafe fn encodeToCommandBuffer_destinationMatrix(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            destination_matrix: &MPSMatrix,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandom {
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
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandom {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Generates random numbers using a Mersenne Twister algorithm
    /// suitable for GPU execution.  It uses a period of 2**11214.
    /// For further details see:
    /// Mutsuo Saito. A Variant of Mersenne Twister Suitable for Graphic Processors. arXiv:1005.4973
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandommtgp32?language=objc)
    #[unsafe(super(MPSMatrixRandom, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixRandomMTGP32;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixRandomMTGP32 {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixRandomMTGP32 {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixRandomMTGP32 {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixRandomMTGP32 {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixRandomMTGP32 {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomMTGP32 {
        /// initialize a MPSMatrixRandomMTGP32 filter to generate 32-bit unsigned
        /// integer values with an initial seed of 0.
        ///
        /// Parameter `device`: The device the filter will run on
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        /// initialize a MPSMatrixRandomMTGP32 filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `destinationDataType`: The data type of the result.
        ///
        /// Parameter `seed`: The seed to initialize the random number generators with.
        ///
        /// Parameter `distributionDescriptor`: A descriptor containing information about the distribution.
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:distributionDescriptor:)]
        pub unsafe fn initWithDevice_destinationDataType_seed_distributionDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
            distribution_descriptor: &MPSMatrixRandomDistributionDescriptor,
        ) -> Retained<Self>;

        /// Synchronize internal MTGP32 state between GPU and CPU.
        ///
        /// Parameter `commandBuffer`: The command buffer on which to encode the synchronization.
        #[method(synchronizeStateOnCommandBuffer:)]
        pub unsafe fn synchronizeStateOnCommandBuffer(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
        );

        #[cfg(feature = "MPSCoreTypes")]
        /// initialize a MPSMatrixRandomMTGP32 filter using a default distribution.
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `destinationDataType`: The data type of the result.
        ///
        /// Parameter `seed`: The seed to initialize the random number generators with.
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:)]
        pub unsafe fn initWithDevice_destinationDataType_seed(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomMTGP32 {
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
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomMTGP32 {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Generates random numbers using a counter based algorithm.
    /// For further details see:
    /// John K. Salmon, Mark A. Moraes, Ron O. Dror, and David E. Shaw. Parallel Random Numbers: As Easy as 1, 2, 3.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandomphilox?language=objc)
    #[unsafe(super(MPSMatrixRandom, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixRandomPhilox;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixRandomPhilox {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixRandomPhilox {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixRandomPhilox {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixRandomPhilox {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixRandomPhilox {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomPhilox {
        /// initialize a MPSMatrixRandomPhilox filter to generate 32-bit unsigned
        /// integer values with an initial seed of 0.
        ///
        /// Parameter `device`: The device the filter will run on
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        /// initialize a MPSMatrixRandomPhilox filter
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `destinationDataType`: The data type of the result.
        ///
        /// Parameter `seed`: The seed to initialize the random number generators with.
        ///
        /// Parameter `distributionDescriptor`: A descriptor containing information about the distribution.
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:distributionDescriptor:)]
        pub unsafe fn initWithDevice_destinationDataType_seed_distributionDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
            distribution_descriptor: &MPSMatrixRandomDistributionDescriptor,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        /// initialize a MPSMatrixRandomPhilox filter using a default distribution.
        ///
        /// Parameter `device`: The device the filter will run on
        ///
        /// Parameter `destinationDataType`: The data type of the result.
        ///
        /// Parameter `seed`: The seed to initialize the random number generators with.
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:)]
        pub unsafe fn initWithDevice_destinationDataType_seed(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomPhilox {
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
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomPhilox {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
