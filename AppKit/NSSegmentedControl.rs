//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSegmentSwitchTracking {
        NSSegmentSwitchTrackingSelectOne = 0,
        NSSegmentSwitchTrackingSelectAny = 1,
        NSSegmentSwitchTrackingMomentary = 2,
        NSSegmentSwitchTrackingMomentaryAccelerator = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSegmentStyle {
        NSSegmentStyleAutomatic = 0,
        NSSegmentStyleRounded = 1,
        NSSegmentStyleRoundRect = 3,
        NSSegmentStyleTexturedSquare = 4,
        NSSegmentStyleSmallSquare = 6,
        NSSegmentStyleSeparated = 8,
        NSSegmentStyleTexturedRounded = 2,
        NSSegmentStyleCapsule = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSegmentDistribution {
        NSSegmentDistributionFit = 0,
        NSSegmentDistributionFill = 1,
        NSSegmentDistributionFillEqually = 2,
        NSSegmentDistributionFillProportionally = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSegmentedControl")]
    pub struct NSSegmentedControl;

    #[cfg(feature = "AppKit_NSSegmentedControl")]
    unsafe impl ClassType for NSSegmentedControl {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSAccessibility for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSAccessibilityElementProtocol for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSAnimatablePropertyContainer for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSAppearanceCustomization for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSCoding for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSDraggingDestination for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSObjectProtocol for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSUserInterfaceCompression for NSSegmentedControl {}

#[cfg(feature = "AppKit_NSSegmentedControl")]
unsafe impl NSUserInterfaceItemIdentification for NSSegmentedControl {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSegmentedControl")]
    unsafe impl NSSegmentedControl {
        #[method(segmentCount)]
        pub unsafe fn segmentCount(&self) -> NSInteger;

        #[method(setSegmentCount:)]
        pub unsafe fn setSegmentCount(&self, segment_count: NSInteger);

        #[method(selectedSegment)]
        pub unsafe fn selectedSegment(&self) -> NSInteger;

        #[method(setSelectedSegment:)]
        pub unsafe fn setSelectedSegment(&self, selected_segment: NSInteger);

        #[method(selectSegmentWithTag:)]
        pub unsafe fn selectSegmentWithTag(&self, tag: NSInteger) -> bool;

        #[method(setWidth:forSegment:)]
        pub unsafe fn setWidth_forSegment(&self, width: CGFloat, segment: NSInteger);

        #[method(widthForSegment:)]
        pub unsafe fn widthForSegment(&self, segment: NSInteger) -> CGFloat;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:forSegment:)]
        pub unsafe fn setImage_forSegment(&self, image: Option<&NSImage>, segment: NSInteger);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageForSegment:)]
        pub unsafe fn imageForSegment(&self, segment: NSInteger) -> Option<Id<NSImage>>;

        #[method(setImageScaling:forSegment:)]
        pub unsafe fn setImageScaling_forSegment(
            &self,
            scaling: NSImageScaling,
            segment: NSInteger,
        );

        #[method(imageScalingForSegment:)]
        pub unsafe fn imageScalingForSegment(&self, segment: NSInteger) -> NSImageScaling;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:forSegment:)]
        pub unsafe fn setLabel_forSegment(&self, label: &NSString, segment: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelForSegment:)]
        pub unsafe fn labelForSegment(&self, segment: NSInteger) -> Option<Id<NSString>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:forSegment:)]
        pub unsafe fn setMenu_forSegment(&self, menu: Option<&NSMenu>, segment: NSInteger);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menuForSegment:)]
        pub unsafe fn menuForSegment(&self, segment: NSInteger) -> Option<Id<NSMenu>>;

        #[method(setSelected:forSegment:)]
        pub unsafe fn setSelected_forSegment(&self, selected: bool, segment: NSInteger);

        #[method(isSelectedForSegment:)]
        pub unsafe fn isSelectedForSegment(&self, segment: NSInteger) -> bool;

        #[method(setEnabled:forSegment:)]
        pub unsafe fn setEnabled_forSegment(&self, enabled: bool, segment: NSInteger);

        #[method(isEnabledForSegment:)]
        pub unsafe fn isEnabledForSegment(&self, segment: NSInteger) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setToolTip:forSegment:)]
        pub unsafe fn setToolTip_forSegment(&self, tool_tip: Option<&NSString>, segment: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other toolTipForSegment:)]
        pub unsafe fn toolTipForSegment(&self, segment: NSInteger) -> Option<Id<NSString>>;

        #[method(setTag:forSegment:)]
        pub unsafe fn setTag_forSegment(&self, tag: NSInteger, segment: NSInteger);

        #[method(tagForSegment:)]
        pub unsafe fn tagForSegment(&self, segment: NSInteger) -> NSInteger;

        #[method(setShowsMenuIndicator:forSegment:)]
        pub unsafe fn setShowsMenuIndicator_forSegment(
            &self,
            shows_menu_indicator: bool,
            segment: NSInteger,
        );

        #[method(showsMenuIndicatorForSegment:)]
        pub unsafe fn showsMenuIndicatorForSegment(&self, segment: NSInteger) -> bool;

        #[method(segmentStyle)]
        pub unsafe fn segmentStyle(&self) -> NSSegmentStyle;

        #[method(setSegmentStyle:)]
        pub unsafe fn setSegmentStyle(&self, segment_style: NSSegmentStyle);

        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        #[method(trackingMode)]
        pub unsafe fn trackingMode(&self) -> NSSegmentSwitchTracking;

        #[method(setTrackingMode:)]
        pub unsafe fn setTrackingMode(&self, tracking_mode: NSSegmentSwitchTracking);

        #[method(doubleValueForSelectedSegment)]
        pub unsafe fn doubleValueForSelectedSegment(&self) -> c_double;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other selectedSegmentBezelColor)]
        pub unsafe fn selectedSegmentBezelColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setSelectedSegmentBezelColor:)]
        pub unsafe fn setSelectedSegmentBezelColor(
            &self,
            selected_segment_bezel_color: Option<&NSColor>,
        );

        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(setAlignment:forSegment:)]
        pub unsafe fn setAlignment_forSegment(
            &self,
            alignment: NSTextAlignment,
            segment: NSInteger,
        );

        #[method(alignmentForSegment:)]
        pub unsafe fn alignmentForSegment(&self, segment: NSInteger) -> NSTextAlignment;

        #[method(segmentDistribution)]
        pub unsafe fn segmentDistribution(&self) -> NSSegmentDistribution;

        #[method(setSegmentDistribution:)]
        pub unsafe fn setSegmentDistribution(&self, segment_distribution: NSSegmentDistribution);

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(&self) -> Id<NSUserInterfaceCompressionOptions>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSegmentedControl")]
    unsafe impl NSSegmentedControl {
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
    #[cfg(feature = "AppKit_NSSegmentedControl")]
    unsafe impl NSSegmentedControl {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSegmentedControl")]
    unsafe impl NSSegmentedControl {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSSegmentedControlConvenience
    #[cfg(feature = "AppKit_NSSegmentedControl")]
    unsafe impl NSSegmentedControl {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other segmentedControlWithLabels:trackingMode:target:action:)]
        pub unsafe fn segmentedControlWithLabels_trackingMode_target_action(
            labels: &NSArray<NSString>,
            tracking_mode: NSSegmentSwitchTracking,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other segmentedControlWithImages:trackingMode:target:action:)]
        pub unsafe fn segmentedControlWithImages_trackingMode_target_action(
            images: &NSArray<NSImage>,
            tracking_mode: NSSegmentSwitchTracking,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;
    }
);
