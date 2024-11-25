//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkclusterannotation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKClusterAnnotation;
);

#[cfg(feature = "MKAnnotation")]
unsafe impl MKAnnotation for MKClusterAnnotation {}

unsafe impl NSObjectProtocol for MKClusterAnnotation {}

extern_methods!(
    unsafe impl MKClusterAnnotation {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "MKAnnotation")]
        #[method_id(@__retain_semantics Other memberAnnotations)]
        pub unsafe fn memberAnnotations(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MKAnnotation>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MKAnnotation")]
        #[method_id(@__retain_semantics Init initWithMemberAnnotations:)]
        pub unsafe fn initWithMemberAnnotations(
            this: Allocated<Self>,
            member_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKClusterAnnotation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
