//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRulerOrientation {
        NSHorizontalRuler = 0,
        NSVerticalRuler = 1,
    }
);

typed_extensible_enum!(
    pub type NSRulerViewUnitName = Foundation::NSString;
);

extern_static!(NSRulerViewUnitInches: &'static AppKit::NSRulerViewUnitName);

extern_static!(NSRulerViewUnitCentimeters: &'static AppKit::NSRulerViewUnitName);

extern_static!(NSRulerViewUnitPoints: &'static AppKit::NSRulerViewUnitName);

extern_static!(NSRulerViewUnitPicas: &'static AppKit::NSRulerViewUnitName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRulerView;

    unsafe impl ClassType for NSRulerView {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSRulerView")]
    unsafe impl NSRulerView {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method(registerUnitWithName:abbreviation:unitToPointsConversionFactor:stepUpCycle:stepDownCycle:)]
        pub unsafe fn registerUnitWithName_abbreviation_unitToPointsConversionFactor_stepUpCycle_stepDownCycle(
            unitName: &AppKit::NSRulerViewUnitName,
            abbreviation: &Foundation::NSString,
            conversionFactor: CGFloat,
            stepUpCycle: &Foundation::NSArray<Foundation::NSNumber>,
            stepDownCycle: &Foundation::NSArray<Foundation::NSNumber>,
        );

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSScrollView")]
        #[method_id(@__retain_semantics Init initWithScrollView:orientation:)]
        pub unsafe fn initWithScrollView_orientation(
            this: Option<Allocated<Self>>,
            scrollView: Option<&AppKit::NSScrollView>,
            orientation: NSRulerOrientation,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSScrollView")]
        #[method_id(@__retain_semantics Other scrollView)]
        pub unsafe fn scrollView(&self) -> Option<Id<AppKit::NSScrollView, Shared>>;

        #[cfg(feature = "AppKit_NSScrollView")]
        #[method(setScrollView:)]
        pub unsafe fn setScrollView(&self, scrollView: Option<&AppKit::NSScrollView>);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSRulerOrientation;

        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSRulerOrientation);

        #[method(baselineLocation)]
        pub unsafe fn baselineLocation(&self) -> CGFloat;

        #[method(requiredThickness)]
        pub unsafe fn requiredThickness(&self) -> CGFloat;

        #[method(ruleThickness)]
        pub unsafe fn ruleThickness(&self) -> CGFloat;

        #[method(setRuleThickness:)]
        pub unsafe fn setRuleThickness(&self, ruleThickness: CGFloat);

        #[method(reservedThicknessForMarkers)]
        pub unsafe fn reservedThicknessForMarkers(&self) -> CGFloat;

        #[method(setReservedThicknessForMarkers:)]
        pub unsafe fn setReservedThicknessForMarkers(&self, reservedThicknessForMarkers: CGFloat);

        #[method(reservedThicknessForAccessoryView)]
        pub unsafe fn reservedThicknessForAccessoryView(&self) -> CGFloat;

        #[method(setReservedThicknessForAccessoryView:)]
        pub unsafe fn setReservedThicknessForAccessoryView(
            &self,
            reservedThicknessForAccessoryView: CGFloat,
        );

        #[method_id(@__retain_semantics Other measurementUnits)]
        pub unsafe fn measurementUnits(&self) -> Id<AppKit::NSRulerViewUnitName, Shared>;

        #[method(setMeasurementUnits:)]
        pub unsafe fn setMeasurementUnits(&self, measurementUnits: &AppKit::NSRulerViewUnitName);

        #[method(originOffset)]
        pub unsafe fn originOffset(&self) -> CGFloat;

        #[method(setOriginOffset:)]
        pub unsafe fn setOriginOffset(&self, originOffset: CGFloat);

        #[method_id(@__retain_semantics Other clientView)]
        pub unsafe fn clientView(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[method(setClientView:)]
        pub unsafe fn setClientView(&self, clientView: Option<&AppKit::NSView>);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(addMarker:)]
        pub unsafe fn addMarker(&self, marker: &AppKit::NSRulerMarker);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(removeMarker:)]
        pub unsafe fn removeMarker(&self, marker: &AppKit::NSRulerMarker);

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other markers)]
        pub unsafe fn markers(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSRulerMarker>, Shared>>;

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "Foundation_NSArray"))]
        #[method(setMarkers:)]
        pub unsafe fn setMarkers(
            &self,
            markers: Option<&Foundation::NSArray<AppKit::NSRulerMarker>>,
        );

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSRulerMarker"))]
        #[method(trackMarker:withMouseEvent:)]
        pub unsafe fn trackMarker_withMouseEvent(
            &self,
            marker: &AppKit::NSRulerMarker,
            event: &AppKit::NSEvent,
        ) -> bool;

        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&AppKit::NSView>);

        #[method(moveRulerlineFromLocation:toLocation:)]
        pub unsafe fn moveRulerlineFromLocation_toLocation(
            &self,
            oldLocation: CGFloat,
            newLocation: CGFloat,
        );

        #[method(invalidateHashMarks)]
        pub unsafe fn invalidateHashMarks(&self);

        #[method(drawHashMarksAndLabelsInRect:)]
        pub unsafe fn drawHashMarksAndLabelsInRect(&self, rect: NSRect);

        #[method(drawMarkersInRect:)]
        pub unsafe fn drawMarkersInRect(&self, rect: NSRect);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);

extern_methods!(
    /// NSRulerMarkerClientViewDelegation
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl AppKit::NSView {
        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:shouldMoveMarker:)]
        pub unsafe fn rulerView_shouldMoveMarker(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:willMoveMarker:toLocation:)]
        pub unsafe fn rulerView_willMoveMarker_toLocation(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:didMoveMarker:)]
        pub unsafe fn rulerView_didMoveMarker(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
        );

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:shouldRemoveMarker:)]
        pub unsafe fn rulerView_shouldRemoveMarker(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:didRemoveMarker:)]
        pub unsafe fn rulerView_didRemoveMarker(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
        );

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:shouldAddMarker:)]
        pub unsafe fn rulerView_shouldAddMarker(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:willAddMarker:atLocation:)]
        pub unsafe fn rulerView_willAddMarker_atLocation(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;

        #[cfg(all(feature = "AppKit_NSRulerMarker", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:didAddMarker:)]
        pub unsafe fn rulerView_didAddMarker(
            &self,
            ruler: &AppKit::NSRulerView,
            marker: &AppKit::NSRulerMarker,
        );

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSRulerView"))]
        #[method(rulerView:handleMouseDown:)]
        pub unsafe fn rulerView_handleMouseDown(
            &self,
            ruler: &AppKit::NSRulerView,
            event: &AppKit::NSEvent,
        );

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method(rulerView:willSetClientView:)]
        pub unsafe fn rulerView_willSetClientView(
            &self,
            ruler: &AppKit::NSRulerView,
            newClient: &AppKit::NSView,
        );

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method(rulerView:locationForPoint:)]
        pub unsafe fn rulerView_locationForPoint(
            &self,
            ruler: &AppKit::NSRulerView,
            point: NSPoint,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSRulerView")]
        #[method(rulerView:pointForLocation:)]
        pub unsafe fn rulerView_pointForLocation(
            &self,
            ruler: &AppKit::NSRulerView,
            point: CGFloat,
        ) -> NSPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSRulerView")]
    unsafe impl AppKit::NSRulerView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
