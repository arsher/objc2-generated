//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNPostalAddress")]
    pub struct CNPostalAddress;

    #[cfg(feature = "Contacts_CNPostalAddress")]
    unsafe impl ClassType for CNPostalAddress {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNPostalAddress")]
unsafe impl NSCoding for CNPostalAddress {}

#[cfg(feature = "Contacts_CNPostalAddress")]
unsafe impl NSCopying for CNPostalAddress {}

#[cfg(feature = "Contacts_CNPostalAddress")]
unsafe impl NSMutableCopying for CNPostalAddress {}

#[cfg(feature = "Contacts_CNPostalAddress")]
unsafe impl NSObjectProtocol for CNPostalAddress {}

#[cfg(feature = "Contacts_CNPostalAddress")]
unsafe impl NSSecureCoding for CNPostalAddress {}

extern_methods!(
    #[cfg(feature = "Contacts_CNPostalAddress")]
    unsafe impl CNPostalAddress {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other street)]
        pub unsafe fn street(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subLocality)]
        pub unsafe fn subLocality(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other city)]
        pub unsafe fn city(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subAdministrativeArea)]
        pub unsafe fn subAdministrativeArea(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other postalCode)]
        pub unsafe fn postalCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other country)]
        pub unsafe fn country(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ISOCountryCode)]
        pub unsafe fn ISOCountryCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString>;
    }
);

extern_static!(CNPostalAddressStreetKey: &'static NSString);

extern_static!(CNPostalAddressSubLocalityKey: &'static NSString);

extern_static!(CNPostalAddressCityKey: &'static NSString);

extern_static!(CNPostalAddressSubAdministrativeAreaKey: &'static NSString);

extern_static!(CNPostalAddressStateKey: &'static NSString);

extern_static!(CNPostalAddressPostalCodeKey: &'static NSString);

extern_static!(CNPostalAddressCountryKey: &'static NSString);

extern_static!(CNPostalAddressISOCountryCodeKey: &'static NSString);
