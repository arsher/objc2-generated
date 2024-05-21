//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIEventAttribution;

    unsafe impl ClassType for UIEventAttribution {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCopying for UIEventAttribution {}

unsafe impl NSObjectProtocol for UIEventAttribution {}

extern_methods!(
    unsafe impl UIEventAttribution {
        #[method(sourceIdentifier)]
        pub unsafe fn sourceIdentifier(&self) -> u8;

        #[method_id(@__retain_semantics Other destinationURL)]
        pub unsafe fn destinationURL(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other reportEndpoint)]
        pub unsafe fn reportEndpoint(&self) -> Option<Retained<NSURL>>;

        #[method_id(@__retain_semantics Other sourceDescription)]
        pub unsafe fn sourceDescription(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other purchaser)]
        pub unsafe fn purchaser(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Init initWithSourceIdentifier:destinationURL:sourceDescription:purchaser:)]
        pub unsafe fn initWithSourceIdentifier_destinationURL_sourceDescription_purchaser(
            this: Allocated<Self>,
            source_identifier: u8,
            destination_url: &NSURL,
            source_description: &NSString,
            purchaser: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
