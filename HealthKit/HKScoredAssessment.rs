//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKScoredAssessment;

    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl ClassType for HKScoredAssessment {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
    }
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Send for HKScoredAssessment {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Sync for HKScoredAssessment {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKScoredAssessment {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCopying for HKScoredAssessment {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl CopyingHelper for HKScoredAssessment {
    type Result = Self;
}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKScoredAssessment {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKScoredAssessment {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKScoredAssessment {
        #[method(score)]
        pub unsafe fn score(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
