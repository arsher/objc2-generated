//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "MKMapItem" on [`NSUserActivity`].
    #[doc(alias = "MKMapItem")]
    pub unsafe trait NSUserActivityMKMapItem {
        #[cfg(feature = "MKMapItem")]
        #[method_id(@__retain_semantics Other mapItem)]
        unsafe fn mapItem(&self) -> Option<Retained<MKMapItem>>;

        #[cfg(feature = "MKMapItem")]
        #[method(setMapItem:)]
        unsafe fn setMapItem(&self, map_item: Option<&MKMapItem>);
    }

    unsafe impl NSUserActivityMKMapItem for NSUserActivity {}
);
