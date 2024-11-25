//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWConfiguration;
);

unsafe impl NSCoding for CWConfiguration {}

unsafe impl NSCopying for CWConfiguration {}

unsafe impl CopyingHelper for CWConfiguration {
    type Result = Self;
}

unsafe impl NSMutableCopying for CWConfiguration {}

unsafe impl MutableCopyingHelper for CWConfiguration {
    type Result = CWMutableConfiguration;
}

unsafe impl NSObjectProtocol for CWConfiguration {}

unsafe impl NSSecureCoding for CWConfiguration {}

extern_methods!(
    unsafe impl CWConfiguration {
        #[cfg(feature = "CWNetworkProfile")]
        #[method_id(@__retain_semantics Other networkProfiles)]
        pub unsafe fn networkProfiles(&self) -> Retained<NSOrderedSet<CWNetworkProfile>>;

        #[method(requireAdministratorForAssociation)]
        pub unsafe fn requireAdministratorForAssociation(&self) -> bool;

        #[method(requireAdministratorForPower)]
        pub unsafe fn requireAdministratorForPower(&self) -> bool;

        #[method(requireAdministratorForIBSSMode)]
        pub unsafe fn requireAdministratorForIBSSMode(&self) -> bool;

        #[method(rememberJoinedNetworks)]
        pub unsafe fn rememberJoinedNetworks(&self) -> bool;

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CWConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithConfiguration:)]
        pub unsafe fn configurationWithConfiguration(
            configuration: &CWConfiguration,
        ) -> Retained<Self>;

        #[method(isEqualToConfiguration:)]
        pub unsafe fn isEqualToConfiguration(&self, configuration: &CWConfiguration) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwmutableconfiguration?language=objc)
    #[unsafe(super(CWConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWMutableConfiguration;
);

unsafe impl NSCoding for CWMutableConfiguration {}

unsafe impl NSCopying for CWMutableConfiguration {}

unsafe impl CopyingHelper for CWMutableConfiguration {
    type Result = CWConfiguration;
}

unsafe impl NSMutableCopying for CWMutableConfiguration {}

unsafe impl MutableCopyingHelper for CWMutableConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CWMutableConfiguration {}

unsafe impl NSSecureCoding for CWMutableConfiguration {}

extern_methods!(
    unsafe impl CWMutableConfiguration {
        #[cfg(feature = "CWNetworkProfile")]
        #[method_id(@__retain_semantics Other networkProfiles)]
        pub unsafe fn networkProfiles(&self) -> Retained<NSOrderedSet<CWNetworkProfile>>;

        #[cfg(feature = "CWNetworkProfile")]
        #[method(setNetworkProfiles:)]
        pub unsafe fn setNetworkProfiles(&self, network_profiles: &NSOrderedSet<CWNetworkProfile>);

        #[method(requireAdministratorForAssociation)]
        pub unsafe fn requireAdministratorForAssociation(&self) -> bool;

        #[method(setRequireAdministratorForAssociation:)]
        pub unsafe fn setRequireAdministratorForAssociation(
            &self,
            require_administrator_for_association: bool,
        );

        #[method(requireAdministratorForPower)]
        pub unsafe fn requireAdministratorForPower(&self) -> bool;

        #[method(setRequireAdministratorForPower:)]
        pub unsafe fn setRequireAdministratorForPower(&self, require_administrator_for_power: bool);

        #[deprecated]
        #[method(requireAdministratorForIBSSMode)]
        pub unsafe fn requireAdministratorForIBSSMode(&self) -> bool;

        #[deprecated]
        #[method(setRequireAdministratorForIBSSMode:)]
        pub unsafe fn setRequireAdministratorForIBSSMode(
            &self,
            require_administrator_for_ibss_mode: bool,
        );

        #[method(rememberJoinedNetworks)]
        pub unsafe fn rememberJoinedNetworks(&self) -> bool;

        #[method(setRememberJoinedNetworks:)]
        pub unsafe fn setRememberJoinedNetworks(&self, remember_joined_networks: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CWConfiguration`
    unsafe impl CWMutableConfiguration {
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CWConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other configurationWithConfiguration:)]
        pub unsafe fn configurationWithConfiguration(
            configuration: &CWConfiguration,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWMutableConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
