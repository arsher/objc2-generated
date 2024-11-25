//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwnetworkprofile?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWNetworkProfile;
);

unsafe impl NSCoding for CWNetworkProfile {}

unsafe impl NSCopying for CWNetworkProfile {}

unsafe impl CopyingHelper for CWNetworkProfile {
    type Result = Self;
}

unsafe impl NSMutableCopying for CWNetworkProfile {}

unsafe impl MutableCopyingHelper for CWNetworkProfile {
    type Result = CWMutableNetworkProfile;
}

unsafe impl NSObjectProtocol for CWNetworkProfile {}

unsafe impl NSSecureCoding for CWNetworkProfile {}

extern_methods!(
    unsafe impl CWNetworkProfile {
        #[method_id(@__retain_semantics Other ssid)]
        pub unsafe fn ssid(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(security)]
        pub unsafe fn security(&self) -> CWSecurity;

        #[method_id(@__retain_semantics Other networkProfile)]
        pub unsafe fn networkProfile() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNetworkProfile:)]
        pub unsafe fn initWithNetworkProfile(
            this: Allocated<Self>,
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other networkProfileWithNetworkProfile:)]
        pub unsafe fn networkProfileWithNetworkProfile(
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;

        #[method(isEqualToNetworkProfile:)]
        pub unsafe fn isEqualToNetworkProfile(&self, network_profile: &CWNetworkProfile) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWNetworkProfile {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corewlan/cwmutablenetworkprofile?language=objc)
    #[unsafe(super(CWNetworkProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWMutableNetworkProfile;
);

unsafe impl NSCoding for CWMutableNetworkProfile {}

unsafe impl NSCopying for CWMutableNetworkProfile {}

unsafe impl CopyingHelper for CWMutableNetworkProfile {
    type Result = CWNetworkProfile;
}

unsafe impl NSMutableCopying for CWMutableNetworkProfile {}

unsafe impl MutableCopyingHelper for CWMutableNetworkProfile {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CWMutableNetworkProfile {}

unsafe impl NSSecureCoding for CWMutableNetworkProfile {}

extern_methods!(
    unsafe impl CWMutableNetworkProfile {
        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Retained<NSData>>;

        #[method(setSsidData:)]
        pub unsafe fn setSsidData(&self, ssid_data: Option<&NSData>);

        #[cfg(feature = "CoreWLANTypes")]
        #[method(security)]
        pub unsafe fn security(&self) -> CWSecurity;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(setSecurity:)]
        pub unsafe fn setSecurity(&self, security: CWSecurity);
    }
);

extern_methods!(
    /// Methods declared on superclass `CWNetworkProfile`
    unsafe impl CWMutableNetworkProfile {
        #[method_id(@__retain_semantics Other networkProfile)]
        pub unsafe fn networkProfile() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNetworkProfile:)]
        pub unsafe fn initWithNetworkProfile(
            this: Allocated<Self>,
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other networkProfileWithNetworkProfile:)]
        pub unsafe fn networkProfileWithNetworkProfile(
            network_profile: &CWNetworkProfile,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWMutableNetworkProfile {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
