//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSliderAccessory;

    unsafe impl ClassType for NSSliderAccessory {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for NSSliderAccessory {}

unsafe impl NSObjectProtocol for NSSliderAccessory {}

extern_methods!(
    unsafe impl NSSliderAccessory {
        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other accessoryWithImage:)]
        pub unsafe fn accessoryWithImage(
            image: &NSImage,
            mtm: MainThreadMarker,
        ) -> Id<NSSliderAccessory>;

        #[method_id(@__retain_semantics Other behavior)]
        pub unsafe fn behavior(&self) -> Id<NSSliderAccessoryBehavior>;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: &NSSliderAccessoryBehavior);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSliderAccessory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    unsafe impl NSSliderAccessory {}
);

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibility for NSSliderAccessory {}

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibilityElementProtocol for NSSliderAccessory {}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSliderAccessoryBehavior;

    unsafe impl ClassType for NSSliderAccessoryBehavior {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for NSSliderAccessoryBehavior {}

unsafe impl NSCopying for NSSliderAccessoryBehavior {}

unsafe impl NSObjectProtocol for NSSliderAccessoryBehavior {}

extern_methods!(
    unsafe impl NSSliderAccessoryBehavior {
        #[method_id(@__retain_semantics Other automaticBehavior)]
        pub unsafe fn automaticBehavior(mtm: MainThreadMarker) -> Id<NSSliderAccessoryBehavior>;

        #[method_id(@__retain_semantics Other valueStepBehavior)]
        pub unsafe fn valueStepBehavior(mtm: MainThreadMarker) -> Id<NSSliderAccessoryBehavior>;

        #[method_id(@__retain_semantics Other valueResetBehavior)]
        pub unsafe fn valueResetBehavior(mtm: MainThreadMarker) -> Id<NSSliderAccessoryBehavior>;

        #[method_id(@__retain_semantics Other behaviorWithTarget:action:)]
        pub unsafe fn behaviorWithTarget_action(
            target: Option<&AnyObject>,
            action: Sel,
            mtm: MainThreadMarker,
        ) -> Id<NSSliderAccessoryBehavior>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other behaviorWithHandler:)]
        pub unsafe fn behaviorWithHandler(
            handler: &block2::Block<dyn Fn(NonNull<NSSliderAccessory>)>,
            mtm: MainThreadMarker,
        ) -> Id<NSSliderAccessoryBehavior>;

        #[method(handleAction:)]
        pub unsafe fn handleAction(&self, sender: &NSSliderAccessory);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSliderAccessoryBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
