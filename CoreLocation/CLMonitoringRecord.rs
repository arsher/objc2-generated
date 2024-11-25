//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clmonitoringrecord?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLMonitoringRecord;
);

unsafe impl NSCoding for CLMonitoringRecord {}

unsafe impl NSObjectProtocol for CLMonitoringRecord {}

unsafe impl NSSecureCoding for CLMonitoringRecord {}

extern_methods!(
    unsafe impl CLMonitoringRecord {
        #[cfg(feature = "CLCondition")]
        #[method_id(@__retain_semantics Other condition)]
        pub unsafe fn condition(&self) -> Retained<CLCondition>;

        #[cfg(feature = "CLMonitoringEvent")]
        #[method_id(@__retain_semantics Other lastEvent)]
        pub unsafe fn lastEvent(&self) -> Retained<CLMonitoringEvent>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
