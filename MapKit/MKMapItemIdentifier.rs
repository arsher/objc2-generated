//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapitemidentifier?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapItemIdentifier;
);

unsafe impl NSCoding for MKMapItemIdentifier {}

unsafe impl NSCopying for MKMapItemIdentifier {}

unsafe impl CopyingHelper for MKMapItemIdentifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MKMapItemIdentifier {}

unsafe impl NSSecureCoding for MKMapItemIdentifier {}

extern_methods!(
    unsafe impl MKMapItemIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIdentifierString:)]
        pub unsafe fn initWithIdentifierString(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other identifierString)]
        pub unsafe fn identifierString(&self) -> Retained<NSString>;
    }
);
