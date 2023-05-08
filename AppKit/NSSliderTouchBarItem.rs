//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSSliderAccessoryWidth = CGFloat;
);

extern_static!(NSSliderAccessoryWidthDefault: NSSliderAccessoryWidth);

extern_static!(NSSliderAccessoryWidthWide: NSSliderAccessoryWidth);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSliderTouchBarItem")]
    pub struct NSSliderTouchBarItem;

    #[cfg(feature = "AppKit_NSSliderTouchBarItem")]
    unsafe impl ClassType for NSSliderTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSSliderTouchBarItem")]
unsafe impl NSCoding for NSSliderTouchBarItem {}

#[cfg(feature = "AppKit_NSSliderTouchBarItem")]
unsafe impl NSObjectProtocol for NSSliderTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSliderTouchBarItem")]
    unsafe impl NSSliderTouchBarItem {
        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Id<NSView>;

        #[cfg(feature = "AppKit_NSSlider")]
        #[method_id(@__retain_semantics Other slider)]
        pub unsafe fn slider(&self) -> Id<NSSlider>;

        #[cfg(feature = "AppKit_NSSlider")]
        #[method(setSlider:)]
        pub unsafe fn setSlider(&self, slider: &NSSlider);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(minimumSliderWidth)]
        pub unsafe fn minimumSliderWidth(&self) -> CGFloat;

        #[method(setMinimumSliderWidth:)]
        pub unsafe fn setMinimumSliderWidth(&self, minimum_slider_width: CGFloat);

        #[method(maximumSliderWidth)]
        pub unsafe fn maximumSliderWidth(&self) -> CGFloat;

        #[method(setMaximumSliderWidth:)]
        pub unsafe fn setMaximumSliderWidth(&self, maximum_slider_width: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "AppKit_NSSliderAccessory")]
        #[method_id(@__retain_semantics Other minimumValueAccessory)]
        pub unsafe fn minimumValueAccessory(&self) -> Option<Id<NSSliderAccessory>>;

        #[cfg(feature = "AppKit_NSSliderAccessory")]
        #[method(setMinimumValueAccessory:)]
        pub unsafe fn setMinimumValueAccessory(
            &self,
            minimum_value_accessory: Option<&NSSliderAccessory>,
        );

        #[cfg(feature = "AppKit_NSSliderAccessory")]
        #[method_id(@__retain_semantics Other maximumValueAccessory)]
        pub unsafe fn maximumValueAccessory(&self) -> Option<Id<NSSliderAccessory>>;

        #[cfg(feature = "AppKit_NSSliderAccessory")]
        #[method(setMaximumValueAccessory:)]
        pub unsafe fn setMaximumValueAccessory(
            &self,
            maximum_value_accessory: Option<&NSSliderAccessory>,
        );

        #[method(valueAccessoryWidth)]
        pub unsafe fn valueAccessoryWidth(&self) -> NSSliderAccessoryWidth;

        #[method(setValueAccessoryWidth:)]
        pub unsafe fn setValueAccessoryWidth(&self, value_accessory_width: NSSliderAccessoryWidth);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSSliderTouchBarItem")]
    unsafe impl NSSliderTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSliderTouchBarItem")]
    unsafe impl NSSliderTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
