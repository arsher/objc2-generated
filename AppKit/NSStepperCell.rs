//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStepperCell")]
    pub struct NSStepperCell;

    #[cfg(feature = "AppKit_NSStepperCell")]
    unsafe impl ClassType for NSStepperCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSStepperCell")]
unsafe impl NSAccessibility for NSStepperCell {}

#[cfg(feature = "AppKit_NSStepperCell")]
unsafe impl NSAccessibilityElementProtocol for NSStepperCell {}

#[cfg(feature = "AppKit_NSStepperCell")]
unsafe impl NSCoding for NSStepperCell {}

#[cfg(feature = "AppKit_NSStepperCell")]
unsafe impl NSCopying for NSStepperCell {}

#[cfg(feature = "AppKit_NSStepperCell")]
unsafe impl NSObjectProtocol for NSStepperCell {}

#[cfg(feature = "AppKit_NSStepperCell")]
unsafe impl NSUserInterfaceItemIdentification for NSStepperCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSStepperCell")]
    unsafe impl NSStepperCell {
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;

        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);

        #[method(valueWraps)]
        pub unsafe fn valueWraps(&self) -> bool;

        #[method(setValueWraps:)]
        pub unsafe fn setValueWraps(&self, value_wraps: bool);

        #[method(autorepeat)]
        pub unsafe fn autorepeat(&self) -> bool;

        #[method(setAutorepeat:)]
        pub unsafe fn setAutorepeat(&self, autorepeat: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSStepperCell")]
    unsafe impl NSStepperCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSStepperCell")]
    unsafe impl NSStepperCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
