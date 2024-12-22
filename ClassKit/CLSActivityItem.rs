//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// CLSActivityItem is used to gather information about the activity generated by a user.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/classkit/clsactivityitem?language=objc)
    #[unsafe(super(CLSObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLSObject")]
    pub struct CLSActivityItem;
);

#[cfg(feature = "CLSObject")]
unsafe impl NSCoding for CLSActivityItem {}

#[cfg(feature = "CLSObject")]
unsafe impl NSObjectProtocol for CLSActivityItem {}

#[cfg(feature = "CLSObject")]
unsafe impl NSSecureCoding for CLSActivityItem {}

extern_methods!(
    #[cfg(feature = "CLSObject")]
    unsafe impl CLSActivityItem {
        /// Title of what this ActivityItem represents.
        ///
        /// This will be the title associated with the activity item in the generated progress report.
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        /// An identifier that is unique within its owning activity
        ///
        /// The identifier can be used to look up existing activityItems in a given activity.
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
