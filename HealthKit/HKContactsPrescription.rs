//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    pub struct HKContactsPrescription;

    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    unsafe impl ClassType for HKContactsPrescription {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKVisionPrescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSCoding for HKContactsPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSCopying for HKContactsPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSObjectProtocol for HKContactsPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSSecureCoding for HKContactsPrescription {}

extern_methods!(
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    unsafe impl HKContactsPrescription {
        #[cfg(all(
            feature = "HKContactsLensSpecification",
            feature = "HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other rightEye)]
        pub unsafe fn rightEye(&self) -> Option<Retained<HKContactsLensSpecification>>;

        #[cfg(all(
            feature = "HKContactsLensSpecification",
            feature = "HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other leftEye)]
        pub unsafe fn leftEye(&self) -> Option<Retained<HKContactsLensSpecification>>;

        #[method_id(@__retain_semantics Other brand)]
        pub unsafe fn brand(&self) -> Retained<NSString>;

        #[cfg(all(
            feature = "HKContactsLensSpecification",
            feature = "HKDevice",
            feature = "HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other prescriptionWithRightEyeSpecification:leftEyeSpecification:brand:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_brand_dateIssued_expirationDate_device_metadata(
            right_eye_specification: Option<&HKContactsLensSpecification>,
            left_eye_specification: Option<&HKContactsLensSpecification>,
            brand: &NSString,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "HKDevice")]
        #[method_id(@__retain_semantics Other prescriptionWithType:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
            r#type: HKVisionPrescriptionType,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);
