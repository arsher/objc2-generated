//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkinactiveschedulingpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKInactiveSchedulingPolicy(pub NSInteger);
impl WKInactiveSchedulingPolicy {
    #[doc(alias = "WKInactiveSchedulingPolicySuspend")]
    pub const Suspend: Self = Self(0);
    #[doc(alias = "WKInactiveSchedulingPolicyThrottle")]
    pub const Throttle: Self = Self(1);
    #[doc(alias = "WKInactiveSchedulingPolicyNone")]
    pub const None: Self = Self(2);
}

unsafe impl Encode for WKInactiveSchedulingPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKInactiveSchedulingPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkpreferences?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKPreferences;
);

unsafe impl NSCoding for WKPreferences {}

unsafe impl NSObjectProtocol for WKPreferences {}

unsafe impl NSSecureCoding for WKPreferences {}

extern_methods!(
    unsafe impl WKPreferences {
        #[method(minimumFontSize)]
        pub unsafe fn minimumFontSize(&self) -> CGFloat;

        #[method(setMinimumFontSize:)]
        pub unsafe fn setMinimumFontSize(&self, minimum_font_size: CGFloat);

        #[method(javaScriptCanOpenWindowsAutomatically)]
        pub unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> bool;

        #[method(setJavaScriptCanOpenWindowsAutomatically:)]
        pub unsafe fn setJavaScriptCanOpenWindowsAutomatically(
            &self,
            java_script_can_open_windows_automatically: bool,
        );

        #[method(isFraudulentWebsiteWarningEnabled)]
        pub unsafe fn isFraudulentWebsiteWarningEnabled(&self) -> bool;

        #[method(setFraudulentWebsiteWarningEnabled:)]
        pub unsafe fn setFraudulentWebsiteWarningEnabled(
            &self,
            fraudulent_website_warning_enabled: bool,
        );

        #[method(shouldPrintBackgrounds)]
        pub unsafe fn shouldPrintBackgrounds(&self) -> bool;

        #[method(setShouldPrintBackgrounds:)]
        pub unsafe fn setShouldPrintBackgrounds(&self, should_print_backgrounds: bool);

        #[method(tabFocusesLinks)]
        pub unsafe fn tabFocusesLinks(&self) -> bool;

        #[method(setTabFocusesLinks:)]
        pub unsafe fn setTabFocusesLinks(&self, tab_focuses_links: bool);

        #[method(isTextInteractionEnabled)]
        pub unsafe fn isTextInteractionEnabled(&self) -> bool;

        #[method(setTextInteractionEnabled:)]
        pub unsafe fn setTextInteractionEnabled(&self, text_interaction_enabled: bool);

        #[method(isSiteSpecificQuirksModeEnabled)]
        pub unsafe fn isSiteSpecificQuirksModeEnabled(&self) -> bool;

        #[method(setSiteSpecificQuirksModeEnabled:)]
        pub unsafe fn setSiteSpecificQuirksModeEnabled(
            &self,
            site_specific_quirks_mode_enabled: bool,
        );

        #[method(isElementFullscreenEnabled)]
        pub unsafe fn isElementFullscreenEnabled(&self) -> bool;

        #[method(setElementFullscreenEnabled:)]
        pub unsafe fn setElementFullscreenEnabled(&self, element_fullscreen_enabled: bool);

        #[method(inactiveSchedulingPolicy)]
        pub unsafe fn inactiveSchedulingPolicy(&self) -> WKInactiveSchedulingPolicy;

        #[method(setInactiveSchedulingPolicy:)]
        pub unsafe fn setInactiveSchedulingPolicy(
            &self,
            inactive_scheduling_policy: WKInactiveSchedulingPolicy,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKPreferences {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// WKDeprecated
    unsafe impl WKPreferences {
        #[deprecated = "Java is no longer supported"]
        #[method(javaEnabled)]
        pub unsafe fn javaEnabled(&self) -> bool;

        #[deprecated = "Java is no longer supported"]
        #[method(setJavaEnabled:)]
        pub unsafe fn setJavaEnabled(&self, java_enabled: bool);

        #[deprecated = "Plug-ins are no longer supported"]
        #[method(plugInsEnabled)]
        pub unsafe fn plugInsEnabled(&self) -> bool;

        #[deprecated = "Plug-ins are no longer supported"]
        #[method(setPlugInsEnabled:)]
        pub unsafe fn setPlugInsEnabled(&self, plug_ins_enabled: bool);

        #[deprecated = "Use WKWebpagePreferences.allowsContentJavaScript to disable content JavaScript on a per-navigation basis"]
        #[method(javaScriptEnabled)]
        pub unsafe fn javaScriptEnabled(&self) -> bool;

        #[deprecated = "Use WKWebpagePreferences.allowsContentJavaScript to disable content JavaScript on a per-navigation basis"]
        #[method(setJavaScriptEnabled:)]
        pub unsafe fn setJavaScriptEnabled(&self, java_script_enabled: bool);
    }
);
