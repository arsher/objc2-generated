//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreendidconnectnotification?language=objc)
    pub static UIScreenDidConnectNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreendiddisconnectnotification?language=objc)
    pub static UIScreenDidDisconnectNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreenmodedidchangenotification?language=objc)
    pub static UIScreenModeDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreenbrightnessdidchangenotification?language=objc)
    pub static UIScreenBrightnessDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreencaptureddidchangenotification?language=objc)
    pub static UIScreenCapturedDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreenreferencedisplaymodestatusdidchangenotification?language=objc)
    pub static UIScreenReferenceDisplayModeStatusDidChangeNotification: &'static NSNotificationName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreenoverscancompensation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIScreenOverscanCompensation(pub NSInteger);
impl UIScreenOverscanCompensation {
    #[doc(alias = "UIScreenOverscanCompensationScale")]
    pub const Scale: Self = Self(0);
    #[doc(alias = "UIScreenOverscanCompensationInsetBounds")]
    pub const InsetBounds: Self = Self(1);
    #[doc(alias = "UIScreenOverscanCompensationNone")]
    pub const None: Self = Self(2);
    #[deprecated]
    #[doc(alias = "UIScreenOverscanCompensationInsetApplicationFrame")]
    pub const InsetApplicationFrame: Self = Self(2);
}

unsafe impl Encode for UIScreenOverscanCompensation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIScreenOverscanCompensation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreenreferencedisplaymodestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIScreenReferenceDisplayModeStatus(pub NSInteger);
impl UIScreenReferenceDisplayModeStatus {
    #[doc(alias = "UIScreenReferenceDisplayModeStatusNotSupported")]
    pub const NotSupported: Self = Self(0);
    #[doc(alias = "UIScreenReferenceDisplayModeStatusNotEnabled")]
    pub const NotEnabled: Self = Self(1);
    #[doc(alias = "UIScreenReferenceDisplayModeStatusLimited")]
    pub const Limited: Self = Self(2);
    #[doc(alias = "UIScreenReferenceDisplayModeStatusEnabled")]
    pub const Enabled: Self = Self(3);
}

unsafe impl Encode for UIScreenReferenceDisplayModeStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIScreenReferenceDisplayModeStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreen?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIScreen;
);

unsafe impl NSObjectProtocol for UIScreen {}

#[cfg(feature = "UITraitCollection")]
unsafe impl UITraitEnvironment for UIScreen {}

extern_methods!(
    unsafe impl UIScreen {
        #[deprecated = "Use UIApplication.shared.openSessions to find open sessions with scenes from other screens"]
        #[method_id(@__retain_semantics Other screens)]
        pub fn screens(mtm: MainThreadMarker) -> Retained<NSArray<UIScreen>>;

        #[deprecated = "Use a UIScreen instance found through context instead: i.e, view.window.windowScene.screen"]
        #[method_id(@__retain_semantics Other mainScreen)]
        pub fn mainScreen(mtm: MainThreadMarker) -> Retained<UIScreen>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scale)]
        pub fn scale(&self) -> CGFloat;

        #[cfg(feature = "UIScreenMode")]
        #[method_id(@__retain_semantics Other availableModes)]
        pub fn availableModes(&self) -> Retained<NSArray<UIScreenMode>>;

        #[cfg(feature = "UIScreenMode")]
        #[method_id(@__retain_semantics Other preferredMode)]
        pub fn preferredMode(&self) -> Option<Retained<UIScreenMode>>;

        #[cfg(feature = "UIScreenMode")]
        #[method_id(@__retain_semantics Other currentMode)]
        pub fn currentMode(&self) -> Option<Retained<UIScreenMode>>;

        #[cfg(feature = "UIScreenMode")]
        #[method(setCurrentMode:)]
        pub fn setCurrentMode(&self, current_mode: Option<&UIScreenMode>);

        #[method(overscanCompensation)]
        pub fn overscanCompensation(&self) -> UIScreenOverscanCompensation;

        #[method(setOverscanCompensation:)]
        pub fn setOverscanCompensation(&self, overscan_compensation: UIScreenOverscanCompensation);

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(overscanCompensationInsets)]
        pub unsafe fn overscanCompensationInsets(&self) -> UIEdgeInsets;

        #[method_id(@__retain_semantics Other mirroredScreen)]
        pub fn mirroredScreen(&self) -> Option<Retained<UIScreen>>;

        #[deprecated = "Use the sceneCaptureState in UITraitCollection instead."]
        #[method(isCaptured)]
        pub unsafe fn isCaptured(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(brightness)]
        pub unsafe fn brightness(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setBrightness:)]
        pub unsafe fn setBrightness(&self, brightness: CGFloat);

        #[method(wantsSoftwareDimming)]
        pub unsafe fn wantsSoftwareDimming(&self) -> bool;

        #[method(setWantsSoftwareDimming:)]
        pub unsafe fn setWantsSoftwareDimming(&self, wants_software_dimming: bool);

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other coordinateSpace)]
        pub fn coordinateSpace(&self) -> Retained<ProtocolObject<dyn UICoordinateSpace>>;

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other fixedCoordinateSpace)]
        pub unsafe fn fixedCoordinateSpace(
            &self,
        ) -> Retained<ProtocolObject<dyn UICoordinateSpace>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(nativeBounds)]
        pub fn nativeBounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(nativeScale)]
        pub fn nativeScale(&self) -> CGFloat;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other displayLinkWithTarget:selector:)]
        pub unsafe fn displayLinkWithTarget_selector(
            &self,
            target: &AnyObject,
            sel: Sel,
        ) -> Option<Retained<CADisplayLink>>;

        #[method(maximumFramesPerSecond)]
        pub fn maximumFramesPerSecond(&self) -> NSInteger;

        #[method(referenceDisplayModeStatus)]
        pub unsafe fn referenceDisplayModeStatus(&self) -> UIScreenReferenceDisplayModeStatus;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(currentEDRHeadroom)]
        pub unsafe fn currentEDRHeadroom(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(potentialEDRHeadroom)]
        pub unsafe fn potentialEDRHeadroom(&self) -> CGFloat;

        #[cfg(feature = "UIFocus")]
        #[deprecated = "Use -[UIWindowScene focusSystem].focusedItem instead"]
        #[method_id(@__retain_semantics Other focusedItem)]
        pub unsafe fn focusedItem(&self) -> Option<Retained<ProtocolObject<dyn UIFocusItem>>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[deprecated = "Use -[UIWindowScene focusSystem].focusedItem instead"]
        #[method_id(@__retain_semantics Other focusedView)]
        pub unsafe fn focusedView(&self) -> Option<Retained<UIView>>;

        #[deprecated = "Use -[UIWindowScene focusSystem] != nil instead"]
        #[method(supportsFocus)]
        pub unsafe fn supportsFocus(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(applicationFrame)]
        pub unsafe fn applicationFrame(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIScreen {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// UISnapshotting
    unsafe impl UIScreen {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other snapshotViewAfterScreenUpdates:)]
        pub unsafe fn snapshotViewAfterScreenUpdates(
            &self,
            after_updates: bool,
        ) -> Retained<UIView>;
    }
);
