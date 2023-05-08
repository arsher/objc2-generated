//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

typed_enum!(
    pub type HKFHIRResourceType = NSString;
);

extern_static!(HKFHIRResourceTypeAllergyIntolerance: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeCondition: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeCoverage: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeImmunization: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeMedicationDispense: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeMedicationOrder: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeMedicationRequest: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeMedicationStatement: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeObservation: &'static HKFHIRResourceType);

extern_static!(HKFHIRResourceTypeProcedure: &'static HKFHIRResourceType);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKFHIRResource")]
    pub struct HKFHIRResource;

    #[cfg(feature = "HealthKit_HKFHIRResource")]
    unsafe impl ClassType for HKFHIRResource {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKFHIRResource")]
unsafe impl NSCoding for HKFHIRResource {}

#[cfg(feature = "HealthKit_HKFHIRResource")]
unsafe impl NSCopying for HKFHIRResource {}

#[cfg(feature = "HealthKit_HKFHIRResource")]
unsafe impl NSObjectProtocol for HKFHIRResource {}

#[cfg(feature = "HealthKit_HKFHIRResource")]
unsafe impl NSSecureCoding for HKFHIRResource {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKFHIRResource")]
    unsafe impl HKFHIRResource {
        #[cfg(feature = "HealthKit_HKFHIRVersion")]
        #[method_id(@__retain_semantics Other FHIRVersion)]
        pub unsafe fn FHIRVersion(&self) -> Id<HKFHIRVersion>;

        #[method_id(@__retain_semantics Other resourceType)]
        pub unsafe fn resourceType(&self) -> Id<HKFHIRResourceType>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other sourceURL)]
        pub unsafe fn sourceURL(&self) -> Option<Id<NSURL>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKFHIRResource")]
    unsafe impl HKFHIRResource {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
