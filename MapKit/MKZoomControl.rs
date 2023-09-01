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
    #[cfg(feature = "MapKit_MKZoomControl")]
    pub struct MKZoomControl;

    #[cfg(feature = "MapKit_MKZoomControl")]
    unsafe impl ClassType for MKZoomControl {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSAccessibility for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSAccessibilityElementProtocol for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSAnimatablePropertyContainer for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSAppearanceCustomization for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSCoding for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSDraggingDestination for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSObjectProtocol for MKZoomControl {}

#[cfg(feature = "MapKit_MKZoomControl")]
unsafe impl NSUserInterfaceItemIdentification for MKZoomControl {}

extern_methods!(
    #[cfg(feature = "MapKit_MKZoomControl")]
    unsafe impl MKZoomControl {
        #[cfg(feature = "MapKit_MKMapView")]
        #[method_id(@__retain_semantics Other zoomControlWithMapView:)]
        pub unsafe fn zoomControlWithMapView(map_view: Option<&MKMapView>) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMapView")]
        #[method_id(@__retain_semantics Other mapView)]
        pub unsafe fn mapView(&self) -> Option<Id<MKMapView>>;

        #[cfg(feature = "MapKit_MKMapView")]
        #[method(setMapView:)]
        pub unsafe fn setMapView(&self, map_view: Option<&MKMapView>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKZoomControl")]
    unsafe impl MKZoomControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "MapKit_MKZoomControl")]
    unsafe impl MKZoomControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKZoomControl")]
    unsafe impl MKZoomControl {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
