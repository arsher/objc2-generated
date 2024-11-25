//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapitemannotation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapItemAnnotation;
);

#[cfg(feature = "MKAnnotation")]
unsafe impl MKAnnotation for MKMapItemAnnotation {}

unsafe impl NSObjectProtocol for MKMapItemAnnotation {}

extern_methods!(
    unsafe impl MKMapItemAnnotation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MKMapItem")]
        #[method_id(@__retain_semantics Init initWithMapItem:)]
        pub unsafe fn initWithMapItem(
            this: Allocated<Self>,
            map_item: &MKMapItem,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MKMapItem")]
        #[method_id(@__retain_semantics Other mapItem)]
        pub unsafe fn mapItem(&self) -> Retained<MKMapItem>;
    }
);
