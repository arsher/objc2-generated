//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitouchphase?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITouchPhase(pub NSInteger);
impl UITouchPhase {
    #[doc(alias = "UITouchPhaseBegan")]
    pub const Began: Self = Self(0);
    #[doc(alias = "UITouchPhaseMoved")]
    pub const Moved: Self = Self(1);
    #[doc(alias = "UITouchPhaseStationary")]
    pub const Stationary: Self = Self(2);
    #[doc(alias = "UITouchPhaseEnded")]
    pub const Ended: Self = Self(3);
    #[doc(alias = "UITouchPhaseCancelled")]
    pub const Cancelled: Self = Self(4);
    #[doc(alias = "UITouchPhaseRegionEntered")]
    pub const RegionEntered: Self = Self(5);
    #[doc(alias = "UITouchPhaseRegionMoved")]
    pub const RegionMoved: Self = Self(6);
    #[doc(alias = "UITouchPhaseRegionExited")]
    pub const RegionExited: Self = Self(7);
}

unsafe impl Encode for UITouchPhase {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITouchPhase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiforcetouchcapability?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIForceTouchCapability(pub NSInteger);
impl UIForceTouchCapability {
    #[doc(alias = "UIForceTouchCapabilityUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "UIForceTouchCapabilityUnavailable")]
    pub const Unavailable: Self = Self(1);
    #[doc(alias = "UIForceTouchCapabilityAvailable")]
    pub const Available: Self = Self(2);
}

unsafe impl Encode for UIForceTouchCapability {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIForceTouchCapability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitouchtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITouchType(pub NSInteger);
impl UITouchType {
    #[doc(alias = "UITouchTypeDirect")]
    pub const Direct: Self = Self(0);
    #[doc(alias = "UITouchTypeIndirect")]
    pub const Indirect: Self = Self(1);
    #[doc(alias = "UITouchTypePencil")]
    pub const Pencil: Self = Self(2);
    #[doc(alias = "UITouchTypeStylus")]
    pub const Stylus: Self = Self(UITouchType::Pencil.0);
    #[doc(alias = "UITouchTypeIndirectPointer")]
    pub const IndirectPointer: Self = Self(3);
}

unsafe impl Encode for UITouchType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITouchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitouchproperties?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITouchProperties(pub NSInteger);
bitflags::bitflags! {
    impl UITouchProperties: NSInteger {
        const UITouchPropertyForce = 1<<0;
        const UITouchPropertyAzimuth = 1<<1;
        const UITouchPropertyAltitude = 1<<2;
        const UITouchPropertyLocation = 1<<3;
        const UITouchPropertyRoll = 1<<4;
    }
}

unsafe impl Encode for UITouchProperties {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITouchProperties {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitouch?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITouch;
);

unsafe impl NSObjectProtocol for UITouch {}

extern_methods!(
    unsafe impl UITouch {
        #[method(timestamp)]
        pub fn timestamp(&self) -> NSTimeInterval;

        #[method(phase)]
        pub fn phase(&self) -> UITouchPhase;

        #[method(tapCount)]
        pub fn tapCount(&self) -> NSUInteger;

        #[method(type)]
        pub fn r#type(&self) -> UITouchType;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(majorRadius)]
        pub unsafe fn majorRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(majorRadiusTolerance)]
        pub unsafe fn majorRadiusTolerance(&self) -> CGFloat;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "UIWindow"))]
        #[method_id(@__retain_semantics Other window)]
        pub fn window(&self) -> Option<Retained<UIWindow>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        pub fn view(&self) -> Option<Retained<UIView>>;

        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gestureRecognizers)]
        pub unsafe fn gestureRecognizers(&self) -> Option<Retained<NSArray<UIGestureRecognizer>>>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(locationInView:)]
        pub fn locationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(previousLocationInView:)]
        pub fn previousLocationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(preciseLocationInView:)]
        pub unsafe fn preciseLocationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(precisePreviousLocationInView:)]
        pub unsafe fn precisePreviousLocationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(force)]
        pub fn force(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(maximumPossibleForce)]
        pub fn maximumPossibleForce(&self) -> CGFloat;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(azimuthAngleInView:)]
        pub fn azimuthAngleInView(&self, view: Option<&UIView>) -> CGFloat;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method(azimuthUnitVectorInView:)]
        pub fn azimuthUnitVectorInView(&self, view: Option<&UIView>) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(altitudeAngle)]
        pub fn altitudeAngle(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other estimationUpdateIndex)]
        pub unsafe fn estimationUpdateIndex(&self) -> Option<Retained<NSNumber>>;

        #[method(estimatedProperties)]
        pub unsafe fn estimatedProperties(&self) -> UITouchProperties;

        #[method(estimatedPropertiesExpectingUpdates)]
        pub unsafe fn estimatedPropertiesExpectingUpdates(&self) -> UITouchProperties;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rollAngle)]
        pub fn rollAngle(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITouch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
