//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AEAssessmentParticipantConfiguration;

    unsafe impl ClassType for AEAssessmentParticipantConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for AEAssessmentParticipantConfiguration {}

unsafe impl CopyingHelper for AEAssessmentParticipantConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AEAssessmentParticipantConfiguration {}

extern_methods!(
    unsafe impl AEAssessmentParticipantConfiguration {
        #[method(allowsNetworkAccess)]
        pub unsafe fn allowsNetworkAccess(&self) -> bool;

        #[method(setAllowsNetworkAccess:)]
        pub unsafe fn setAllowsNetworkAccess(&self, allows_network_access: bool);

        #[method_id(@__retain_semantics Other configurationInfo)]
        pub unsafe fn configurationInfo(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(setConfigurationInfo:)]
        pub unsafe fn setConfigurationInfo(
            &self,
            configuration_info: &NSDictionary<NSString, AnyObject>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
