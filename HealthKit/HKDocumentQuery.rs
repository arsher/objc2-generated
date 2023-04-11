//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKDocumentQuery")]
    pub struct HKDocumentQuery;

    #[cfg(feature = "HealthKit_HKDocumentQuery")]
    unsafe impl ClassType for HKDocumentQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKDocumentQuery")]
unsafe impl NSObjectProtocol for HKDocumentQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKDocumentQuery")]
    unsafe impl HKDocumentQuery {
        #[method(limit)]
        pub unsafe fn limit(&self) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Id<NSArray<NSSortDescriptor>>>;

        #[method(includeDocumentData)]
        pub unsafe fn includeDocumentData(&self) -> bool;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSSortDescriptor",
            feature = "HealthKit_HKDocumentSample",
            feature = "HealthKit_HKDocumentType"
        ))]
        #[method_id(@__retain_semantics Init initWithDocumentType:predicate:limit:sortDescriptors:includeDocumentData:resultsHandler:)]
        pub unsafe fn initWithDocumentType_predicate_limit_sortDescriptors_includeDocumentData_resultsHandler(
            this: Option<Allocated<Self>>,
            document_type: &HKDocumentType,
            predicate: Option<&NSPredicate>,
            limit: NSUInteger,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
            include_document_data: bool,
            results_handler: &Block<
                (
                    NonNull<HKDocumentQuery>,
                    *mut NSArray<HKDocumentSample>,
                    Bool,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);
