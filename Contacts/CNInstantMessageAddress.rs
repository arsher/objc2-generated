//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNInstantMessageAddress")]
    pub struct CNInstantMessageAddress;

    #[cfg(feature = "Contacts_CNInstantMessageAddress")]
    unsafe impl ClassType for CNInstantMessageAddress {
        type Super = NSObject;
    }
);

#[cfg(feature = "Contacts_CNInstantMessageAddress")]
unsafe impl NSCoding for CNInstantMessageAddress {}

#[cfg(feature = "Contacts_CNInstantMessageAddress")]
unsafe impl NSObjectProtocol for CNInstantMessageAddress {}

#[cfg(feature = "Contacts_CNInstantMessageAddress")]
unsafe impl NSSecureCoding for CNInstantMessageAddress {}

extern_methods!(
    #[cfg(feature = "Contacts_CNInstantMessageAddress")]
    unsafe impl CNInstantMessageAddress {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other username)]
        pub unsafe fn username(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other service)]
        pub unsafe fn service(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithUsername:service:)]
        pub unsafe fn initWithUsername_service(
            this: Option<Allocated<Self>>,
            username: &NSString,
            service: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForService:)]
        pub unsafe fn localizedStringForService(service: &NSString) -> Id<NSString, Shared>;
    }
);

extern_static!(CNInstantMessageAddressUsernameKey: &'static NSString);

extern_static!(CNInstantMessageAddressServiceKey: &'static NSString);

extern_static!(CNInstantMessageServiceAIM: &'static NSString);

extern_static!(CNInstantMessageServiceFacebook: &'static NSString);

extern_static!(CNInstantMessageServiceGaduGadu: &'static NSString);

extern_static!(CNInstantMessageServiceGoogleTalk: &'static NSString);

extern_static!(CNInstantMessageServiceICQ: &'static NSString);

extern_static!(CNInstantMessageServiceJabber: &'static NSString);

extern_static!(CNInstantMessageServiceMSN: &'static NSString);

extern_static!(CNInstantMessageServiceQQ: &'static NSString);

extern_static!(CNInstantMessageServiceSkype: &'static NSString);

extern_static!(CNInstantMessageServiceYahoo: &'static NSString);
