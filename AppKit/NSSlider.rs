//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSlider")]
    pub struct NSSlider;

    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl ClassType for NSSlider {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSAccessibility for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSAccessibilityElement for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSAccessibilitySlider for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSAnimatablePropertyContainer for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSAppearanceCustomization for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSCoding for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSDraggingDestination for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSObjectProtocol for NSSlider {}

#[cfg(feature = "AppKit_NSSlider")]
unsafe impl NSUserInterfaceItemIdentification for NSSlider {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method(sliderType)]
        pub unsafe fn sliderType(&self) -> NSSliderType;

        #[method(setSliderType:)]
        pub unsafe fn setSliderType(&self, slider_type: NSSliderType);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(altIncrementValue)]
        pub unsafe fn altIncrementValue(&self) -> c_double;

        #[method(setAltIncrementValue:)]
        pub unsafe fn setAltIncrementValue(&self, alt_increment_value: c_double);

        #[method(knobThickness)]
        pub unsafe fn knobThickness(&self) -> CGFloat;

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(acceptsFirstMouse:)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&NSEvent>) -> bool;

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other trackFillColor)]
        pub unsafe fn trackFillColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTrackFillColor:)]
        pub unsafe fn setTrackFillColor(&self, track_fill_color: Option<&NSColor>);
    }
);

extern_methods!(
    /// NSSliderVerticalGetter
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {}
);

extern_methods!(
    /// NSTickMarkSupport
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, number_of_tick_marks: NSInteger);

        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tick_mark_position: NSTickMarkPosition);

        #[method(allowsTickMarkValuesOnly)]
        pub unsafe fn allowsTickMarkValuesOnly(&self) -> bool;

        #[method(setAllowsTickMarkValuesOnly:)]
        pub unsafe fn setAllowsTickMarkValuesOnly(&self, allows_tick_mark_values_only: bool);

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;

        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[method(indexOfTickMarkAtPoint:)]
        pub unsafe fn indexOfTickMarkAtPoint(&self, point: NSPoint) -> NSInteger;

        #[method(closestTickMarkValueToValue:)]
        pub unsafe fn closestTickMarkValueToValue(&self, value: c_double) -> c_double;
    }
);

extern_methods!(
    /// NSSliderConvenience
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method_id(@__retain_semantics Other sliderWithTarget:action:)]
        pub unsafe fn sliderWithTarget_action(
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other sliderWithValue:minValue:maxValue:target:action:)]
        pub unsafe fn sliderWithValue_minValue_maxValue_target_action(
            value: c_double,
            min_value: c_double,
            max_value: c_double,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSSliderDeprecated
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[cfg(feature = "AppKit_NSCell")]
        #[deprecated = "-setTitleCell: had no effect since 10.0"]
        #[method(setTitleCell:)]
        pub unsafe fn setTitleCell(&self, cell: Option<&NSCell>);

        #[deprecated = "-titleCell has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other titleCell)]
        pub unsafe fn titleCell(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[deprecated = "-setTitleColor: had no effect since 10.0"]
        #[method(setTitleColor:)]
        pub unsafe fn setTitleColor(&self, new_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        #[deprecated = "-titleColor has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other titleColor)]
        pub unsafe fn titleColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "-setTitleFont: had no effect since 10.0"]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, font_obj: Option<&NSFont>);

        #[cfg(feature = "AppKit_NSFont")]
        #[deprecated = "-titleFont has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Option<Id<NSFont, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "-title has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "-setTitle: had no effect since 10.0"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: Option<&NSString>);

        #[deprecated = "-knobThickness has returned 0 since 10.0"]
        #[method(setKnobThickness:)]
        pub unsafe fn setKnobThickness(&self, thickness: CGFloat);

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated = "-setImage: had no effect since 10.0"]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, background_image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated = "-image has returned nil since 10.0"]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSlider")]
    unsafe impl NSSlider {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
