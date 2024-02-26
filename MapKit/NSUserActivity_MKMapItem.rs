//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_category!(
    /// Category "MKMapItem" on [`NSUserActivity`].
    #[doc(alias = "MKMapItem")]
    pub unsafe trait NSUserActivityMKMapItem {
        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other mapItem)]
        unsafe fn mapItem(&self) -> Option<Id<MKMapItem>>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method(setMapItem:)]
        unsafe fn setMapItem(&self, map_item: Option<&MKMapItem>);
    }

    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl NSUserActivityMKMapItem for NSUserActivity {}
);
