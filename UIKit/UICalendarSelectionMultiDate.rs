//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarselectionmultidate?language=objc)
    #[unsafe(super(UICalendarSelection, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICalendarSelection")]
    pub struct UICalendarSelectionMultiDate;
);

#[cfg(feature = "UICalendarSelection")]
unsafe impl NSObjectProtocol for UICalendarSelectionMultiDate {}

extern_methods!(
    #[cfg(feature = "UICalendarSelection")]
    unsafe impl UICalendarSelectionMultiDate {
        /// The currently selected dates in the Calendar view.
        #[method_id(@__retain_semantics Other selectedDates)]
        pub unsafe fn selectedDates(&self) -> Retained<NSArray<NSDateComponents>>;

        /// Setter for [`selectedDates`][Self::selectedDates].
        #[method(setSelectedDates:)]
        pub unsafe fn setSelectedDates(&self, selected_dates: &NSArray<NSDateComponents>);

        /// Sets the dates to display in the calendar, with an option to animate the setting.
        #[method(setSelectedDates:animated:)]
        pub unsafe fn setSelectedDates_animated(
            &self,
            selected_dates: &NSArray<NSDateComponents>,
            animated: bool,
        );

        /// The object that acts as the delegate of the calendar view selection
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UICalendarSelectionMultiDateDelegate>>>;

        /// Creates a new multi-date selection with the specified delegate.
        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UICalendarSelectionMultiDateDelegate>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UICalendarSelection`
    #[cfg(feature = "UICalendarSelection")]
    unsafe impl UICalendarSelectionMultiDate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarselectionmultidatedelegate?language=objc)
    pub unsafe trait UICalendarSelectionMultiDateDelegate: NSObjectProtocol {
        #[cfg(feature = "UICalendarSelection")]
        /// Called after the user selects a date in the calendar view.
        ///
        ///
        /// Parameter `selection`: The
        /// `UICalendarSelectionMultiDate`
        /// Parameter `dateComponents`: The date that was selected by the user.
        #[method(multiDateSelection:didSelectDate:)]
        unsafe fn multiDateSelection_didSelectDate(
            &self,
            selection: &UICalendarSelectionMultiDate,
            date_components: &NSDateComponents,
        );

        #[cfg(feature = "UICalendarSelection")]
        /// Called after the user removes selection from one of hte selected dates the calendar view.
        ///
        ///
        /// Parameter `selection`: The
        /// `UICalendarSelectionMultiDate`
        /// Parameter `dateComponents`: The date that was deselected by the user.
        #[method(multiDateSelection:didDeselectDate:)]
        unsafe fn multiDateSelection_didDeselectDate(
            &self,
            selection: &UICalendarSelectionMultiDate,
            date_components: &NSDateComponents,
        );

        #[cfg(feature = "UICalendarSelection")]
        /// Determines if a date is selectable. Dates that are not selectable will be disabled in the calendar view.
        ///
        ///
        /// Parameter `selection`: The
        /// `UICalendarSelectionMultiDate`
        /// Parameter `dateComponents`: The date to be checked by selection.
        ///
        ///
        /// Returns: YES if the date can be selected, NO otherwise.
        #[optional]
        #[method(multiDateSelection:canSelectDate:)]
        unsafe fn multiDateSelection_canSelectDate(
            &self,
            selection: &UICalendarSelectionMultiDate,
            date_components: &NSDateComponents,
        ) -> bool;

        #[cfg(feature = "UICalendarSelection")]
        /// Determines if a date can be deselected.
        ///
        ///
        /// Parameter `selection`: The
        /// `UICalendarSelectionMultiDate`
        /// Parameter `dateComponents`: The date to be checked by selection.
        ///
        ///
        /// Returns: YES if the date can be deselected, NO otherwise.
        #[optional]
        #[method(multiDateSelection:canDeselectDate:)]
        unsafe fn multiDateSelection_canDeselectDate(
            &self,
            selection: &UICalendarSelectionMultiDate,
            date_components: &NSDateComponents,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UICalendarSelectionMultiDateDelegate {}
);
