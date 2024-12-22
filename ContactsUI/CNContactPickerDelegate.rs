//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-contacts")]
use objc2_contacts::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/contactsui/cncontactpickerdelegate?language=objc)
    pub unsafe trait CNContactPickerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CNContactPicker", feature = "objc2-contacts"))]
        /// Notifies the delegate when the user selects a contact. Only called when keysToDisplay is empty.
        #[optional]
        #[method(contactPicker:didSelectContact:)]
        unsafe fn contactPicker_didSelectContact(
            &self,
            picker: &CNContactPicker,
            contact: &CNContact,
        );

        #[cfg(all(feature = "CNContactPicker", feature = "objc2-contacts"))]
        /// Notifies the delegate when the user selects a particular value of contact. Only called when keysToDisplay is non-empty.
        #[optional]
        #[method(contactPicker:didSelectContactProperty:)]
        unsafe fn contactPicker_didSelectContactProperty(
            &self,
            picker: &CNContactPicker,
            contact_property: &CNContactProperty,
        );

        #[cfg(feature = "CNContactPicker")]
        /// Notifies the delegate when the contact picker's popover will close.
        #[optional]
        #[method(contactPickerWillClose:)]
        unsafe fn contactPickerWillClose(&self, picker: &CNContactPicker);

        #[cfg(feature = "CNContactPicker")]
        /// Notifies the delegate when the contact picker's popover has closed.
        #[optional]
        #[method(contactPickerDidClose:)]
        unsafe fn contactPickerDidClose(&self, picker: &CNContactPicker);
    }

    unsafe impl ProtocolType for dyn CNContactPickerDelegate {}
);
