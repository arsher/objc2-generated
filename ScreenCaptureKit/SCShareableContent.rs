//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// Defines the type of content being shared
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/scshareablecontentstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SCShareableContentStyle(pub NSInteger);
impl SCShareableContentStyle {
    #[doc(alias = "SCShareableContentStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "SCShareableContentStyleWindow")]
    pub const Window: Self = Self(1);
    #[doc(alias = "SCShareableContentStyleDisplay")]
    pub const Display: Self = Self(2);
    #[doc(alias = "SCShareableContentStyleApplication")]
    pub const Application: Self = Self(3);
}

unsafe impl Encode for SCShareableContentStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SCShareableContentStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/scrunningapplication?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCRunningApplication;
);

unsafe impl NSObjectProtocol for SCRunningApplication {}

extern_methods!(
    unsafe impl SCRunningApplication {
        /// bundleIdentifier the bundleIdentifier for the SCRunningApplication
        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Retained<NSString>;

        /// applicationName the application name for the SCRunningApplication
        #[method_id(@__retain_semantics Other applicationName)]
        pub unsafe fn applicationName(&self) -> Retained<NSString>;

        #[cfg(feature = "libc")]
        /// processID the SCRunningApplication
        #[method(processID)]
        pub unsafe fn processID(&self) -> libc::pid_t;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/scwindow?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCWindow;
);

unsafe impl NSObjectProtocol for SCWindow {}

extern_methods!(
    unsafe impl SCWindow {
        #[cfg(feature = "objc2-core-graphics")]
        /// windowID the CGWindowID for the SCWindow
        #[method(windowID)]
        pub unsafe fn windowID(&self) -> CGWindowID;

        #[cfg(feature = "objc2-core-foundation")]
        /// frame the CGRect for the SCWindow
        #[method(frame)]
        pub unsafe fn frame(&self) -> CGRect;

        /// title the window title for the SCWindow
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// windowLayer the window layer for the SCWindow
        #[method(windowLayer)]
        pub unsafe fn windowLayer(&self) -> NSInteger;

        /// owningApplication is the SCRunningApplication that owns this SCWindow
        #[method_id(@__retain_semantics Other owningApplication)]
        pub unsafe fn owningApplication(&self) -> Option<Retained<SCRunningApplication>>;

        /// onScreen the bool property denoting of the SCWindow is on the screen
        #[method(isOnScreen)]
        pub unsafe fn isOnScreen(&self) -> bool;

        /// active the bool property denoting of the SCWindow is active. with Stage Manager, SCWindow can be offScreen and active
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/scdisplay?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCDisplay;
);

unsafe impl NSObjectProtocol for SCDisplay {}

extern_methods!(
    unsafe impl SCDisplay {
        #[cfg(feature = "objc2-core-graphics")]
        /// displayId the CGDirectDisplayID for the SCDisplay
        #[method(displayID)]
        pub unsafe fn displayID(&self) -> CGDirectDisplayID;

        /// width the width, in points, for the SCDisplay
        #[method(width)]
        pub unsafe fn width(&self) -> NSInteger;

        /// height the height, in points, for the SCDisplay
        #[method(height)]
        pub unsafe fn height(&self) -> NSInteger;

        #[cfg(feature = "objc2-core-foundation")]
        /// frame the CGRect frame for the SCDisplay
        #[method(frame)]
        pub unsafe fn frame(&self) -> CGRect;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// SCShareableContentInfo
    ///
    /// SCShareableContentInformation is an object that has information about the content of the stream
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/scshareablecontentinfo?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCShareableContentInfo;
);

unsafe impl NSObjectProtocol for SCShareableContentInfo {}

extern_methods!(
    unsafe impl SCShareableContentInfo {
        /// style of stream
        #[method(style)]
        pub unsafe fn style(&self) -> SCShareableContentStyle;

        /// Pixel to points scaling factor
        #[method(pointPixelScale)]
        pub unsafe fn pointPixelScale(&self) -> c_float;

        #[cfg(feature = "objc2-core-foundation")]
        /// Size and location of content in points
        #[method(contentRect)]
        pub unsafe fn contentRect(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SCShareableContentInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/screencapturekit/scshareablecontent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCShareableContent;
);

unsafe impl NSObjectProtocol for SCShareableContent {}

extern_methods!(
    unsafe impl SCShareableContent {
        #[cfg(feature = "block2")]
        /// getShareableContentWithCompletionHandler:completionHandler
        ///
        /// Parameter `completionHandler`: the call back that will hand you back a SCShareableContent object
        ///
        /// this method will create a SCShareableContent object that is called on the supplied queue. The SCShareableContent will contain the windows, displays and applications that are available to capture
        #[method(getShareableContentWithCompletionHandler:)]
        pub unsafe fn getShareableContentWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(*mut SCShareableContent, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// getCurrentProcessShareableContentWithCompletionHandler:completionHandler
        ///
        /// Parameter `completionHandler`: the call back that will hand you back a SCShareableContent object
        ///
        /// this method will create a SCShareableContent object that is called on the supplied queue. The SCShareableContent will contain redacted information about windows, displays and applications that are available to capture by current process without user consent via TCC
        #[method(getCurrentProcessShareableContentWithCompletionHandler:)]
        pub unsafe fn getCurrentProcessShareableContentWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(*mut SCShareableContent, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// getShareableContentExcludingDesktopWindows:onScreenWindowsOnly:completionHandler
        ///
        /// Parameter `excludeDesktopWindows`: a BOOL indicating if we should exclude desktop windows
        ///
        /// Parameter `onScreenWindowsOnly`: filter only windows that are on screen
        ///
        /// Parameter `completionHandler`: the call back that will hand you back a SCShareableContent object
        ///
        /// this method will create a SCShareableContent object that is called on the supplied queue. The SCShareableContent will contain the windows, displays and applications that are available to capture
        #[method(getShareableContentExcludingDesktopWindows:onScreenWindowsOnly:completionHandler:)]
        pub unsafe fn getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(
            exclude_desktop_windows: bool,
            on_screen_windows_only: bool,
            completion_handler: &block2::Block<dyn Fn(*mut SCShareableContent, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// getShareableContentExcludingDesktopWindows:onScreenWindowsOnlyBelowWindow:completionHandler
        ///
        /// Parameter `excludeDesktopWindows`: a BOOL indicating if we should exclude desktop windows
        ///
        /// Parameter `window`: filter only windows below this SCWindow
        ///
        /// Parameter `completionHandler`: the call back that will hand you back a SCShareableContent object
        ///
        /// this method will create a SCShareableContent object that is called on the supplied queue. The SCShareableContent will contain the windows, displays and applications that are available to capture
        #[method(getShareableContentExcludingDesktopWindows:onScreenWindowsOnlyBelowWindow:completionHandler:)]
        pub unsafe fn getShareableContentExcludingDesktopWindows_onScreenWindowsOnlyBelowWindow_completionHandler(
            exclude_desktop_windows: bool,
            window: &SCWindow,
            completion_handler: &block2::Block<dyn Fn(*mut SCShareableContent, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// getShareableContentExcludingDesktopWindows:onScreenWindowsOnlyAboveWindow:completionHandler
        ///
        /// Parameter `excludeDesktopWindows`: a BOOL indicating if we should exclude desktop windows
        ///
        /// Parameter `window`: filter only windows above this SCWindow
        ///
        /// Parameter `completionHandler`: the call back that will hand you back a SCShareableContent object
        ///
        /// this method will create a SCShareableContent object that is called on the supplied queue. The SCShareableContent will contain the windows, displays and applications that are available to capture
        #[method(getShareableContentExcludingDesktopWindows:onScreenWindowsOnlyAboveWindow:completionHandler:)]
        pub unsafe fn getShareableContentExcludingDesktopWindows_onScreenWindowsOnlyAboveWindow_completionHandler(
            exclude_desktop_windows: bool,
            window: &SCWindow,
            completion_handler: &block2::Block<dyn Fn(*mut SCShareableContent, *mut NSError)>,
        );

        #[cfg(feature = "SCStream")]
        /// infoForFilter:
        ///
        /// Parameter `filter`: content filter to translate to content details
        ///
        /// this method will create a SCShareableContentInformation object given a filter
        #[method_id(@__retain_semantics Other infoForFilter:)]
        pub unsafe fn infoForFilter(filter: &SCContentFilter) -> Retained<SCShareableContentInfo>;

        /// windows SCShareableContent property that contains all the sharable SCWindows
        #[method_id(@__retain_semantics Other windows)]
        pub unsafe fn windows(&self) -> Retained<NSArray<SCWindow>>;

        /// displays SCShareableContent property that contains all the sharable SCDisplays
        #[method_id(@__retain_semantics Other displays)]
        pub unsafe fn displays(&self) -> Retained<NSArray<SCDisplay>>;

        /// applications SCShareableContent property that contains all the sharable SCRunningApplications
        #[method_id(@__retain_semantics Other applications)]
        pub unsafe fn applications(&self) -> Retained<NSArray<SCRunningApplication>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
