//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkobject?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKObject;
);

unsafe impl Send for HKObject {}

unsafe impl Sync for HKObject {}

unsafe impl NSCoding for HKObject {}

unsafe impl NSObjectProtocol for HKObject {}

unsafe impl NSSecureCoding for HKObject {}

extern_methods!(
    unsafe impl HKObject {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[cfg(feature = "HKSource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Retained<HKSource>;

        #[cfg(feature = "HKSourceRevision")]
        #[method_id(@__retain_semantics Other sourceRevision)]
        pub unsafe fn sourceRevision(&self) -> Retained<HKSourceRevision>;

        #[cfg(feature = "HKDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Retained<HKDevice>>;

        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKObject {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathuuid?language=objc)
    pub static HKPredicateKeyPathUUID: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathsource?language=objc)
    pub static HKPredicateKeyPathSource: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathmetadata?language=objc)
    pub static HKPredicateKeyPathMetadata: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathcorrelation?language=objc)
    pub static HKPredicateKeyPathCorrelation: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkout?language=objc)
    pub static HKPredicateKeyPathWorkout: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathdevice?language=objc)
    pub static HKPredicateKeyPathDevice: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathsourcerevision?language=objc)
    pub static HKPredicateKeyPathSourceRevision: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkpredicatekeypathworkouteffortrelationship?language=objc)
    pub static HKPredicateKeyPathWorkoutEffortRelationship: &'static NSString;
}
