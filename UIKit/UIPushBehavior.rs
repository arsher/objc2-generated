//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipushbehaviormode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPushBehaviorMode(pub NSInteger);
impl UIPushBehaviorMode {
    #[doc(alias = "UIPushBehaviorModeContinuous")]
    pub const Continuous: Self = Self(0);
    #[doc(alias = "UIPushBehaviorModeInstantaneous")]
    pub const Instantaneous: Self = Self(1);
}

unsafe impl Encode for UIPushBehaviorMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPushBehaviorMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipushbehavior?language=objc)
    #[unsafe(super(UIDynamicBehavior, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UIPushBehavior;
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UIPushBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIPushBehavior {
        #[method_id(@__retain_semantics Init initWithItems:mode:)]
        pub unsafe fn initWithItems_mode(
            this: Allocated<Self>,
            items: &NSArray<ProtocolObject<dyn UIDynamicItem>>,
            mode: UIPushBehaviorMode,
        ) -> Retained<Self>;

        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(targetOffsetFromCenterForItem:)]
        pub unsafe fn targetOffsetFromCenterForItem(
            &self,
            item: &ProtocolObject<dyn UIDynamicItem>,
        ) -> UIOffset;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setTargetOffsetFromCenter:forItem:)]
        pub unsafe fn setTargetOffsetFromCenter_forItem(
            &self,
            o: UIOffset,
            item: &ProtocolObject<dyn UIDynamicItem>,
        );

        #[method(mode)]
        pub unsafe fn mode(&self) -> UIPushBehaviorMode;

        #[method(active)]
        pub unsafe fn active(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(angle)]
        pub unsafe fn angle(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAngle:)]
        pub unsafe fn setAngle(&self, angle: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(magnitude)]
        pub unsafe fn magnitude(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setMagnitude:)]
        pub unsafe fn setMagnitude(&self, magnitude: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(pushDirection)]
        pub unsafe fn pushDirection(&self) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPushDirection:)]
        pub unsafe fn setPushDirection(&self, push_direction: CGVector);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAngle:magnitude:)]
        pub unsafe fn setAngle_magnitude(&self, angle: CGFloat, magnitude: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIPushBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
