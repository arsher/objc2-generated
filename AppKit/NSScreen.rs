//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(target_vendor = "apple")]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScreen;

    unsafe impl ClassType for NSScreen {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSScreen {}

extern_methods!(
    unsafe impl NSScreen {
        #[method_id(@__retain_semantics Other screens)]
        pub fn screens(mtm: MainThreadMarker) -> Retained<NSArray<NSScreen>>;

        #[method_id(@__retain_semantics Other mainScreen)]
        pub fn mainScreen(mtm: MainThreadMarker) -> Option<Retained<NSScreen>>;

        #[method_id(@__retain_semantics Other deepestScreen)]
        pub unsafe fn deepestScreen(mtm: MainThreadMarker) -> Option<Retained<NSScreen>>;

        #[method(screensHaveSeparateSpaces)]
        pub unsafe fn screensHaveSeparateSpaces(mtm: MainThreadMarker) -> bool;

        #[cfg(feature = "NSGraphics")]
        #[method(depth)]
        pub unsafe fn depth(&self) -> NSWindowDepth;

        #[method(frame)]
        pub fn frame(&self) -> NSRect;

        #[method(visibleFrame)]
        pub fn visibleFrame(&self) -> NSRect;

        #[cfg(feature = "NSGraphics")]
        #[method_id(@__retain_semantics Other deviceDescription)]
        pub fn deviceDescription(
            &self,
        ) -> Retained<NSDictionary<NSDeviceDescriptionKey, AnyObject>>;

        #[cfg(feature = "NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Option<Retained<NSColorSpace>>;

        #[cfg(feature = "NSGraphics")]
        #[method(supportedWindowDepths)]
        pub unsafe fn supportedWindowDepths(&self) -> NonNull<NSWindowDepth>;

        #[cfg(feature = "NSGraphics")]
        #[method(canRepresentDisplayGamut:)]
        pub unsafe fn canRepresentDisplayGamut(&self, display_gamut: NSDisplayGamut) -> bool;

        #[method(convertRectToBacking:)]
        pub unsafe fn convertRectToBacking(&self, rect: NSRect) -> NSRect;

        #[method(convertRectFromBacking:)]
        pub unsafe fn convertRectFromBacking(&self, rect: NSRect) -> NSRect;

        #[method(backingAlignedRect:options:)]
        pub unsafe fn backingAlignedRect_options(
            &self,
            rect: NSRect,
            options: NSAlignmentOptions,
        ) -> NSRect;

        #[method(backingScaleFactor)]
        pub fn backingScaleFactor(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Retained<NSString>;

        #[method(safeAreaInsets)]
        pub unsafe fn safeAreaInsets(&self) -> NSEdgeInsets;

        #[method(auxiliaryTopLeftArea)]
        pub unsafe fn auxiliaryTopLeftArea(&self) -> NSRect;

        #[method(auxiliaryTopRightArea)]
        pub unsafe fn auxiliaryTopRightArea(&self) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScreen {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static NSScreenColorSpaceDidChangeNotification: &'static NSNotificationName;
}

extern_methods!(
    unsafe impl NSScreen {
        #[method(maximumExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;

        #[method(maximumPotentialExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumPotentialExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;

        #[method(maximumReferenceExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumReferenceExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;
    }
);

extern_methods!(
    unsafe impl NSScreen {
        #[method(maximumFramesPerSecond)]
        pub unsafe fn maximumFramesPerSecond(&self) -> NSInteger;

        #[method(minimumRefreshInterval)]
        pub unsafe fn minimumRefreshInterval(&self) -> NSTimeInterval;

        #[method(maximumRefreshInterval)]
        pub unsafe fn maximumRefreshInterval(&self) -> NSTimeInterval;

        #[method(displayUpdateGranularity)]
        pub unsafe fn displayUpdateGranularity(&self) -> NSTimeInterval;

        #[method(lastDisplayUpdateTimestamp)]
        pub unsafe fn lastDisplayUpdateTimestamp(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// NSDisplayLink
    unsafe impl NSScreen {
        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Other displayLinkWithTarget:selector:)]
        pub unsafe fn displayLinkWithTarget_selector(
            &self,
            target: &AnyObject,
            selector: Sel,
        ) -> Retained<CADisplayLink>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSScreen {
        #[deprecated = "Use -convertRectToBacking: or -backingScaleFactor instead"]
        #[method(userSpaceScaleFactor)]
        pub unsafe fn userSpaceScaleFactor(&self) -> CGFloat;
    }
);
