//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UICalendarSelection")]
    pub struct UICalendarSelectionSingleDate;

    #[cfg(feature = "UICalendarSelection")]
    unsafe impl ClassType for UICalendarSelectionSingleDate {
        #[inherits(NSObject)]
        type Super = UICalendarSelection;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UICalendarSelection")]
unsafe impl NSObjectProtocol for UICalendarSelectionSingleDate {}

extern_methods!(
    #[cfg(feature = "UICalendarSelection")]
    unsafe impl UICalendarSelectionSingleDate {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UICalendarSelectionSingleDateDelegate>>>;

        #[method_id(@__retain_semantics Other selectedDate)]
        pub unsafe fn selectedDate(&self) -> Option<Id<NSDateComponents>>;

        #[method(setSelectedDate:)]
        pub unsafe fn setSelectedDate(&self, selected_date: Option<&NSDateComponents>);

        #[method(setSelectedDate:animated:)]
        pub unsafe fn setSelectedDate_animated(
            &self,
            selected_date: Option<&NSDateComponents>,
            animated: bool,
        );

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UICalendarSelectionSingleDateDelegate>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UICalendarSelection`
    #[cfg(feature = "UICalendarSelection")]
    unsafe impl UICalendarSelectionSingleDate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UICalendarSelectionSingleDateDelegate: NSObjectProtocol {
        #[cfg(feature = "UICalendarSelection")]
        #[method(dateSelection:didSelectDate:)]
        unsafe fn dateSelection_didSelectDate(
            &self,
            selection: &UICalendarSelectionSingleDate,
            date_components: Option<&NSDateComponents>,
        );

        #[cfg(feature = "UICalendarSelection")]
        #[optional]
        #[method(dateSelection:canSelectDate:)]
        unsafe fn dateSelection_canSelectDate(
            &self,
            selection: &UICalendarSelectionSingleDate,
            date_components: Option<&NSDateComponents>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UICalendarSelectionSingleDateDelegate {}
);
