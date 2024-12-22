//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Job types supported by a printer.
///
/// This enumeration provides the abstract job types
/// reported by the UIPrinter supportedJobTypes method.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprinterjobtypes?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPrinterJobTypes(pub NSInteger);
bitflags::bitflags! {
    impl UIPrinterJobTypes: NSInteger {
        const UIPrinterJobTypeUnknown = 0;
        const UIPrinterJobTypeDocument = 1<<0;
        const UIPrinterJobTypeEnvelope = 1<<1;
        const UIPrinterJobTypeLabel = 1<<2;
        const UIPrinterJobTypePhoto = 1<<3;
        const UIPrinterJobTypeReceipt = 1<<4;
        const UIPrinterJobTypeRoll = 1<<5;
        const UIPrinterJobTypeLargeFormat = 1<<6;
        const UIPrinterJobTypePostcard = 1<<7;
    }
}

unsafe impl Encode for UIPrinterJobTypes {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPrinterJobTypes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprinter?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrinter;
);

unsafe impl NSObjectProtocol for UIPrinter {}

extern_methods!(
    unsafe impl UIPrinter {
        /// Create a printer from its URL
        ///
        /// This method creates a new printer object from the printer's URL.
        /// A UIPrinter object is returned even if the printer is not available
        /// on the network.
        #[method_id(@__retain_semantics Other printerWithURL:)]
        pub unsafe fn printerWithURL(url: &NSURL, mtm: MainThreadMarker) -> Retained<UIPrinter>;

        /// Return the URL of the printer.
        ///
        /// This method returns the full URL of the printer which can be
        /// used in future calls to printerWithURL to access the same
        /// printer.
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        /// Return a human-readable printer name.
        ///
        /// This method returns the printer name suitable for displaying in the UI.
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;

        /// Return a human-readable location.
        ///
        /// This method returns the printer's location. This is human-readable text that
        /// usually appears in the UI below the printer's name (such as "Front Office").
        /// Returns nil if the printer doesn't have a location string.
        /// This property's value is undefined until contactPrinter: has been called and
        /// completed successfully.
        #[method_id(@__retain_semantics Other displayLocation)]
        pub unsafe fn displayLocation(&self) -> Option<Retained<NSString>>;

        /// Returns the supported job types of this printer.
        ///
        /// This method returns a mask with all the UIPrinterJobTypes values that
        /// the printer supports.
        /// This property's value is undefined until contactPrinter: has been called and
        /// completed successfully.
        #[method(supportedJobTypes)]
        pub unsafe fn supportedJobTypes(&self) -> UIPrinterJobTypes;

        /// Return make (manufacturer) and model of the printer.
        ///
        /// This method returns the make and model of the printer, which
        /// is usually the manufacturer, model, and model number.
        /// This property's value is undefined until contactPrinter: has been called and
        /// completed successfully.
        #[method_id(@__retain_semantics Other makeAndModel)]
        pub unsafe fn makeAndModel(&self) -> Option<Retained<NSString>>;

        /// Return whether this printer supports color printing.
        ///
        /// This method returns YES if the printer supports full color printing, NO
        /// otherwise.
        /// This property's value is undefined until contactPrinter: has been called and
        /// completed successfully.
        #[method(supportsColor)]
        pub unsafe fn supportsColor(&self) -> bool;

        /// Return whether this printer supports duplex (double-sided) printing.
        ///
        /// This method returns YES if the printer supports duplex (double-sided)
        /// printing, NO otherwise.
        /// This property's value is undefined until contactPrinter: has been called and
        /// completed successfully.
        #[method(supportsDuplex)]
        pub unsafe fn supportsDuplex(&self) -> bool;

        #[cfg(feature = "block2")]
        /// Check if printer is reachable, and update printer information.
        ///
        /// This method checks to see if this printer is available on the network,
        /// and sets the displayName, displayLocation, supportedJobTypes, makeAndModel,
        /// supportsColor, and supportsDuplex for the printer.
        /// The operation can take up to 30 seconds.
        #[method(contactPrinter:)]
        pub unsafe fn contactPrinter(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPrinter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
