//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSEventType {
        NSEventTypeLeftMouseDown = 1,
        NSEventTypeLeftMouseUp = 2,
        NSEventTypeRightMouseDown = 3,
        NSEventTypeRightMouseUp = 4,
        NSEventTypeMouseMoved = 5,
        NSEventTypeLeftMouseDragged = 6,
        NSEventTypeRightMouseDragged = 7,
        NSEventTypeMouseEntered = 8,
        NSEventTypeMouseExited = 9,
        NSEventTypeKeyDown = 10,
        NSEventTypeKeyUp = 11,
        NSEventTypeFlagsChanged = 12,
        NSEventTypeAppKitDefined = 13,
        NSEventTypeSystemDefined = 14,
        NSEventTypeApplicationDefined = 15,
        NSEventTypePeriodic = 16,
        NSEventTypeCursorUpdate = 17,
        NSEventTypeScrollWheel = 22,
        NSEventTypeTabletPoint = 23,
        NSEventTypeTabletProximity = 24,
        NSEventTypeOtherMouseDown = 25,
        NSEventTypeOtherMouseUp = 26,
        NSEventTypeOtherMouseDragged = 27,
        NSEventTypeGesture = 29,
        NSEventTypeMagnify = 30,
        NSEventTypeSwipe = 31,
        NSEventTypeRotate = 18,
        NSEventTypeBeginGesture = 19,
        NSEventTypeEndGesture = 20,
        NSEventTypeSmartMagnify = 32,
        NSEventTypeQuickLook = 33,
        NSEventTypePressure = 34,
        NSEventTypeDirectTouch = 37,
        NSEventTypeChangeMode = 38,
    }
);

extern_static!(NSLeftMouseDown: NSEventType = NSEventTypeLeftMouseDown);

extern_static!(NSLeftMouseUp: NSEventType = NSEventTypeLeftMouseUp);

extern_static!(NSRightMouseDown: NSEventType = NSEventTypeRightMouseDown);

extern_static!(NSRightMouseUp: NSEventType = NSEventTypeRightMouseUp);

extern_static!(NSMouseMoved: NSEventType = NSEventTypeMouseMoved);

extern_static!(NSLeftMouseDragged: NSEventType = NSEventTypeLeftMouseDragged);

extern_static!(NSRightMouseDragged: NSEventType = NSEventTypeRightMouseDragged);

extern_static!(NSMouseEntered: NSEventType = NSEventTypeMouseEntered);

extern_static!(NSMouseExited: NSEventType = NSEventTypeMouseExited);

extern_static!(NSKeyDown: NSEventType = NSEventTypeKeyDown);

extern_static!(NSKeyUp: NSEventType = NSEventTypeKeyUp);

extern_static!(NSFlagsChanged: NSEventType = NSEventTypeFlagsChanged);

extern_static!(NSAppKitDefined: NSEventType = NSEventTypeAppKitDefined);

extern_static!(NSSystemDefined: NSEventType = NSEventTypeSystemDefined);

extern_static!(NSApplicationDefined: NSEventType = NSEventTypeApplicationDefined);

extern_static!(NSPeriodic: NSEventType = NSEventTypePeriodic);

extern_static!(NSCursorUpdate: NSEventType = NSEventTypeCursorUpdate);

extern_static!(NSScrollWheel: NSEventType = NSEventTypeScrollWheel);

extern_static!(NSTabletPoint: NSEventType = NSEventTypeTabletPoint);

extern_static!(NSTabletProximity: NSEventType = NSEventTypeTabletProximity);

extern_static!(NSOtherMouseDown: NSEventType = NSEventTypeOtherMouseDown);

extern_static!(NSOtherMouseUp: NSEventType = NSEventTypeOtherMouseUp);

extern_static!(NSOtherMouseDragged: NSEventType = NSEventTypeOtherMouseDragged);

ns_options!(
    #[underlying(c_ulonglong)]
    pub enum NSEventMask {
        NSEventMaskLeftMouseDown = 1 << NSEventTypeLeftMouseDown,
        NSEventMaskLeftMouseUp = 1 << NSEventTypeLeftMouseUp,
        NSEventMaskRightMouseDown = 1 << NSEventTypeRightMouseDown,
        NSEventMaskRightMouseUp = 1 << NSEventTypeRightMouseUp,
        NSEventMaskMouseMoved = 1 << NSEventTypeMouseMoved,
        NSEventMaskLeftMouseDragged = 1 << NSEventTypeLeftMouseDragged,
        NSEventMaskRightMouseDragged = 1 << NSEventTypeRightMouseDragged,
        NSEventMaskMouseEntered = 1 << NSEventTypeMouseEntered,
        NSEventMaskMouseExited = 1 << NSEventTypeMouseExited,
        NSEventMaskKeyDown = 1 << NSEventTypeKeyDown,
        NSEventMaskKeyUp = 1 << NSEventTypeKeyUp,
        NSEventMaskFlagsChanged = 1 << NSEventTypeFlagsChanged,
        NSEventMaskAppKitDefined = 1 << NSEventTypeAppKitDefined,
        NSEventMaskSystemDefined = 1 << NSEventTypeSystemDefined,
        NSEventMaskApplicationDefined = 1 << NSEventTypeApplicationDefined,
        NSEventMaskPeriodic = 1 << NSEventTypePeriodic,
        NSEventMaskCursorUpdate = 1 << NSEventTypeCursorUpdate,
        NSEventMaskScrollWheel = 1 << NSEventTypeScrollWheel,
        NSEventMaskTabletPoint = 1 << NSEventTypeTabletPoint,
        NSEventMaskTabletProximity = 1 << NSEventTypeTabletProximity,
        NSEventMaskOtherMouseDown = 1 << NSEventTypeOtherMouseDown,
        NSEventMaskOtherMouseUp = 1 << NSEventTypeOtherMouseUp,
        NSEventMaskOtherMouseDragged = 1 << NSEventTypeOtherMouseDragged,
        NSEventMaskGesture = 1 << NSEventTypeGesture,
        NSEventMaskMagnify = 1 << NSEventTypeMagnify,
        NSEventMaskSwipe = 1 << NSEventTypeSwipe,
        NSEventMaskRotate = 1 << NSEventTypeRotate,
        NSEventMaskBeginGesture = 1 << NSEventTypeBeginGesture,
        NSEventMaskEndGesture = 1 << NSEventTypeEndGesture,
        NSEventMaskSmartMagnify = 1 << NSEventTypeSmartMagnify,
        NSEventMaskPressure = 1 << NSEventTypePressure,
        NSEventMaskDirectTouch = 1 << NSEventTypeDirectTouch,
        NSEventMaskChangeMode = 1 << NSEventTypeChangeMode,
        NSEventMaskAny = NSUIntegerMax as _,
    }
);

extern_static!(NSLeftMouseDownMask: NSEventMask = NSEventMaskLeftMouseDown);

extern_static!(NSLeftMouseUpMask: NSEventMask = NSEventMaskLeftMouseUp);

extern_static!(NSRightMouseDownMask: NSEventMask = NSEventMaskRightMouseDown);

extern_static!(NSRightMouseUpMask: NSEventMask = NSEventMaskRightMouseUp);

extern_static!(NSMouseMovedMask: NSEventMask = NSEventMaskMouseMoved);

extern_static!(NSLeftMouseDraggedMask: NSEventMask = NSEventMaskLeftMouseDragged);

extern_static!(NSRightMouseDraggedMask: NSEventMask = NSEventMaskRightMouseDragged);

extern_static!(NSMouseEnteredMask: NSEventMask = NSEventMaskMouseEntered);

extern_static!(NSMouseExitedMask: NSEventMask = NSEventMaskMouseExited);

extern_static!(NSKeyDownMask: NSEventMask = NSEventMaskKeyDown);

extern_static!(NSKeyUpMask: NSEventMask = NSEventMaskKeyUp);

extern_static!(NSFlagsChangedMask: NSEventMask = NSEventMaskFlagsChanged);

extern_static!(NSAppKitDefinedMask: NSEventMask = NSEventMaskAppKitDefined);

extern_static!(NSSystemDefinedMask: NSEventMask = NSEventMaskSystemDefined);

extern_static!(NSApplicationDefinedMask: NSEventMask = NSEventMaskApplicationDefined);

extern_static!(NSPeriodicMask: NSEventMask = NSEventMaskPeriodic);

extern_static!(NSCursorUpdateMask: NSEventMask = NSEventMaskCursorUpdate);

extern_static!(NSScrollWheelMask: NSEventMask = NSEventMaskScrollWheel);

extern_static!(NSTabletPointMask: NSEventMask = NSEventMaskTabletPoint);

extern_static!(NSTabletProximityMask: NSEventMask = NSEventMaskTabletProximity);

extern_static!(NSOtherMouseDownMask: NSEventMask = NSEventMaskOtherMouseDown);

extern_static!(NSOtherMouseUpMask: NSEventMask = NSEventMaskOtherMouseUp);

extern_static!(NSOtherMouseDraggedMask: NSEventMask = NSEventMaskOtherMouseDragged);

inline_fn!(
    pub unsafe fn NSEventMaskFromType(r#type: NSEventType) -> NSEventMask {
        todo!()
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventModifierFlags {
        NSEventModifierFlagCapsLock = 1 << 16,
        NSEventModifierFlagShift = 1 << 17,
        NSEventModifierFlagControl = 1 << 18,
        NSEventModifierFlagOption = 1 << 19,
        NSEventModifierFlagCommand = 1 << 20,
        NSEventModifierFlagNumericPad = 1 << 21,
        NSEventModifierFlagHelp = 1 << 22,
        NSEventModifierFlagFunction = 1 << 23,
        NSEventModifierFlagDeviceIndependentFlagsMask = 0xffff0000,
    }
);

extern_static!(NSAlphaShiftKeyMask: NSEventModifierFlags = NSEventModifierFlagCapsLock);

extern_static!(NSShiftKeyMask: NSEventModifierFlags = NSEventModifierFlagShift);

extern_static!(NSControlKeyMask: NSEventModifierFlags = NSEventModifierFlagControl);

extern_static!(NSAlternateKeyMask: NSEventModifierFlags = NSEventModifierFlagOption);

extern_static!(NSCommandKeyMask: NSEventModifierFlags = NSEventModifierFlagCommand);

extern_static!(NSNumericPadKeyMask: NSEventModifierFlags = NSEventModifierFlagNumericPad);

extern_static!(NSHelpKeyMask: NSEventModifierFlags = NSEventModifierFlagHelp);

extern_static!(NSFunctionKeyMask: NSEventModifierFlags = NSEventModifierFlagFunction);

extern_static!(
    NSDeviceIndependentModifierFlagsMask: NSEventModifierFlags =
        NSEventModifierFlagDeviceIndependentFlagsMask
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPointingDeviceType {
        NSPointingDeviceTypeUnknown = 0,
        NSPointingDeviceTypePen = 1,
        NSPointingDeviceTypeCursor = 2,
        NSPointingDeviceTypeEraser = 3,
    }
);

extern_static!(NSUnknownPointingDevice: NSPointingDeviceType = NSPointingDeviceTypeUnknown);

extern_static!(NSPenPointingDevice: NSPointingDeviceType = NSPointingDeviceTypePen);

extern_static!(NSCursorPointingDevice: NSPointingDeviceType = NSPointingDeviceTypeCursor);

extern_static!(NSEraserPointingDevice: NSPointingDeviceType = NSPointingDeviceTypeEraser);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventButtonMask {
        NSEventButtonMaskPenTip = 1,
        NSEventButtonMaskPenLowerSide = 2,
        NSEventButtonMaskPenUpperSide = 4,
    }
);

extern_static!(NSPenTipMask: NSEventButtonMask = NSEventButtonMaskPenTip);

extern_static!(NSPenLowerSideMask: NSEventButtonMask = NSEventButtonMaskPenLowerSide);

extern_static!(NSPenUpperSideMask: NSEventButtonMask = NSEventButtonMaskPenUpperSide);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventPhase {
        NSEventPhaseNone = 0,
        NSEventPhaseBegan = 0x1 << 0,
        NSEventPhaseStationary = 0x1 << 1,
        NSEventPhaseChanged = 0x1 << 2,
        NSEventPhaseEnded = 0x1 << 3,
        NSEventPhaseCancelled = 0x1 << 4,
        NSEventPhaseMayBegin = 0x1 << 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSEventGestureAxis {
        NSEventGestureAxisNone = 0,
        NSEventGestureAxisHorizontal = 1,
        NSEventGestureAxisVertical = 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSEventSwipeTrackingOptions {
        NSEventSwipeTrackingLockDirection = 0x1 << 0,
        NSEventSwipeTrackingClampGestureAmount = 0x1 << 1,
    }
);

ns_enum!(
    #[underlying(c_short)]
    pub enum NSEventSubtype {
        NSEventSubtypeWindowExposed = 0,
        NSEventSubtypeApplicationActivated = 1,
        NSEventSubtypeApplicationDeactivated = 2,
        NSEventSubtypeWindowMoved = 4,
        NSEventSubtypeScreenChanged = 8,
        NSEventSubtypePowerOff = 1,
        NSEventSubtypeMouseEvent = 0,
        NSEventSubtypeTabletPoint = 1,
        NSEventSubtypeTabletProximity = 2,
        NSEventSubtypeTouch = 3,
    }
);

extern_static!(NSWindowExposedEventType: NSEventSubtype = NSEventSubtypeWindowExposed);

extern_static!(
    NSApplicationActivatedEventType: NSEventSubtype = NSEventSubtypeApplicationActivated
);

extern_static!(
    NSApplicationDeactivatedEventType: NSEventSubtype = NSEventSubtypeApplicationDeactivated
);

extern_static!(NSWindowMovedEventType: NSEventSubtype = NSEventSubtypeWindowMoved);

extern_static!(NSScreenChangedEventType: NSEventSubtype = NSEventSubtypeScreenChanged);

extern_static!(NSAWTEventType: NSEventSubtype = 16);

extern_static!(NSPowerOffEventType: NSEventSubtype = NSEventSubtypePowerOff);

extern_static!(NSMouseEventSubtype: NSEventSubtype = NSEventSubtypeMouseEvent);

extern_static!(NSTabletPointEventSubtype: NSEventSubtype = NSEventSubtypeTabletPoint);

extern_static!(NSTabletProximityEventSubtype: NSEventSubtype = NSEventSubtypeTabletProximity);

extern_static!(NSTouchEventSubtype: NSEventSubtype = NSEventSubtypeTouch);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPressureBehavior {
        NSPressureBehaviorUnknown = -1,
        NSPressureBehaviorPrimaryDefault = 0,
        NSPressureBehaviorPrimaryClick = 1,
        NSPressureBehaviorPrimaryGeneric = 2,
        NSPressureBehaviorPrimaryAccelerator = 3,
        NSPressureBehaviorPrimaryDeepClick = 5,
        NSPressureBehaviorPrimaryDeepDrag = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSEvent")]
    pub struct NSEvent;

    #[cfg(feature = "AppKit_NSEvent")]
    unsafe impl ClassType for NSEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSEvent")]
unsafe impl NSCoding for NSEvent {}

#[cfg(feature = "AppKit_NSEvent")]
unsafe impl NSCopying for NSEvent {}

#[cfg(feature = "AppKit_NSEvent")]
unsafe impl NSObjectProtocol for NSEvent {}

extern_methods!(
    #[cfg(feature = "AppKit_NSEvent")]
    unsafe impl NSEvent {
        #[method(type)]
        pub unsafe fn r#type(&self) -> NSEventType;

        #[method(modifierFlags)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow>>;

        #[method(windowNumber)]
        pub unsafe fn windowNumber(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSGraphicsContext")]
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

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other charactersIgnoringModifiers)]
        pub unsafe fn charactersIgnoringModifiers(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
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

        #[cfg(feature = "AppKit_NSTrackingArea")]
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
        pub unsafe fn vendorDefined(&self) -> Id<Object>;

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

        #[cfg(all(
            feature = "AppKit_NSTouch",
            feature = "AppKit_NSView",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other touchesMatchingPhase:inView:)]
        pub unsafe fn touchesMatchingPhase_inView(
            &self,
            phase: NSTouchPhase,
            view: Option<&NSView>,
        ) -> Id<NSSet<NSTouch>>;

        #[cfg(all(feature = "AppKit_NSTouch", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other allTouches)]
        pub unsafe fn allTouches(&self) -> Id<NSSet<NSTouch>>;

        #[cfg(all(
            feature = "AppKit_NSTouch",
            feature = "AppKit_NSView",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other touchesForView:)]
        pub unsafe fn touchesForView(&self, view: &NSView) -> Id<NSSet<NSTouch>>;

        #[cfg(all(feature = "AppKit_NSTouch", feature = "Foundation_NSArray"))]
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

        #[method(trackSwipeEventWithOptions:dampenAmountThresholdMin:max:usingHandler:)]
        pub unsafe fn trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler(
            &self,
            options: NSEventSwipeTrackingOptions,
            min_dampen_threshold: CGFloat,
            max_dampen_threshold: CGFloat,
            tracking_handler: &Block<(CGFloat, NSEventPhase, Bool, NonNull<Bool>), ()>,
        );

        #[method(startPeriodicEventsAfterDelay:withPeriod:)]
        pub unsafe fn startPeriodicEventsAfterDelay_withPeriod(
            delay: NSTimeInterval,
            period: NSTimeInterval,
        );

        #[method(stopPeriodicEvents)]
        pub unsafe fn stopPeriodicEvents();

        #[cfg(feature = "AppKit_NSGraphicsContext")]
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

        #[cfg(all(feature = "AppKit_NSGraphicsContext", feature = "Foundation_NSString"))]
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

        #[cfg(feature = "AppKit_NSGraphicsContext")]
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

        #[cfg(feature = "AppKit_NSGraphicsContext")]
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

        #[method_id(@__retain_semantics Other addGlobalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addGlobalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: &Block<(NonNull<NSEvent>,), ()>,
        ) -> Option<Id<Object>>;

        #[method_id(@__retain_semantics Other addLocalMonitorForEventsMatchingMask:handler:)]
        pub unsafe fn addLocalMonitorForEventsMatchingMask_handler(
            mask: NSEventMask,
            block: &Block<(NonNull<NSEvent>,), *mut NSEvent>,
        ) -> Option<Id<Object>>;

        #[method(removeMonitor:)]
        pub unsafe fn removeMonitor(event_monitor: &Object);
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSUpArrowFunctionKey = 0xF700,
        NSDownArrowFunctionKey = 0xF701,
        NSLeftArrowFunctionKey = 0xF702,
        NSRightArrowFunctionKey = 0xF703,
        NSF1FunctionKey = 0xF704,
        NSF2FunctionKey = 0xF705,
        NSF3FunctionKey = 0xF706,
        NSF4FunctionKey = 0xF707,
        NSF5FunctionKey = 0xF708,
        NSF6FunctionKey = 0xF709,
        NSF7FunctionKey = 0xF70A,
        NSF8FunctionKey = 0xF70B,
        NSF9FunctionKey = 0xF70C,
        NSF10FunctionKey = 0xF70D,
        NSF11FunctionKey = 0xF70E,
        NSF12FunctionKey = 0xF70F,
        NSF13FunctionKey = 0xF710,
        NSF14FunctionKey = 0xF711,
        NSF15FunctionKey = 0xF712,
        NSF16FunctionKey = 0xF713,
        NSF17FunctionKey = 0xF714,
        NSF18FunctionKey = 0xF715,
        NSF19FunctionKey = 0xF716,
        NSF20FunctionKey = 0xF717,
        NSF21FunctionKey = 0xF718,
        NSF22FunctionKey = 0xF719,
        NSF23FunctionKey = 0xF71A,
        NSF24FunctionKey = 0xF71B,
        NSF25FunctionKey = 0xF71C,
        NSF26FunctionKey = 0xF71D,
        NSF27FunctionKey = 0xF71E,
        NSF28FunctionKey = 0xF71F,
        NSF29FunctionKey = 0xF720,
        NSF30FunctionKey = 0xF721,
        NSF31FunctionKey = 0xF722,
        NSF32FunctionKey = 0xF723,
        NSF33FunctionKey = 0xF724,
        NSF34FunctionKey = 0xF725,
        NSF35FunctionKey = 0xF726,
        NSInsertFunctionKey = 0xF727,
        NSDeleteFunctionKey = 0xF728,
        NSHomeFunctionKey = 0xF729,
        NSBeginFunctionKey = 0xF72A,
        NSEndFunctionKey = 0xF72B,
        NSPageUpFunctionKey = 0xF72C,
        NSPageDownFunctionKey = 0xF72D,
        NSPrintScreenFunctionKey = 0xF72E,
        NSScrollLockFunctionKey = 0xF72F,
        NSPauseFunctionKey = 0xF730,
        NSSysReqFunctionKey = 0xF731,
        NSBreakFunctionKey = 0xF732,
        NSResetFunctionKey = 0xF733,
        NSStopFunctionKey = 0xF734,
        NSMenuFunctionKey = 0xF735,
        NSUserFunctionKey = 0xF736,
        NSSystemFunctionKey = 0xF737,
        NSPrintFunctionKey = 0xF738,
        NSClearLineFunctionKey = 0xF739,
        NSClearDisplayFunctionKey = 0xF73A,
        NSInsertLineFunctionKey = 0xF73B,
        NSDeleteLineFunctionKey = 0xF73C,
        NSInsertCharFunctionKey = 0xF73D,
        NSDeleteCharFunctionKey = 0xF73E,
        NSPrevFunctionKey = 0xF73F,
        NSNextFunctionKey = 0xF740,
        NSSelectFunctionKey = 0xF741,
        NSExecuteFunctionKey = 0xF742,
        NSUndoFunctionKey = 0xF743,
        NSRedoFunctionKey = 0xF744,
        NSFindFunctionKey = 0xF745,
        NSHelpFunctionKey = 0xF746,
        NSModeSwitchFunctionKey = 0xF747,
    }
);
