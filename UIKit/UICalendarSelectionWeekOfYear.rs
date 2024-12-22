//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarselectionweekofyear?language=objc)
    #[unsafe(super(UICalendarSelection, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICalendarSelection")]
    pub struct UICalendarSelectionWeekOfYear;
);

#[cfg(feature = "UICalendarSelection")]
unsafe impl NSObjectProtocol for UICalendarSelectionWeekOfYear {}

extern_methods!(
    #[cfg(feature = "UICalendarSelection")]
    unsafe impl UICalendarSelectionWeekOfYear {
        /// The currently selected week of year in the Calendar view. The components must include `[.yearForWeekOfYear, .weekOfYear]`.
        #[method_id(@__retain_semantics Other selectedWeekOfYear)]
        pub unsafe fn selectedWeekOfYear(&self) -> Option<Retained<NSDateComponents>>;

        /// Setter for [`selectedWeekOfYear`][Self::selectedWeekOfYear].
        #[method(setSelectedWeekOfYear:)]
        pub unsafe fn setSelectedWeekOfYear(
            &self,
            selected_week_of_year: Option<&NSDateComponents>,
        );

        /// Sets the dates to display in the calendar, with an option to animate the setting.
        #[method(setSelectedWeekOfYear:animated:)]
        pub unsafe fn setSelectedWeekOfYear_animated(
            &self,
            selected_week_of_year: Option<&NSDateComponents>,
            animated: bool,
        );

        /// The object that acts as the delegate of the calendar view selection
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UICalendarSelectionWeekOfYearDelegate>>>;

        /// Creates a new multi-date selection with the specified delegate.
        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UICalendarSelectionWeekOfYearDelegate>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UICalendarSelection`
    #[cfg(feature = "UICalendarSelection")]
    unsafe impl UICalendarSelectionWeekOfYear {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicalendarselectionweekofyeardelegate?language=objc)
    pub unsafe trait UICalendarSelectionWeekOfYearDelegate: NSObjectProtocol {
        #[cfg(feature = "UICalendarSelection")]
        /// Called after the user selects a week of year in the calendar view.
        ///
        ///
        /// Parameter `selection`: The
        /// `UICalendarSelectionWeekOfYear`
        /// Parameter `dateComponents`: The date that was selected by the user.
        #[method(weekOfYearSelection:didSelectWeekOfYear:)]
        unsafe fn weekOfYearSelection_didSelectWeekOfYear(
            &self,
            selection: &UICalendarSelectionWeekOfYear,
            week_of_year_components: Option<&NSDateComponents>,
        );

        #[cfg(feature = "UICalendarSelection")]
        /// Determines if a week of year is selectable. Dates that are not selectable will be disabled in the calendar view.
        ///
        ///
        /// Parameter `selection`: The
        /// `UICalendarSelectionWeekOfYear`
        /// Parameter `dateComponents`: The date to be checked by selection.
        ///
        ///
        /// Returns: YES if the date can be selected, NO otherwise.
        #[optional]
        #[method(weekOfYearSelection:canSelectWeekOfYear:)]
        unsafe fn weekOfYearSelection_canSelectWeekOfYear(
            &self,
            selection: &UICalendarSelectionWeekOfYear,
            week_of_year_components: Option<&NSDateComponents>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UICalendarSelectionWeekOfYearDelegate {}
);
