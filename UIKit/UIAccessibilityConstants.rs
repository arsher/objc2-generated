//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraits?language=objc)
// NS_TYPED_ENUM
pub type UIAccessibilityTraits = u64;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitnone?language=objc)
    pub static UIAccessibilityTraitNone: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitbutton?language=objc)
    pub static UIAccessibilityTraitButton: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitlink?language=objc)
    pub static UIAccessibilityTraitLink: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitheader?language=objc)
    pub static UIAccessibilityTraitHeader: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitsearchfield?language=objc)
    pub static UIAccessibilityTraitSearchField: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitimage?language=objc)
    pub static UIAccessibilityTraitImage: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitselected?language=objc)
    pub static UIAccessibilityTraitSelected: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitplayssound?language=objc)
    pub static UIAccessibilityTraitPlaysSound: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitkeyboardkey?language=objc)
    pub static UIAccessibilityTraitKeyboardKey: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitstatictext?language=objc)
    pub static UIAccessibilityTraitStaticText: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitsummaryelement?language=objc)
    pub static UIAccessibilityTraitSummaryElement: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitnotenabled?language=objc)
    pub static UIAccessibilityTraitNotEnabled: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitupdatesfrequently?language=objc)
    pub static UIAccessibilityTraitUpdatesFrequently: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitstartsmediasession?language=objc)
    pub static UIAccessibilityTraitStartsMediaSession: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitadjustable?language=objc)
    pub static UIAccessibilityTraitAdjustable: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitallowsdirectinteraction?language=objc)
    pub static UIAccessibilityTraitAllowsDirectInteraction: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitcausespageturn?language=objc)
    pub static UIAccessibilityTraitCausesPageTurn: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraittabbar?language=objc)
    pub static UIAccessibilityTraitTabBar: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraittogglebutton?language=objc)
    pub static UIAccessibilityTraitToggleButton: UIAccessibilityTraits;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytraitsupportszoom?language=objc)
    pub static UIAccessibilityTraitSupportsZoom: UIAccessibilityTraits;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitynotifications?language=objc)
// NS_TYPED_ENUM
pub type UIAccessibilityNotifications = u32;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityscreenchangednotification?language=objc)
    pub static UIAccessibilityScreenChangedNotification: UIAccessibilityNotifications;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitylayoutchangednotification?language=objc)
    pub static UIAccessibilityLayoutChangedNotification: UIAccessibilityNotifications;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityannouncementnotification?language=objc)
    pub static UIAccessibilityAnnouncementNotification: UIAccessibilityNotifications;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitypagescrollednotification?language=objc)
    pub static UIAccessibilityPageScrolledNotification: UIAccessibilityNotifications;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitypauseassistivetechnologynotification?language=objc)
    pub static UIAccessibilityPauseAssistiveTechnologyNotification: UIAccessibilityNotifications;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityresumeassistivetechnologynotification?language=objc)
    pub static UIAccessibilityResumeAssistiveTechnologyNotification: UIAccessibilityNotifications;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityannouncementdidfinishnotification?language=objc)
    pub static UIAccessibilityAnnouncementDidFinishNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityannouncementkeystringvalue?language=objc)
    pub static UIAccessibilityAnnouncementKeyStringValue: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityannouncementkeywassuccessful?language=objc)
    pub static UIAccessibilityAnnouncementKeyWasSuccessful: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityelementfocusednotification?language=objc)
    pub static UIAccessibilityElementFocusedNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityfocusedelementkey?language=objc)
    pub static UIAccessibilityFocusedElementKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityunfocusedelementkey?language=objc)
    pub static UIAccessibilityUnfocusedElementKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityassistivetechnologykey?language=objc)
    pub static UIAccessibilityAssistiveTechnologyKey: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityassistivetechnologyidentifier?language=objc)
// NS_TYPED_ENUM
pub type UIAccessibilityAssistiveTechnologyIdentifier = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitynotificationswitchcontrolidentifier?language=objc)
    pub static UIAccessibilityNotificationSwitchControlIdentifier:
        &'static UIAccessibilityAssistiveTechnologyIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitynotificationvoiceoveridentifier?language=objc)
    pub static UIAccessibilityNotificationVoiceOverIdentifier:
        &'static UIAccessibilityAssistiveTechnologyIdentifier;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitynavigationstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityNavigationStyle(pub NSInteger);
impl UIAccessibilityNavigationStyle {
    #[doc(alias = "UIAccessibilityNavigationStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIAccessibilityNavigationStyleSeparate")]
    pub const Separate: Self = Self(1);
    #[doc(alias = "UIAccessibilityNavigationStyleCombined")]
    pub const Combined: Self = Self(2);
}

unsafe impl Encode for UIAccessibilityNavigationStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityNavigationStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitycontainertype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityContainerType(pub NSInteger);
impl UIAccessibilityContainerType {
    #[doc(alias = "UIAccessibilityContainerTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIAccessibilityContainerTypeDataTable")]
    pub const DataTable: Self = Self(1);
    #[doc(alias = "UIAccessibilityContainerTypeList")]
    pub const List: Self = Self(2);
    #[doc(alias = "UIAccessibilityContainerTypeLandmark")]
    pub const Landmark: Self = Self(3);
    #[doc(alias = "UIAccessibilityContainerTypeSemanticGroup")]
    pub const SemanticGroup: Self = Self(4);
}

unsafe impl Encode for UIAccessibilityContainerType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityContainerType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitydirecttouchoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityDirectTouchOptions(pub NSUInteger);
bitflags::bitflags! {
    impl UIAccessibilityDirectTouchOptions: NSUInteger {
        const UIAccessibilityDirectTouchOptionNone = 0;
        const UIAccessibilityDirectTouchOptionSilentOnTouch = 1<<0;
        const UIAccessibilityDirectTouchOptionRequiresActivation = 1<<1;
    }
}

unsafe impl Encode for UIAccessibilityDirectTouchOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityDirectTouchOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontext?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type UIAccessibilityTextualContext = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextwordprocessing?language=objc)
    pub static UIAccessibilityTextualContextWordProcessing: &'static UIAccessibilityTextualContext;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextnarrative?language=objc)
    pub static UIAccessibilityTextualContextNarrative: &'static UIAccessibilityTextualContext;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextmessaging?language=objc)
    pub static UIAccessibilityTextualContextMessaging: &'static UIAccessibilityTextualContext;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextspreadsheet?language=objc)
    pub static UIAccessibilityTextualContextSpreadsheet: &'static UIAccessibilityTextualContext;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextfilesystem?language=objc)
    pub static UIAccessibilityTextualContextFileSystem: &'static UIAccessibilityTextualContext;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextsourcecode?language=objc)
    pub static UIAccessibilityTextualContextSourceCode: &'static UIAccessibilityTextualContext;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextualcontextconsole?language=objc)
    pub static UIAccessibilityTextualContextConsole: &'static UIAccessibilityTextualContext;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitypriority?language=objc)
// NS_TYPED_ENUM
pub type UIAccessibilityPriority = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitypriorityhigh?language=objc)
    pub static UIAccessibilityPriorityHigh: &'static UIAccessibilityPriority;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityprioritydefault?language=objc)
    pub static UIAccessibilityPriorityDefault: &'static UIAccessibilityPriority;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityprioritylow?language=objc)
    pub static UIAccessibilityPriorityLow: &'static UIAccessibilityPriority;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityexpandedstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIAccessibilityExpandedStatus(pub NSInteger);
impl UIAccessibilityExpandedStatus {
    #[doc(alias = "UIAccessibilityExpandedStatusUnsupported")]
    pub const Unsupported: Self = Self(0);
    #[doc(alias = "UIAccessibilityExpandedStatusExpanded")]
    pub const Expanded: Self = Self(1);
    #[doc(alias = "UIAccessibilityExpandedStatusCollapsed")]
    pub const Collapsed: Self = Self(2);
}

unsafe impl Encode for UIAccessibilityExpandedStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIAccessibilityExpandedStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributepunctuation?language=objc)
    pub static UIAccessibilitySpeechAttributePunctuation: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributelanguage?language=objc)
    pub static UIAccessibilitySpeechAttributeLanguage: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributepitch?language=objc)
    pub static UIAccessibilitySpeechAttributePitch: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributequeueannouncement?language=objc)
    pub static UIAccessibilitySpeechAttributeQueueAnnouncement: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributeannouncementpriority?language=objc)
    pub static UIAccessibilitySpeechAttributeAnnouncementPriority: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributeipanotation?language=objc)
    pub static UIAccessibilitySpeechAttributeIPANotation: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilityspeechattributespellout?language=objc)
    pub static UIAccessibilitySpeechAttributeSpellOut: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextattributeheadinglevel?language=objc)
    pub static UIAccessibilityTextAttributeHeadingLevel: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextattributecustom?language=objc)
    pub static UIAccessibilityTextAttributeCustom: &'static NSAttributedStringKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitytextattributecontext?language=objc)
    pub static UIAccessibilityTextAttributeContext: &'static NSAttributedStringKey;
}
