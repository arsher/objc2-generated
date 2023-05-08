//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLookAroundSnapshot")]
    pub struct MKLookAroundSnapshot;

    #[cfg(feature = "MapKit_MKLookAroundSnapshot")]
    unsafe impl ClassType for MKLookAroundSnapshot {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLookAroundSnapshot")]
unsafe impl NSObjectProtocol for MKLookAroundSnapshot {}

extern_methods!(
    #[cfg(feature = "MapKit_MKLookAroundSnapshot")]
    unsafe impl MKLookAroundSnapshot {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKLookAroundSnapshot")]
    unsafe impl MKLookAroundSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
