//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSProgressIndicatorStyle {
        NSProgressIndicatorStyleBar = 0,
        NSProgressIndicatorStyleSpinning = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSProgressIndicator")]
    pub struct NSProgressIndicator;

    #[cfg(feature = "AppKit_NSProgressIndicator")]
    unsafe impl ClassType for NSProgressIndicator {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSAccessibility for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSAccessibilityElementProtocol for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSAccessibilityGroup for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSAccessibilityProgressIndicator for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSAnimatablePropertyContainer for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSAppearanceCustomization for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSCoding for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSDraggingDestination for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSObjectProtocol for NSProgressIndicator {}

#[cfg(feature = "AppKit_NSProgressIndicator")]
unsafe impl NSUserInterfaceItemIdentification for NSProgressIndicator {}

extern_methods!(
    #[cfg(feature = "AppKit_NSProgressIndicator")]
    unsafe impl NSProgressIndicator {
        #[method(isIndeterminate)]
        pub unsafe fn isIndeterminate(&self) -> bool;

        #[method(setIndeterminate:)]
        pub unsafe fn setIndeterminate(&self, indeterminate: bool);

        #[method(isBezeled)]
        pub unsafe fn isBezeled(&self) -> bool;

        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, bezeled: bool);

        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(incrementBy:)]
        pub unsafe fn incrementBy(&self, delta: c_double);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(usesThreadedAnimation)]
        pub unsafe fn usesThreadedAnimation(&self) -> bool;

        #[method(setUsesThreadedAnimation:)]
        pub unsafe fn setUsesThreadedAnimation(&self, uses_threaded_animation: bool);

        #[method(startAnimation:)]
        pub unsafe fn startAnimation(&self, sender: Option<&Object>);

        #[method(stopAnimation:)]
        pub unsafe fn stopAnimation(&self, sender: Option<&Object>);

        #[method(style)]
        pub unsafe fn style(&self) -> NSProgressIndicatorStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSProgressIndicatorStyle);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(isDisplayedWhenStopped)]
        pub unsafe fn isDisplayedWhenStopped(&self) -> bool;

        #[method(setDisplayedWhenStopped:)]
        pub unsafe fn setDisplayedWhenStopped(&self, displayed_when_stopped: bool);
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
    pub enum NSProgressIndicatorThickness {
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredThickness = 14,
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredSmallThickness = 10,
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredLargeThickness = 18,
        #[deprecated = "These constants do not accurately represent the geometry of NSProgressIndicator.  Use `controlSize` and `sizeToFit` instead."]
        NSProgressIndicatorPreferredAquaThickness = 12,
    }
);

extern_static!(NSProgressIndicatorBarStyle: NSProgressIndicatorStyle = NSProgressIndicatorStyleBar);

extern_static!(
    NSProgressIndicatorSpinningStyle: NSProgressIndicatorStyle = NSProgressIndicatorStyleSpinning
);

extern_methods!(
    /// NSProgressIndicatorDeprecated
    #[cfg(feature = "AppKit_NSProgressIndicator")]
    unsafe impl NSProgressIndicator {
        #[deprecated = "The animationDelay property does nothing."]
        #[method(animationDelay)]
        pub unsafe fn animationDelay(&self) -> NSTimeInterval;

        #[deprecated = "The animationDelay property does nothing."]
        #[method(setAnimationDelay:)]
        pub unsafe fn setAnimationDelay(&self, delay: NSTimeInterval);

        #[deprecated = "Use -startAnimation and -stopAnimation instead."]
        #[method(animate:)]
        pub unsafe fn animate(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSProgressIndicator")]
    unsafe impl NSProgressIndicator {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
