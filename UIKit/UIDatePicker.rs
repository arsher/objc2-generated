//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidatepickermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDatePickerMode(pub NSInteger);
impl UIDatePickerMode {
    /// Displays hour, minute, and optionally AM/PM designation depending on the locale setting (e.g. 6 | 53 | PM)
    #[doc(alias = "UIDatePickerModeTime")]
    pub const Time: Self = Self(0);
    /// Displays month, day, and year depending on the locale setting (e.g. November | 15 | 2007)
    #[doc(alias = "UIDatePickerModeDate")]
    pub const Date: Self = Self(1);
    /// Displays date, hour, minute, and optionally AM/PM designation depending on the locale setting (e.g. Wed Nov 15 | 6 | 53 | PM)
    #[doc(alias = "UIDatePickerModeDateAndTime")]
    pub const DateAndTime: Self = Self(2);
    /// Displays hour and minute (e.g. 1 | 53); only supported in `UIDatePickerStyleWheels`
    #[doc(alias = "UIDatePickerModeCountDownTimer")]
    pub const CountDownTimer: Self = Self(3);
    /// Displays year and month depending on the locale setting (e.g. March | 2024); only supported in `UIDatePickerStyleWheels`
    #[doc(alias = "UIDatePickerModeYearAndMonth")]
    pub const YearAndMonth: Self = Self(4);
}

unsafe impl Encode for UIDatePickerMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDatePickerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidatepickerstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDatePickerStyle(pub NSInteger);
impl UIDatePickerStyle {
    /// Automatically pick the best style available for the current platform
    /// &
    /// mode.
    #[doc(alias = "UIDatePickerStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    /// Use the wheels (UIPickerView) style. Editing occurs inline.
    #[doc(alias = "UIDatePickerStyleWheels")]
    pub const Wheels: Self = Self(1);
    /// Use a compact style for the date picker. Editing occurs in an overlay.
    #[doc(alias = "UIDatePickerStyleCompact")]
    pub const Compact: Self = Self(2);
    /// Use a style for the date picker that allows editing in place.
    #[doc(alias = "UIDatePickerStyleInline")]
    pub const Inline: Self = Self(3);
}

unsafe impl Encode for UIDatePickerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDatePickerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidatepicker?language=objc)
    #[unsafe(super(UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UIDatePicker;
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIDatePicker {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UIDatePicker {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIDatePicker {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIDatePicker {
        #[method(datePickerMode)]
        pub unsafe fn datePickerMode(&self) -> UIDatePickerMode;

        /// Setter for [`datePickerMode`][Self::datePickerMode].
        #[method(setDatePickerMode:)]
        pub unsafe fn setDatePickerMode(&self, date_picker_mode: UIDatePickerMode);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Retained<NSLocale>>;

        /// Setter for [`locale`][Self::locale].
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Retained<NSCalendar>;

        /// Setter for [`calendar`][Self::calendar].
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        /// Setter for [`timeZone`][Self::timeZone].
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        /// Setter for [`date`][Self::date].
        #[method(setDate:)]
        pub unsafe fn setDate(&self, date: &NSDate);

        #[method_id(@__retain_semantics Other minimumDate)]
        pub unsafe fn minimumDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`minimumDate`][Self::minimumDate].
        #[method(setMinimumDate:)]
        pub unsafe fn setMinimumDate(&self, minimum_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Other maximumDate)]
        pub unsafe fn maximumDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`maximumDate`][Self::maximumDate].
        #[method(setMaximumDate:)]
        pub unsafe fn setMaximumDate(&self, maximum_date: Option<&NSDate>);

        #[method(countDownDuration)]
        pub unsafe fn countDownDuration(&self) -> NSTimeInterval;

        /// Setter for [`countDownDuration`][Self::countDownDuration].
        #[method(setCountDownDuration:)]
        pub unsafe fn setCountDownDuration(&self, count_down_duration: NSTimeInterval);

        #[method(minuteInterval)]
        pub unsafe fn minuteInterval(&self) -> NSInteger;

        /// Setter for [`minuteInterval`][Self::minuteInterval].
        #[method(setMinuteInterval:)]
        pub unsafe fn setMinuteInterval(&self, minute_interval: NSInteger);

        #[method(setDate:animated:)]
        pub unsafe fn setDate_animated(&self, date: &NSDate, animated: bool);

        /// Request a style for the date picker. If the style changed, then the date picker may need to be resized and will generate a layout pass to display correctly.
        #[method(preferredDatePickerStyle)]
        pub unsafe fn preferredDatePickerStyle(&self) -> UIDatePickerStyle;

        /// Setter for [`preferredDatePickerStyle`][Self::preferredDatePickerStyle].
        #[method(setPreferredDatePickerStyle:)]
        pub unsafe fn setPreferredDatePickerStyle(
            &self,
            preferred_date_picker_style: UIDatePickerStyle,
        );

        /// The style that the date picker is using for its layout. This property always returns a concrete style (never automatic).
        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> UIDatePickerStyle;

        /// When this property is YES,
        /// `date`will always round to the
        /// `minuteInterval,`and
        /// only emit dates that are aligned with the
        /// `minuteInterval.`Otherwise, any changes
        /// to
        /// `date`will ignore the
        /// `minuteInterval`property. Default is
        /// `YES.`
        #[method(roundsToMinuteInterval)]
        pub unsafe fn roundsToMinuteInterval(&self) -> bool;

        /// Setter for [`roundsToMinuteInterval`][Self::roundsToMinuteInterval].
        #[method(setRoundsToMinuteInterval:)]
        pub unsafe fn setRoundsToMinuteInterval(&self, rounds_to_minute_interval: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIDatePicker {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
        /// Initializes the control and adds primaryAction for the UIControlEventPrimaryActionTriggered control event. Subclasses of UIControl may alter or add behaviors around the usage of primaryAction, see subclass documentation of this initializer for additional information.
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIDatePicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
