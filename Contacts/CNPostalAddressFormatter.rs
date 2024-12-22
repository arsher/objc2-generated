//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// The formatting styles for postal addresses.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cnpostaladdressformatterstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CNPostalAddressFormatterStyle(pub NSInteger);
impl CNPostalAddressFormatterStyle {
    /// Combine the postal address components into a multi-line mailing address.
    #[doc(alias = "CNPostalAddressFormatterStyleMailingAddress")]
    pub const MailingAddress: Self = Self(0);
}

unsafe impl Encode for CNPostalAddressFormatterStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CNPostalAddressFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Formats a postal address.
    ///
    ///
    /// This formatter handles international formatting of a postal address.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cnpostaladdressformatter?language=objc)
    #[unsafe(super(NSFormatter, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNPostalAddressFormatter;
);

unsafe impl NSCoding for CNPostalAddressFormatter {}

unsafe impl NSCopying for CNPostalAddressFormatter {}

unsafe impl CopyingHelper for CNPostalAddressFormatter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNPostalAddressFormatter {}

extern_methods!(
    unsafe impl CNPostalAddressFormatter {
        #[cfg(feature = "CNPostalAddress")]
        /// Formats the postal address.
        ///
        ///
        /// Parameter `postalAddress`: The postal address to be formatted.
        ///
        /// Parameter `style`: The formatting style to be used for the postal address.
        ///
        /// Returns: The formatted postal address.
        #[method_id(@__retain_semantics Other stringFromPostalAddress:style:)]
        pub unsafe fn stringFromPostalAddress_style(
            postal_address: &CNPostalAddress,
            style: CNPostalAddressFormatterStyle,
        ) -> Retained<NSString>;

        #[cfg(feature = "CNPostalAddress")]
        /// Formats the postal address returning an attributed string.
        ///
        ///
        /// This behaves like +stringFromPostalAddress: except it returns an attributed string. Includes attribute keys CNPostalAddressPropertyAttribute and CNPostalAddressLocalizedPropertyNameAttribute.
        ///
        ///
        /// Parameter `postalAddress`: The postal address to be formatted.
        ///
        /// Parameter `style`: The formatting style to be used for the postal address.
        ///
        /// Parameter `attributes`: The default attributes to use. See NSFormatter for details.
        ///
        /// Returns: The formatted postal address as an attributed string.
        #[method_id(@__retain_semantics Other attributedStringFromPostalAddress:style:withDefaultAttributes:)]
        pub unsafe fn attributedStringFromPostalAddress_style_withDefaultAttributes(
            postal_address: &CNPostalAddress,
            style: CNPostalAddressFormatterStyle,
            attributes: &NSDictionary,
        ) -> Retained<NSAttributedString>;

        /// The style for a postal address formatter instance.
        ///
        ///
        /// The default value is CNPostalAddressFormatterStyleMailingAddress.
        #[method(style)]
        pub unsafe fn style(&self) -> CNPostalAddressFormatterStyle;

        /// Setter for [`style`][Self::style].
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: CNPostalAddressFormatterStyle);

        #[cfg(feature = "CNPostalAddress")]
        /// Formats the postal address.
        ///
        ///
        /// Parameter `postalAddress`: The postal address to be formatted.
        ///
        /// Returns: The formatted postal address.
        #[method_id(@__retain_semantics Other stringFromPostalAddress:)]
        pub unsafe fn stringFromPostalAddress(
            &self,
            postal_address: &CNPostalAddress,
        ) -> Retained<NSString>;

        #[cfg(feature = "CNPostalAddress")]
        /// Formats the postal address returning an attributed string.
        ///
        ///
        /// This behaves like +stringFromPostalAddress: except it returns an attributed string. Includes attribute keys CNPostalAddressPropertyAttribute and CNPostalAddressLocalizedPropertyNameAttribute.
        ///
        ///
        /// Parameter `postalAddress`: The postal address to be formatted.
        ///
        /// Parameter `attributes`: The default attributes to use. See NSFormatter for details.
        ///
        /// Returns: The formatted postal address as an attributed string.
        #[method_id(@__retain_semantics Other attributedStringFromPostalAddress:withDefaultAttributes:)]
        pub unsafe fn attributedStringFromPostalAddress_withDefaultAttributes(
            &self,
            postal_address: &CNPostalAddress,
            attributes: &NSDictionary,
        ) -> Retained<NSAttributedString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNPostalAddressFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnpostaladdresspropertyattribute?language=objc)
    pub static CNPostalAddressPropertyAttribute: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/contacts/cnpostaladdresslocalizedpropertynameattribute?language=objc)
    pub static CNPostalAddressLocalizedPropertyNameAttribute: &'static NSString;
}
