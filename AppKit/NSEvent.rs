//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventType(pub NSUInteger);
impl NSEventType {
    #[doc(alias = "NSEventTypeLeftMouseDown")]
    pub const LeftMouseDown: Self = Self(1);
    #[doc(alias = "NSEventTypeLeftMouseUp")]
    pub const LeftMouseUp: Self = Self(2);
    #[doc(alias = "NSEventTypeRightMouseDown")]
    pub const RightMouseDown: Self = Self(3);
    #[doc(alias = "NSEventTypeRightMouseUp")]
    pub const RightMouseUp: Self = Self(4);
    #[doc(alias = "NSEventTypeMouseMoved")]
    pub const MouseMoved: Self = Self(5);
    #[doc(alias = "NSEventTypeLeftMouseDragged")]
    pub const LeftMouseDragged: Self = Self(6);
    #[doc(alias = "NSEventTypeRightMouseDragged")]
    pub const RightMouseDragged: Self = Self(7);
    #[doc(alias = "NSEventTypeMouseEntered")]
    pub const MouseEntered: Self = Self(8);
    #[doc(alias = "NSEventTypeMouseExited")]
    pub const MouseExited: Self = Self(9);
    #[doc(alias = "NSEventTypeKeyDown")]
    pub const KeyDown: Self = Self(10);
    #[doc(alias = "NSEventTypeKeyUp")]
    pub const KeyUp: Self = Self(11);
    #[doc(alias = "NSEventTypeFlagsChanged")]
    pub const FlagsChanged: Self = Self(12);
    #[doc(alias = "NSEventTypeAppKitDefined")]
    pub const AppKitDefined: Self = Self(13);
    #[doc(alias = "NSEventTypeSystemDefined")]
    pub const SystemDefined: Self = Self(14);
    #[doc(alias = "NSEventTypeApplicationDefined")]
    pub const ApplicationDefined: Self = Self(15);
    #[doc(alias = "NSEventTypePeriodic")]
    pub const Periodic: Self = Self(16);
    #[doc(alias = "NSEventTypeCursorUpdate")]
    pub const CursorUpdate: Self = Self(17);
    #[doc(alias = "NSEventTypeScrollWheel")]
    pub const ScrollWheel: Self = Self(22);
    #[doc(alias = "NSEventTypeTabletPoint")]
    pub const TabletPoint: Self = Self(23);
    #[doc(alias = "NSEventTypeTabletProximity")]
    pub const TabletProximity: Self = Self(24);
    #[doc(alias = "NSEventTypeOtherMouseDown")]
    pub const OtherMouseDown: Self = Self(25);
    #[doc(alias = "NSEventTypeOtherMouseUp")]
    pub const OtherMouseUp: Self = Self(26);
    #[doc(alias = "NSEventTypeOtherMouseDragged")]
    pub const OtherMouseDragged: Self = Self(27);
    #[doc(alias = "NSEventTypeGesture")]
    pub const Gesture: Self = Self(29);
    #[doc(alias = "NSEventTypeMagnify")]
    pub const Magnify: Self = Self(30);
    #[doc(alias = "NSEventTypeSwipe")]
    pub const Swipe: Self = Self(31);
    #[doc(alias = "NSEventTypeRotate")]
    pub const Rotate: Self = Self(18);
    #[doc(alias = "NSEventTypeBeginGesture")]
    pub const BeginGesture: Self = Self(19);
    #[doc(alias = "NSEventTypeEndGesture")]
    pub const EndGesture: Self = Self(20);
    #[doc(alias = "NSEventTypeSmartMagnify")]
    pub const SmartMagnify: Self = Self(32);
    #[doc(alias = "NSEventTypeQuickLook")]
    pub const QuickLook: Self = Self(33);
    #[doc(alias = "NSEventTypePressure")]
    pub const Pressure: Self = Self(34);
    #[doc(alias = "NSEventTypeDirectTouch")]
    pub const DirectTouch: Self = Self(37);
    #[doc(alias = "NSEventTypeChangeMode")]
    pub const ChangeMode: Self = Self(38);
}

unsafe impl Encode for NSEventType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEventType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSLeftMouseDown: NSEventType = NSEventType(NSEventType::LeftMouseDown.0);

pub static NSLeftMouseUp: NSEventType = NSEventType(NSEventType::LeftMouseUp.0);

pub static NSRightMouseDown: NSEventType = NSEventType(NSEventType::RightMouseDown.0);

pub static NSRightMouseUp: NSEventType = NSEventType(NSEventType::RightMouseUp.0);

pub static NSMouseMoved: NSEventType = NSEventType(NSEventType::MouseMoved.0);

pub static NSLeftMouseDragged: NSEventType = NSEventType(NSEventType::LeftMouseDragged.0);

pub static NSRightMouseDragged: NSEventType = NSEventType(NSEventType::RightMouseDragged.0);

pub static NSMouseEntered: NSEventType = NSEventType(NSEventType::MouseEntered.0);

pub static NSMouseExited: NSEventType = NSEventType(NSEventType::MouseExited.0);

pub static NSKeyDown: NSEventType = NSEventType(NSEventType::KeyDown.0);

pub static NSKeyUp: NSEventType = NSEventType(NSEventType::KeyUp.0);

pub static NSFlagsChanged: NSEventType = NSEventType(NSEventType::FlagsChanged.0);

pub static NSAppKitDefined: NSEventType = NSEventType(NSEventType::AppKitDefined.0);

pub static NSSystemDefined: NSEventType = NSEventType(NSEventType::SystemDefined.0);

pub static NSApplicationDefined: NSEventType = NSEventType(NSEventType::ApplicationDefined.0);

pub static NSPeriodic: NSEventType = NSEventType(NSEventType::Periodic.0);

pub static NSCursorUpdate: NSEventType = NSEventType(NSEventType::CursorUpdate.0);

pub static NSScrollWheel: NSEventType = NSEventType(NSEventType::ScrollWheel.0);

pub static NSTabletPoint: NSEventType = NSEventType(NSEventType::TabletPoint.0);

pub static NSTabletProximity: NSEventType = NSEventType(NSEventType::TabletProximity.0);

pub static NSOtherMouseDown: NSEventType = NSEventType(NSEventType::OtherMouseDown.0);

pub static NSOtherMouseUp: NSEventType = NSEventType(NSEventType::OtherMouseUp.0);

pub static NSOtherMouseDragged: NSEventType = NSEventType(NSEventType::OtherMouseDragged.0);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventMask(pub c_ulonglong);
bitflags::bitflags! {
    impl NSEventMask: c_ulonglong {
        #[doc(alias = "NSEventMaskLeftMouseDown")]
        const LeftMouseDown = 1<<NSEventType::LeftMouseDown.0;
        #[doc(alias = "NSEventMaskLeftMouseUp")]
        const LeftMouseUp = 1<<NSEventType::LeftMouseUp.0;
        #[doc(alias = "NSEventMaskRightMouseDown")]
        const RightMouseDown = 1<<NSEventType::RightMouseDown.0;
        #[doc(alias = "NSEventMaskRightMouseUp")]
        const RightMouseUp = 1<<NSEventType::RightMouseUp.0;
        #[doc(alias = "NSEventMaskMouseMoved")]
        const MouseMoved = 1<<NSEventType::MouseMoved.0;
        #[doc(alias = "NSEventMaskLeftMouseDragged")]
        const LeftMouseDragged = 1<<NSEventType::LeftMouseDragged.0;
        #[doc(alias = "NSEventMaskRightMouseDragged")]
        const RightMouseDragged = 1<<NSEventType::RightMouseDragged.0;
        #[doc(alias = "NSEventMaskMouseEntered")]
        const MouseEntered = 1<<NSEventType::MouseEntered.0;
        #[doc(alias = "NSEventMaskMouseExited")]
        const MouseExited = 1<<NSEventType::MouseExited.0;
        #[doc(alias = "NSEventMaskKeyDown")]
        const KeyDown = 1<<NSEventType::KeyDown.0;
        #[doc(alias = "NSEventMaskKeyUp")]
        const KeyUp = 1<<NSEventType::KeyUp.0;
        #[doc(alias = "NSEventMaskFlagsChanged")]
        const FlagsChanged = 1<<NSEventType::FlagsChanged.0;
        #[doc(alias = "NSEventMaskAppKitDefined")]
        const AppKitDefined = 1<<NSEventType::AppKitDefined.0;
        #[doc(alias = "NSEventMaskSystemDefined")]
        const SystemDefined = 1<<NSEventType::SystemDefined.0;
        #[doc(alias = "NSEventMaskApplicationDefined")]
        const ApplicationDefined = 1<<NSEventType::ApplicationDefined.0;
        #[doc(alias = "NSEventMaskPeriodic")]
        const Periodic = 1<<NSEventType::Periodic.0;
        #[doc(alias = "NSEventMaskCursorUpdate")]
        const CursorUpdate = 1<<NSEventType::CursorUpdate.0;
        #[doc(alias = "NSEventMaskScrollWheel")]
        const ScrollWheel = 1<<NSEventType::ScrollWheel.0;
        #[doc(alias = "NSEventMaskTabletPoint")]
        const TabletPoint = 1<<NSEventType::TabletPoint.0;
        #[doc(alias = "NSEventMaskTabletProximity")]
        const TabletProximity = 1<<NSEventType::TabletProximity.0;
        #[doc(alias = "NSEventMaskOtherMouseDown")]
        const OtherMouseDown = 1<<NSEventType::OtherMouseDown.0;
        #[doc(alias = "NSEventMaskOtherMouseUp")]
        const OtherMouseUp = 1<<NSEventType::OtherMouseUp.0;
        #[doc(alias = "NSEventMaskOtherMouseDragged")]
        const OtherMouseDragged = 1<<NSEventType::OtherMouseDragged.0;
        #[doc(alias = "NSEventMaskGesture")]
        const Gesture = 1<<NSEventType::Gesture.0;
        #[doc(alias = "NSEventMaskMagnify")]
        const Magnify = 1<<NSEventType::Magnify.0;
        #[doc(alias = "NSEventMaskSwipe")]
        const Swipe = 1<<NSEventType::Swipe.0;
        #[doc(alias = "NSEventMaskRotate")]
        const Rotate = 1<<NSEventType::Rotate.0;
        #[doc(alias = "NSEventMaskBeginGesture")]
        const BeginGesture = 1<<NSEventType::BeginGesture.0;
        #[doc(alias = "NSEventMaskEndGesture")]
        const EndGesture = 1<<NSEventType::EndGesture.0;
        #[doc(alias = "NSEventMaskSmartMagnify")]
        const SmartMagnify = 1<<NSEventType::SmartMagnify.0;
        #[doc(alias = "NSEventMaskPressure")]
        const Pressure = 1<<NSEventType::Pressure.0;
        #[doc(alias = "NSEventMaskDirectTouch")]
        const DirectTouch = 1<<NSEventType::DirectTouch.0;
        #[doc(alias = "NSEventMaskChangeMode")]
        const ChangeMode = 1<<NSEventType::ChangeMode.0;
        #[doc(alias = "NSEventMaskAny")]
        const Any = NSUIntegerMax as _;
    }
}

unsafe impl Encode for NSEventMask {
    const ENCODING: Encoding = c_ulonglong::ENCODING;
}

unsafe impl RefEncode for NSEventMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSLeftMouseDownMask: NSEventMask = NSEventMask(NSEventMask::LeftMouseDown.0);

pub static NSLeftMouseUpMask: NSEventMask = NSEventMask(NSEventMask::LeftMouseUp.0);

pub static NSRightMouseDownMask: NSEventMask = NSEventMask(NSEventMask::RightMouseDown.0);

pub static NSRightMouseUpMask: NSEventMask = NSEventMask(NSEventMask::RightMouseUp.0);

pub static NSMouseMovedMask: NSEventMask = NSEventMask(NSEventMask::MouseMoved.0);

pub static NSLeftMouseDraggedMask: NSEventMask = NSEventMask(NSEventMask::LeftMouseDragged.0);

pub static NSRightMouseDraggedMask: NSEventMask = NSEventMask(NSEventMask::RightMouseDragged.0);

pub static NSMouseEnteredMask: NSEventMask = NSEventMask(NSEventMask::MouseEntered.0);

pub static NSMouseExitedMask: NSEventMask = NSEventMask(NSEventMask::MouseExited.0);

pub static NSKeyDownMask: NSEventMask = NSEventMask(NSEventMask::KeyDown.0);

pub static NSKeyUpMask: NSEventMask = NSEventMask(NSEventMask::KeyUp.0);

pub static NSFlagsChangedMask: NSEventMask = NSEventMask(NSEventMask::FlagsChanged.0);

pub static NSAppKitDefinedMask: NSEventMask = NSEventMask(NSEventMask::AppKitDefined.0);

pub static NSSystemDefinedMask: NSEventMask = NSEventMask(NSEventMask::SystemDefined.0);

pub static NSApplicationDefinedMask: NSEventMask = NSEventMask(NSEventMask::ApplicationDefined.0);

pub static NSPeriodicMask: NSEventMask = NSEventMask(NSEventMask::Periodic.0);

pub static NSCursorUpdateMask: NSEventMask = NSEventMask(NSEventMask::CursorUpdate.0);

pub static NSScrollWheelMask: NSEventMask = NSEventMask(NSEventMask::ScrollWheel.0);

pub static NSTabletPointMask: NSEventMask = NSEventMask(NSEventMask::TabletPoint.0);

pub static NSTabletProximityMask: NSEventMask = NSEventMask(NSEventMask::TabletProximity.0);

pub static NSOtherMouseDownMask: NSEventMask = NSEventMask(NSEventMask::OtherMouseDown.0);

pub static NSOtherMouseUpMask: NSEventMask = NSEventMask(NSEventMask::OtherMouseUp.0);

pub static NSOtherMouseDraggedMask: NSEventMask = NSEventMask(NSEventMask::OtherMouseDragged.0);

pub static NSAnyEventMask: NSEventMask = NSEventMask(NSUIntegerMax as _);

// TODO: pub fn NSEventMaskFromType(r#type: NSEventType,) -> NSEventMask;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventModifierFlags(pub NSUInteger);
bitflags::bitflags! {
    impl NSEventModifierFlags: NSUInteger {
        const NSEventModifierFlagCapsLock = 1<<16;
        const NSEventModifierFlagShift = 1<<17;
        const NSEventModifierFlagControl = 1<<18;
        const NSEventModifierFlagOption = 1<<19;
        const NSEventModifierFlagCommand = 1<<20;
        const NSEventModifierFlagNumericPad = 1<<21;
        const NSEventModifierFlagHelp = 1<<22;
        const NSEventModifierFlagFunction = 1<<23;
        const NSEventModifierFlagDeviceIndependentFlagsMask = 0xffff0000;
    }
}

unsafe impl Encode for NSEventModifierFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEventModifierFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSAlphaShiftKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagCapsLock.0);

pub static NSShiftKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagShift.0);

pub static NSControlKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagControl.0);

pub static NSAlternateKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagOption.0);

pub static NSCommandKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagCommand.0);

pub static NSNumericPadKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagNumericPad.0);

pub static NSHelpKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagHelp.0);

pub static NSFunctionKeyMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagFunction.0);

pub static NSDeviceIndependentModifierFlagsMask: NSEventModifierFlags =
    NSEventModifierFlags(NSEventModifierFlags::NSEventModifierFlagDeviceIndependentFlagsMask.0);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPointingDeviceType(pub NSUInteger);
impl NSPointingDeviceType {
    #[doc(alias = "NSPointingDeviceTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "NSPointingDeviceTypePen")]
    pub const Pen: Self = Self(1);
    #[doc(alias = "NSPointingDeviceTypeCursor")]
    pub const Cursor: Self = Self(2);
    #[doc(alias = "NSPointingDeviceTypeEraser")]
    pub const Eraser: Self = Self(3);
}

unsafe impl Encode for NSPointingDeviceType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPointingDeviceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSUnknownPointingDevice: NSPointingDeviceType =
    NSPointingDeviceType(NSPointingDeviceType::Unknown.0);

pub static NSPenPointingDevice: NSPointingDeviceType =
    NSPointingDeviceType(NSPointingDeviceType::Pen.0);

pub static NSCursorPointingDevice: NSPointingDeviceType =
    NSPointingDeviceType(NSPointingDeviceType::Cursor.0);

pub static NSEraserPointingDevice: NSPointingDeviceType =
    NSPointingDeviceType(NSPointingDeviceType::Eraser.0);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventButtonMask(pub NSUInteger);
bitflags::bitflags! {
    impl NSEventButtonMask: NSUInteger {
        #[doc(alias = "NSEventButtonMaskPenTip")]
        const PenTip = 1;
        #[doc(alias = "NSEventButtonMaskPenLowerSide")]
        const PenLowerSide = 2;
        #[doc(alias = "NSEventButtonMaskPenUpperSide")]
        const PenUpperSide = 4;
    }
}

unsafe impl Encode for NSEventButtonMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEventButtonMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSPenTipMask: NSEventButtonMask = NSEventButtonMask(NSEventButtonMask::PenTip.0);

pub static NSPenLowerSideMask: NSEventButtonMask =
    NSEventButtonMask(NSEventButtonMask::PenLowerSide.0);

pub static NSPenUpperSideMask: NSEventButtonMask =
    NSEventButtonMask(NSEventButtonMask::PenUpperSide.0);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventPhase(pub NSUInteger);
bitflags::bitflags! {
    impl NSEventPhase: NSUInteger {
        #[doc(alias = "NSEventPhaseNone")]
        const None = 0;
        #[doc(alias = "NSEventPhaseBegan")]
        const Began = 0x1<<0;
        #[doc(alias = "NSEventPhaseStationary")]
        const Stationary = 0x1<<1;
        #[doc(alias = "NSEventPhaseChanged")]
        const Changed = 0x1<<2;
        #[doc(alias = "NSEventPhaseEnded")]
        const Ended = 0x1<<3;
        #[doc(alias = "NSEventPhaseCancelled")]
        const Cancelled = 0x1<<4;
        #[doc(alias = "NSEventPhaseMayBegin")]
        const MayBegin = 0x1<<5;
    }
}

unsafe impl Encode for NSEventPhase {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEventPhase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventGestureAxis(pub NSInteger);
impl NSEventGestureAxis {
    #[doc(alias = "NSEventGestureAxisNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSEventGestureAxisHorizontal")]
    pub const Horizontal: Self = Self(1);
    #[doc(alias = "NSEventGestureAxisVertical")]
    pub const Vertical: Self = Self(2);
}

unsafe impl Encode for NSEventGestureAxis {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSEventGestureAxis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventSwipeTrackingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSEventSwipeTrackingOptions: NSUInteger {
        const NSEventSwipeTrackingLockDirection = 0x1<<0;
        const NSEventSwipeTrackingClampGestureAmount = 0x1<<1;
    }
}

unsafe impl Encode for NSEventSwipeTrackingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSEventSwipeTrackingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSEventSubtype(pub c_short);
impl NSEventSubtype {
    #[doc(alias = "NSEventSubtypeWindowExposed")]
    pub const WindowExposed: Self = Self(0);
    #[doc(alias = "NSEventSubtypeApplicationActivated")]
    pub const ApplicationActivated: Self = Self(1);
    #[doc(alias = "NSEventSubtypeApplicationDeactivated")]
    pub const ApplicationDeactivated: Self = Self(2);
    #[doc(alias = "NSEventSubtypeWindowMoved")]
    pub const WindowMoved: Self = Self(4);
    #[doc(alias = "NSEventSubtypeScreenChanged")]
    pub const ScreenChanged: Self = Self(8);
    #[doc(alias = "NSEventSubtypePowerOff")]
    pub const PowerOff: Self = Self(1);
    #[doc(alias = "NSEventSubtypeMouseEvent")]
    pub const MouseEvent: Self = Self(0);
    #[doc(alias = "NSEventSubtypeTabletPoint")]
    pub const TabletPoint: Self = Self(1);
    #[doc(alias = "NSEventSubtypeTabletProximity")]
    pub const TabletProximity: Self = Self(2);
    #[doc(alias = "NSEventSubtypeTouch")]
    pub const Touch: Self = Self(3);
}

unsafe impl Encode for NSEventSubtype {
    const ENCODING: Encoding = c_short::ENCODING;
}

unsafe impl RefEncode for NSEventSubtype {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSWindowExposedEventType: NSEventSubtype =
    NSEventSubtype(NSEventSubtype::WindowExposed.0);

pub static NSApplicationActivatedEventType: NSEventSubtype =
    NSEventSubtype(NSEventSubtype::ApplicationActivated.0);

pub static NSApplicationDeactivatedEventType: NSEventSubtype =
    NSEventSubtype(NSEventSubtype::ApplicationDeactivated.0);

pub static NSWindowMovedEventType: NSEventSubtype = NSEventSubtype(NSEventSubtype::WindowMoved.0);

pub static NSScreenChangedEventType: NSEventSubtype =
    NSEventSubtype(NSEventSubtype::ScreenChanged.0);

pub static NSAWTEventType: NSEventSubtype = NSEventSubtype(16);

pub static NSPowerOffEventType: NSEventSubtype = NSEventSubtype(NSEventSubtype::PowerOff.0);

pub static NSMouseEventSubtype: NSEventSubtype = NSEventSubtype(NSEventSubtype::MouseEvent.0);

pub static NSTabletPointEventSubtype: NSEventSubtype =
    NSEventSubtype(NSEventSubtype::TabletPoint.0);

pub static NSTabletProximityEventSubtype: NSEventSubtype =
    NSEventSubtype(NSEventSubtype::TabletProximity.0);

pub static NSTouchEventSubtype: NSEventSubtype = NSEventSubtype(NSEventSubtype::Touch.0);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPressureBehavior(pub NSInteger);
impl NSPressureBehavior {
    #[doc(alias = "NSPressureBehaviorUnknown")]
    pub const Unknown: Self = Self(-1);
    #[doc(alias = "NSPressureBehaviorPrimaryDefault")]
    pub const PrimaryDefault: Self = Self(0);
    #[doc(alias = "NSPressureBehaviorPrimaryClick")]
    pub const PrimaryClick: Self = Self(1);
    #[doc(alias = "NSPressureBehaviorPrimaryGeneric")]
    pub const PrimaryGeneric: Self = Self(2);
    #[doc(alias = "NSPressureBehaviorPrimaryAccelerator")]
    pub const PrimaryAccelerator: Self = Self(3);
    #[doc(alias = "NSPressureBehaviorPrimaryDeepClick")]
    pub const PrimaryDeepClick: Self = Self(5);
    #[doc(alias = "NSPressureBehaviorPrimaryDeepDrag")]
    pub const PrimaryDeepDrag: Self = Self(6);
}

unsafe impl Encode for NSPressureBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPressureBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEvent;

    unsafe impl ClassType for NSEvent {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

unsafe impl NSCoding for NSEvent {}

unsafe impl NSCopying for NSEvent {}

unsafe impl NSObjectProtocol for NSEvent {}

extern_methods!(
    unsafe impl NSEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSEventType;

        #[method(modifierFlags)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self, mtm: MainThreadMarker) -> Option<Id<NSWindow>>;

        #[method(windowNumber)]
        pub unsafe fn windowNumber(&self) -> NSInteger;

        #[cfg(feature = "NSGraphicsContext")]
        #[deprecated = "This method always returns nil. If you need access to the current drawing context, use [NSGraphicsContext currentContext] inside of a draw operation."]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext>>;

        #[method(clickCount)]
        pub unsafe fn clickCount(&self) -> NSInteger;

        #[method(buttonNumber)]
        pub unsafe fn buttonNumber(&self) -> NSInteger;

        #[method(eventNumber)]
        pub unsafe fn eventNumber(&self) -> NSInteger;

        #[method(pressure)]
        pub unsafe fn pressure(&self) -> c_float;

        #[method(locationInWindow)]
        pub unsafe fn locationInWindow(&self) -> NSPoint;

        #[method(deltaX)]
        pub unsafe fn deltaX(&self) -> CGFloat;

        #[method(deltaY)]
        pub unsafe fn deltaY(&self) -> CGFloat;

        #[method(deltaZ)]
        pub unsafe fn deltaZ(&self) -> CGFloat;

        #[method(hasPreciseScrollingDeltas)]
        pub unsafe fn hasPreciseScrollingDeltas(&self) -> bool;

        #[method(scrollingDeltaX)]
        pub unsafe fn scrollingDeltaX(&self) -> CGFloat;

        #[method(scrollingDeltaY)]
        pub unsafe fn scrollingDeltaY(&self) -> CGFloat;

        #[method(momentumPhase)]
        pub unsafe fn momentumPhase(&self) -> NSEventPhase;

        #[method(isDirectionInvertedFromDevice)]
        pub unsafe fn isDirectionInvertedFromDevice(&self) -> bool;

        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other charactersIgnoringModifiers)]
        pub unsafe fn charactersIgnoringModifiers(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other charactersByApplyingModifiers:)]
        pub unsafe fn charactersByApplyingModifiers(
            &self,
            modifiers: NSEventModifierFlags,
        ) -> Option<Id<NSString>>;

        #[method(isARepeat)]
        pub unsafe fn isARepeat(&self) -> bool;

        #[method(keyCode)]
        pub unsafe fn keyCode(&self) -> c_ushort;

        #[method(trackingNumber)]
        pub unsafe fn trackingNumber(&self) -> NSInteger;

        #[method(userData)]
        pub unsafe fn userData(&self) -> *mut c_void;

        #[cfg(feature = "NSTrackingArea")]
        #[method_id(@__retain_semantics Other trackingArea)]
        pub unsafe fn trackingArea(&self) -> Option<Id<NSTrackingArea>>;

        #[method(subtype)]
        pub unsafe fn subtype(&self) -> NSEventSubtype;

        #[method(data1)]
        pub unsafe fn data1(&self) -> NSInteger;

        #[method(data2)]
        pub unsafe fn data2(&self) -> NSInteger;

        #[method(eventRef)]
        pub unsafe fn eventRef(&self) -> *mut c_void;

        #[method_id(@__retain_semantics Other eventWithEventRef:)]
        pub unsafe fn eventWithEventRef(event_ref: NonNull<c_void>) -> Option<Id<NSEvent>>;

        #[method(isMouseCoalescingEnabled)]
        pub unsafe fn isMouseCoalescingEnabled() -> bool;

        #[method(setMouseCoalescingEnabled:)]
        pub unsafe fn setMouseCoalescingEnabled(mouse_coalescing_enabled: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(deviceID)]
        pub unsafe fn deviceID(&self) -> NSUInteger;

        #[method(rotation)]
        pub unsafe fn rotation(&self) -> c_float;

        #[method(absoluteX)]
        pub unsafe fn absoluteX(&self) -> NSInteger;

        #[method(absoluteY)]
        pub unsafe fn absoluteY(&self) -> NSInteger;

        #[method(absoluteZ)]
        pub unsafe fn absoluteZ(&self) -> NSInteger;

        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSEventButtonMask;

        #[method(tilt)]
        pub unsafe fn tilt(&self) -> NSPoint;

        #[method(tangentialPressure)]
        pub unsafe fn tangentialPressure(&self) -> c_float;

        #[method_id(@__retain_semantics Other vendorDefined)]
        pub unsafe fn vendorDefined(&self) -> Id<AnyObject>;

        #[method(vendorID)]
        pub unsafe fn vendorID(&self) -> NSUInteger;

        #[method(tabletID)]
        pub unsafe fn tabletID(&self) -> NSUInteger;

        #[method(pointingDeviceID)]
        pub unsafe fn pointingDeviceID(&self) -> NSUInteger;

        #[method(systemTabletID)]
        pub unsafe fn systemTabletID(&self) -> NSUInteger;

        #[method(vendorPointingDeviceType)]
        pub unsafe fn vendorPointingDeviceType(&self) -> NSUInteger;

        #[method(pointingDeviceSerialNumber)]
        pub unsafe fn pointingDeviceSerialNumber(&self) -> NSUInteger;

        #[method(uniqueID)]
        pub unsafe fn uniqueID(&self) -> c_ulonglong;

        #[method(capabilityMask)]
        pub unsafe fn capabilityMask(&self) -> NSUInteger;

        #[method(pointingDeviceType)]
        pub unsafe fn pointingDeviceType(&self) -> NSPointingDeviceType;

        #[method(isEnteringProximity)]
        pub unsafe fn isEnteringProximity(&self) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSTouch", feature = "NSView"))]
        #[method_id(@__retain_semantics Other touchesMatchingPhase:inView:)]
        pub unsafe fn touchesMatchingPhase_inView(
            &self,
            phase: NSTouchPhase,
            view: Option<&NSView>,
        ) -> Id<NSSet<NSTouch>>;

        #[cfg(feature = "NSTouch")]
        #[method_id(@__retain_semantics Other allTouches)]
        pub unsafe fn allTouches(&self) -> Id<NSSet<NSTouch>>;

        #[cfg(all(feature = "NSResponder", feature = "NSTouch", feature = "NSView"))]
        #[method_id(@__retain_semantics Other touchesForView:)]
        pub unsafe fn touchesForView(&self, view: &NSView) -> Id<NSSet<NSTouch>>;

        #[cfg(feature = "NSTouch")]
        #[method_id(@__retain_semantics Other coalescedTouchesForTouch:)]
        pub unsafe fn coalescedTouchesForTouch(&self, touch: &NSTouch) -> Id<NSArray<NSTouch>>;

        #[method(phase)]
        pub unsafe fn phase(&self) -> NSEventPhase;

        #[method(stage)]
        pub unsafe fn stage(&self) -> NSInteger;

        #[method(stageTransition)]
        pub unsafe fn stageTransition(&self) -> CGFloat;

        #[method(associatedEventsMask)]
        pub unsafe fn associatedEventsMask(&self) -> NSEventMask;

        #[method(pressureBehavior)]
        pub unsafe fn pressureBehavior(&self) -> NSPressureBehavior;

        #[method(isSwipeTrackingFromScrollEventsEnabled)]
        pub unsafe fn isSwipeTrackingFromScrollEventsEnabled() -> bool;

        #[cfg(feature = "block2")]
        #[method(trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler:)]
        pub unsafe fn trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler(
            &self,
            options: NSEventSwipeTrackingOptions,
            min_dampen_threshold: CGFloat,
            max_dampen_threshold: CGFloat,
            tracking_handler: &block2::Block<dyn Fn(CGFloat, NSEventPhase, Bool, NonNull<Bool>)>,
        );

        #[method(startPeriodicEventsAfterDelay:withPeriod:)]
        pub unsafe fn startPeriodicEventsAfterDelay_withPeriod(
            delay: NSTimeInterval,
            period: NSTimeInterval,
        );

        #[method(stopPeriodicEvents)]
        pub unsafe fn stopPeriodicEvents();

        #[cfg(feature = "NSGraphicsContext")]
        #[method_id(@__retain_semantics Other mouseEventWithType:location:modifierFlags:timestamp:windowNumber:context:eventNumber:clickCount:pressure:)]
        pub unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            e_num: NSInteger,
            c_num: NSInteger,
            pressure: c_float,
        ) -> Option<Id<NSEvent>>;

        #[cfg(feature = "NSGraphicsContext")]
        #[method_id(@__retain_semantics Other keyEventWithType:location:modifierFlags:timestamp:windowNumber:context:characters:charactersIgnoringModifiers:isARepeat:keyCode:)]
        pub unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            keys: &NSString,
            ukeys: &NSString,
            flag: bool,
            code: c_ushort,
        ) -> Option<Id<NSEvent>>;

        #[cfg(feature = "NSGraphicsContext")]
        #[method_id(@__retain_semantics Other enterExitEventWithType:location:modifierFlags:timestamp:windowNumber:context:eventNumber:trackingNumber:userData:)]
        pub unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            e_num: NSInteger,
            t_num: NSInteger,
            data: *mut c_void,
        ) -> Option<Id<NSEvent>>;

        #[cfg(feature = "NSGraphicsContext")]
        #[method_id(@__retain_semantics Other otherEventWithType:location:modifierFlags:timestamp:windowNumber:context:subtype:data1:data2:)]
        pub unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
            r#type: NSEventType,
            location: NSPoint,
            flags: NSEventModifierFlags,
            time: NSTimeInterval,
            w_num: NSInteger,
            unused_pass_nil: Option<&NSGraphicsContext>,
            subtype: c_short,
            d1: NSInteger,
            d2: NSInteger,
        ) -> Option<Id<NSEvent>>;

        #[method(mouseLocation)]
        pub unsafe fn mouseLocation() -> NSPoint;

        #[method(modifierFlags)]
        pub unsafe fn modifierFlags_class() -> NSEventModifierFlags;

        #[method(pressedMouseButtons)]
        pub unsafe fn pressedMouseButtons() -> NSUInteger;

        #[method(doubleClickInterval)]
        pub unsafe fn doubleClickInterval() -> NSTimeInterval;

        #[method(keyRepeatDelay)]
        pub unsafe fn keyRepeatDelay() -> NSTimeInterval;

        #[method(keyRepeatInterval)]
        pub unsafe fn keyRepeatInterval() -> NSTimeInterval;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other addGlobalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addGlobalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: &block2::Block<dyn Fn(NonNull<NSEvent>)>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other addLocalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addLocalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: &block2::Block<dyn Fn(NonNull<NSEvent>) -> *mut NSEvent>,
        ) -> Option<Id<AnyObject>>;

        #[method(removeMonitor:)]
        pub unsafe fn removeMonitor(event_monitor: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub const NSUpArrowFunctionKey: c_uint = 0xF700;
pub const NSDownArrowFunctionKey: c_uint = 0xF701;
pub const NSLeftArrowFunctionKey: c_uint = 0xF702;
pub const NSRightArrowFunctionKey: c_uint = 0xF703;
pub const NSF1FunctionKey: c_uint = 0xF704;
pub const NSF2FunctionKey: c_uint = 0xF705;
pub const NSF3FunctionKey: c_uint = 0xF706;
pub const NSF4FunctionKey: c_uint = 0xF707;
pub const NSF5FunctionKey: c_uint = 0xF708;
pub const NSF6FunctionKey: c_uint = 0xF709;
pub const NSF7FunctionKey: c_uint = 0xF70A;
pub const NSF8FunctionKey: c_uint = 0xF70B;
pub const NSF9FunctionKey: c_uint = 0xF70C;
pub const NSF10FunctionKey: c_uint = 0xF70D;
pub const NSF11FunctionKey: c_uint = 0xF70E;
pub const NSF12FunctionKey: c_uint = 0xF70F;
pub const NSF13FunctionKey: c_uint = 0xF710;
pub const NSF14FunctionKey: c_uint = 0xF711;
pub const NSF15FunctionKey: c_uint = 0xF712;
pub const NSF16FunctionKey: c_uint = 0xF713;
pub const NSF17FunctionKey: c_uint = 0xF714;
pub const NSF18FunctionKey: c_uint = 0xF715;
pub const NSF19FunctionKey: c_uint = 0xF716;
pub const NSF20FunctionKey: c_uint = 0xF717;
pub const NSF21FunctionKey: c_uint = 0xF718;
pub const NSF22FunctionKey: c_uint = 0xF719;
pub const NSF23FunctionKey: c_uint = 0xF71A;
pub const NSF24FunctionKey: c_uint = 0xF71B;
pub const NSF25FunctionKey: c_uint = 0xF71C;
pub const NSF26FunctionKey: c_uint = 0xF71D;
pub const NSF27FunctionKey: c_uint = 0xF71E;
pub const NSF28FunctionKey: c_uint = 0xF71F;
pub const NSF29FunctionKey: c_uint = 0xF720;
pub const NSF30FunctionKey: c_uint = 0xF721;
pub const NSF31FunctionKey: c_uint = 0xF722;
pub const NSF32FunctionKey: c_uint = 0xF723;
pub const NSF33FunctionKey: c_uint = 0xF724;
pub const NSF34FunctionKey: c_uint = 0xF725;
pub const NSF35FunctionKey: c_uint = 0xF726;
pub const NSInsertFunctionKey: c_uint = 0xF727;
pub const NSDeleteFunctionKey: c_uint = 0xF728;
pub const NSHomeFunctionKey: c_uint = 0xF729;
pub const NSBeginFunctionKey: c_uint = 0xF72A;
pub const NSEndFunctionKey: c_uint = 0xF72B;
pub const NSPageUpFunctionKey: c_uint = 0xF72C;
pub const NSPageDownFunctionKey: c_uint = 0xF72D;
pub const NSPrintScreenFunctionKey: c_uint = 0xF72E;
pub const NSScrollLockFunctionKey: c_uint = 0xF72F;
pub const NSPauseFunctionKey: c_uint = 0xF730;
pub const NSSysReqFunctionKey: c_uint = 0xF731;
pub const NSBreakFunctionKey: c_uint = 0xF732;
pub const NSResetFunctionKey: c_uint = 0xF733;
pub const NSStopFunctionKey: c_uint = 0xF734;
pub const NSMenuFunctionKey: c_uint = 0xF735;
pub const NSUserFunctionKey: c_uint = 0xF736;
pub const NSSystemFunctionKey: c_uint = 0xF737;
pub const NSPrintFunctionKey: c_uint = 0xF738;
pub const NSClearLineFunctionKey: c_uint = 0xF739;
pub const NSClearDisplayFunctionKey: c_uint = 0xF73A;
pub const NSInsertLineFunctionKey: c_uint = 0xF73B;
pub const NSDeleteLineFunctionKey: c_uint = 0xF73C;
pub const NSInsertCharFunctionKey: c_uint = 0xF73D;
pub const NSDeleteCharFunctionKey: c_uint = 0xF73E;
pub const NSPrevFunctionKey: c_uint = 0xF73F;
pub const NSNextFunctionKey: c_uint = 0xF740;
pub const NSSelectFunctionKey: c_uint = 0xF741;
pub const NSExecuteFunctionKey: c_uint = 0xF742;
pub const NSUndoFunctionKey: c_uint = 0xF743;
pub const NSRedoFunctionKey: c_uint = 0xF744;
pub const NSFindFunctionKey: c_uint = 0xF745;
pub const NSHelpFunctionKey: c_uint = 0xF746;
pub const NSModeSwitchFunctionKey: c_uint = 0xF747;
