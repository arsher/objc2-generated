//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxhistogrambucket?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXHistogramBucket<UnitType: ?Sized = AnyObject>;
);

unsafe impl<UnitType: ?Sized + NSCoding> NSCoding for MXHistogramBucket<UnitType> {}

unsafe impl<UnitType: ?Sized> NSObjectProtocol for MXHistogramBucket<UnitType> {}

unsafe impl<UnitType: ?Sized + NSSecureCoding> NSSecureCoding for MXHistogramBucket<UnitType> {}

extern_methods!(
    unsafe impl<UnitType: Message> MXHistogramBucket<UnitType> {
        #[method_id(@__retain_semantics Other bucketStart)]
        pub unsafe fn bucketStart(&self) -> Retained<NSMeasurement<UnitType>>;

        #[method_id(@__retain_semantics Other bucketEnd)]
        pub unsafe fn bucketEnd(&self) -> Retained<NSMeasurement<UnitType>>;

        #[method(bucketCount)]
        pub unsafe fn bucketCount(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<UnitType: Message> MXHistogramBucket<UnitType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxhistogram?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXHistogram<UnitType: ?Sized = AnyObject>;
);

unsafe impl<UnitType: ?Sized + NSCoding> NSCoding for MXHistogram<UnitType> {}

unsafe impl<UnitType: ?Sized> NSObjectProtocol for MXHistogram<UnitType> {}

unsafe impl<UnitType: ?Sized + NSSecureCoding> NSSecureCoding for MXHistogram<UnitType> {}

extern_methods!(
    unsafe impl<UnitType: Message> MXHistogram<UnitType> {
        #[method(totalBucketCount)]
        pub unsafe fn totalBucketCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other bucketEnumerator)]
        pub unsafe fn bucketEnumerator(
            &self,
        ) -> Retained<NSEnumerator<MXHistogramBucket<UnitType>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<UnitType: Message> MXHistogram<UnitType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
