//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A mutable value object representing a contact.
    ///
    ///
    /// CNMutableContact is not thread safe. If this is a mutable copy of CNContact then it will throw CNContactPropertyNotFetchedExceptionName when accessing a property that was not fetched for the CNContact.
    ///
    ///
    /// Note: To remove properties when saving a mutable contact, set string properties and array properties to empty values. Set other properties to nil.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cnmutablecontact?language=objc)
    #[unsafe(super(CNContact, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNContact")]
    pub struct CNMutableContact;
);

#[cfg(feature = "CNContact")]
unsafe impl NSCoding for CNMutableContact {}

#[cfg(feature = "CNContact")]
unsafe impl NSCopying for CNMutableContact {}

#[cfg(feature = "CNContact")]
unsafe impl CopyingHelper for CNMutableContact {
    type Result = CNContact;
}

#[cfg(feature = "CNContact")]
unsafe impl NSMutableCopying for CNMutableContact {}

#[cfg(feature = "CNContact")]
unsafe impl MutableCopyingHelper for CNMutableContact {
    type Result = Self;
}

#[cfg(feature = "CNContact")]
unsafe impl NSObjectProtocol for CNMutableContact {}

#[cfg(feature = "CNContact")]
unsafe impl NSSecureCoding for CNMutableContact {}

extern_methods!(
    #[cfg(feature = "CNContact")]
    unsafe impl CNMutableContact {
        #[method(contactType)]
        pub unsafe fn contactType(&self) -> CNContactType;

        /// Setter for [`contactType`][Self::contactType].
        #[method(setContactType:)]
        pub unsafe fn setContactType(&self, contact_type: CNContactType);

        #[method_id(@__retain_semantics Other namePrefix)]
        pub unsafe fn namePrefix(&self) -> Retained<NSString>;

        /// Setter for [`namePrefix`][Self::namePrefix].
        #[method(setNamePrefix:)]
        pub unsafe fn setNamePrefix(&self, name_prefix: &NSString);

        #[method_id(@__retain_semantics Other givenName)]
        pub unsafe fn givenName(&self) -> Retained<NSString>;

        /// Setter for [`givenName`][Self::givenName].
        #[method(setGivenName:)]
        pub unsafe fn setGivenName(&self, given_name: &NSString);

        #[method_id(@__retain_semantics Other middleName)]
        pub unsafe fn middleName(&self) -> Retained<NSString>;

        /// Setter for [`middleName`][Self::middleName].
        #[method(setMiddleName:)]
        pub unsafe fn setMiddleName(&self, middle_name: &NSString);

        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Retained<NSString>;

        /// Setter for [`familyName`][Self::familyName].
        #[method(setFamilyName:)]
        pub unsafe fn setFamilyName(&self, family_name: &NSString);

        #[method_id(@__retain_semantics Other previousFamilyName)]
        pub unsafe fn previousFamilyName(&self) -> Retained<NSString>;

        /// Setter for [`previousFamilyName`][Self::previousFamilyName].
        #[method(setPreviousFamilyName:)]
        pub unsafe fn setPreviousFamilyName(&self, previous_family_name: &NSString);

        #[method_id(@__retain_semantics Other nameSuffix)]
        pub unsafe fn nameSuffix(&self) -> Retained<NSString>;

        /// Setter for [`nameSuffix`][Self::nameSuffix].
        #[method(setNameSuffix:)]
        pub unsafe fn setNameSuffix(&self, name_suffix: &NSString);

        #[method_id(@__retain_semantics Other nickname)]
        pub unsafe fn nickname(&self) -> Retained<NSString>;

        /// Setter for [`nickname`][Self::nickname].
        #[method(setNickname:)]
        pub unsafe fn setNickname(&self, nickname: &NSString);

        #[method_id(@__retain_semantics Other organizationName)]
        pub unsafe fn organizationName(&self) -> Retained<NSString>;

        /// Setter for [`organizationName`][Self::organizationName].
        #[method(setOrganizationName:)]
        pub unsafe fn setOrganizationName(&self, organization_name: &NSString);

        #[method_id(@__retain_semantics Other departmentName)]
        pub unsafe fn departmentName(&self) -> Retained<NSString>;

        /// Setter for [`departmentName`][Self::departmentName].
        #[method(setDepartmentName:)]
        pub unsafe fn setDepartmentName(&self, department_name: &NSString);

        #[method_id(@__retain_semantics Other jobTitle)]
        pub unsafe fn jobTitle(&self) -> Retained<NSString>;

        /// Setter for [`jobTitle`][Self::jobTitle].
        #[method(setJobTitle:)]
        pub unsafe fn setJobTitle(&self, job_title: &NSString);

        #[method_id(@__retain_semantics Other phoneticGivenName)]
        pub unsafe fn phoneticGivenName(&self) -> Retained<NSString>;

        /// Setter for [`phoneticGivenName`][Self::phoneticGivenName].
        #[method(setPhoneticGivenName:)]
        pub unsafe fn setPhoneticGivenName(&self, phonetic_given_name: &NSString);

        #[method_id(@__retain_semantics Other phoneticMiddleName)]
        pub unsafe fn phoneticMiddleName(&self) -> Retained<NSString>;

        /// Setter for [`phoneticMiddleName`][Self::phoneticMiddleName].
        #[method(setPhoneticMiddleName:)]
        pub unsafe fn setPhoneticMiddleName(&self, phonetic_middle_name: &NSString);

        #[method_id(@__retain_semantics Other phoneticFamilyName)]
        pub unsafe fn phoneticFamilyName(&self) -> Retained<NSString>;

        /// Setter for [`phoneticFamilyName`][Self::phoneticFamilyName].
        #[method(setPhoneticFamilyName:)]
        pub unsafe fn setPhoneticFamilyName(&self, phonetic_family_name: &NSString);

        #[method_id(@__retain_semantics Other phoneticOrganizationName)]
        pub unsafe fn phoneticOrganizationName(&self) -> Retained<NSString>;

        /// Setter for [`phoneticOrganizationName`][Self::phoneticOrganizationName].
        #[method(setPhoneticOrganizationName:)]
        pub unsafe fn setPhoneticOrganizationName(&self, phonetic_organization_name: &NSString);

        #[method_id(@__retain_semantics Other note)]
        pub unsafe fn note(&self) -> Retained<NSString>;

        /// Setter for [`note`][Self::note].
        #[method(setNote:)]
        pub unsafe fn setNote(&self, note: &NSString);

        #[method_id(@__retain_semantics Other imageData)]
        pub unsafe fn imageData(&self) -> Option<Retained<NSData>>;

        /// Setter for [`imageData`][Self::imageData].
        #[method(setImageData:)]
        pub unsafe fn setImageData(&self, image_data: Option<&NSData>);

        #[cfg(all(feature = "CNLabeledValue", feature = "CNPhoneNumber"))]
        #[method_id(@__retain_semantics Other phoneNumbers)]
        pub unsafe fn phoneNumbers(&self) -> Retained<NSArray<CNLabeledValue<CNPhoneNumber>>>;

        #[cfg(all(feature = "CNLabeledValue", feature = "CNPhoneNumber"))]
        /// Setter for [`phoneNumbers`][Self::phoneNumbers].
        #[method(setPhoneNumbers:)]
        pub unsafe fn setPhoneNumbers(
            &self,
            phone_numbers: &NSArray<CNLabeledValue<CNPhoneNumber>>,
        );

        #[cfg(feature = "CNLabeledValue")]
        #[method_id(@__retain_semantics Other emailAddresses)]
        pub unsafe fn emailAddresses(&self) -> Retained<NSArray<CNLabeledValue<NSString>>>;

        #[cfg(feature = "CNLabeledValue")]
        /// Setter for [`emailAddresses`][Self::emailAddresses].
        #[method(setEmailAddresses:)]
        pub unsafe fn setEmailAddresses(&self, email_addresses: &NSArray<CNLabeledValue<NSString>>);

        #[cfg(all(feature = "CNLabeledValue", feature = "CNPostalAddress"))]
        #[method_id(@__retain_semantics Other postalAddresses)]
        pub unsafe fn postalAddresses(&self) -> Retained<NSArray<CNLabeledValue<CNPostalAddress>>>;

        #[cfg(all(feature = "CNLabeledValue", feature = "CNPostalAddress"))]
        /// Setter for [`postalAddresses`][Self::postalAddresses].
        #[method(setPostalAddresses:)]
        pub unsafe fn setPostalAddresses(
            &self,
            postal_addresses: &NSArray<CNLabeledValue<CNPostalAddress>>,
        );

        #[cfg(feature = "CNLabeledValue")]
        #[method_id(@__retain_semantics Other urlAddresses)]
        pub unsafe fn urlAddresses(&self) -> Retained<NSArray<CNLabeledValue<NSString>>>;

        #[cfg(feature = "CNLabeledValue")]
        /// Setter for [`urlAddresses`][Self::urlAddresses].
        #[method(setUrlAddresses:)]
        pub unsafe fn setUrlAddresses(&self, url_addresses: &NSArray<CNLabeledValue<NSString>>);

        #[cfg(all(feature = "CNContactRelation", feature = "CNLabeledValue"))]
        #[method_id(@__retain_semantics Other contactRelations)]
        pub unsafe fn contactRelations(
            &self,
        ) -> Retained<NSArray<CNLabeledValue<CNContactRelation>>>;

        #[cfg(all(feature = "CNContactRelation", feature = "CNLabeledValue"))]
        /// Setter for [`contactRelations`][Self::contactRelations].
        #[method(setContactRelations:)]
        pub unsafe fn setContactRelations(
            &self,
            contact_relations: &NSArray<CNLabeledValue<CNContactRelation>>,
        );

        #[cfg(all(feature = "CNLabeledValue", feature = "CNSocialProfile"))]
        #[method_id(@__retain_semantics Other socialProfiles)]
        pub unsafe fn socialProfiles(&self) -> Retained<NSArray<CNLabeledValue<CNSocialProfile>>>;

        #[cfg(all(feature = "CNLabeledValue", feature = "CNSocialProfile"))]
        /// Setter for [`socialProfiles`][Self::socialProfiles].
        #[method(setSocialProfiles:)]
        pub unsafe fn setSocialProfiles(
            &self,
            social_profiles: &NSArray<CNLabeledValue<CNSocialProfile>>,
        );

        #[cfg(all(feature = "CNInstantMessageAddress", feature = "CNLabeledValue"))]
        #[method_id(@__retain_semantics Other instantMessageAddresses)]
        pub unsafe fn instantMessageAddresses(
            &self,
        ) -> Retained<NSArray<CNLabeledValue<CNInstantMessageAddress>>>;

        #[cfg(all(feature = "CNInstantMessageAddress", feature = "CNLabeledValue"))]
        /// Setter for [`instantMessageAddresses`][Self::instantMessageAddresses].
        #[method(setInstantMessageAddresses:)]
        pub unsafe fn setInstantMessageAddresses(
            &self,
            instant_message_addresses: &NSArray<CNLabeledValue<CNInstantMessageAddress>>,
        );

        /// The Gregorian birthday.
        ///
        /// Only uses day, month and year components. Needs to have at least a day and a month.
        #[method_id(@__retain_semantics Other birthday)]
        pub unsafe fn birthday(&self) -> Option<Retained<NSDateComponents>>;

        /// Setter for [`birthday`][Self::birthday].
        #[method(setBirthday:)]
        pub unsafe fn setBirthday(&self, birthday: Option<&NSDateComponents>);

        /// The alternate birthday (Lunisolar).
        ///
        /// Only uses day, month, year and calendar components. Needs to have at least a day and a month. Calendar must be Chinese, Hebrew or Islamic.
        #[method_id(@__retain_semantics Other nonGregorianBirthday)]
        pub unsafe fn nonGregorianBirthday(&self) -> Option<Retained<NSDateComponents>>;

        /// Setter for [`nonGregorianBirthday`][Self::nonGregorianBirthday].
        #[method(setNonGregorianBirthday:)]
        pub unsafe fn setNonGregorianBirthday(
            &self,
            non_gregorian_birthday: Option<&NSDateComponents>,
        );

        #[cfg(feature = "CNLabeledValue")]
        /// Other Gregorian dates (anniversaries, etc).
        ///
        /// Only uses day, month and year components. Needs to have at least a day and a month.
        #[method_id(@__retain_semantics Other dates)]
        pub unsafe fn dates(&self) -> Retained<NSArray<CNLabeledValue<NSDateComponents>>>;

        #[cfg(feature = "CNLabeledValue")]
        /// Setter for [`dates`][Self::dates].
        #[method(setDates:)]
        pub unsafe fn setDates(&self, dates: &NSArray<CNLabeledValue<NSDateComponents>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CNContact")]
    unsafe impl CNMutableContact {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
