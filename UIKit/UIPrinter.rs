//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPrinterJobTypes(pub NSInteger);
impl UIPrinterJobTypes {
    pub const UIPrinterJobTypeUnknown: Self = Self(0);
    pub const UIPrinterJobTypeDocument: Self = Self(1 << 0);
    pub const UIPrinterJobTypeEnvelope: Self = Self(1 << 1);
    pub const UIPrinterJobTypeLabel: Self = Self(1 << 2);
    pub const UIPrinterJobTypePhoto: Self = Self(1 << 3);
    pub const UIPrinterJobTypeReceipt: Self = Self(1 << 4);
    pub const UIPrinterJobTypeRoll: Self = Self(1 << 5);
    pub const UIPrinterJobTypeLargeFormat: Self = Self(1 << 6);
    pub const UIPrinterJobTypePostcard: Self = Self(1 << 7);
}

unsafe impl Encode for UIPrinterJobTypes {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPrinterJobTypes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrinter;

    unsafe impl ClassType for UIPrinter {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIPrinter {}

extern_methods!(
    unsafe impl UIPrinter {
        #[method_id(@__retain_semantics Other printerWithURL:)]
        pub unsafe fn printerWithURL(url: &NSURL, mtm: MainThreadMarker) -> Id<UIPrinter>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL>;

        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other displayLocation)]
        pub unsafe fn displayLocation(&self) -> Option<Id<NSString>>;

        #[method(supportedJobTypes)]
        pub unsafe fn supportedJobTypes(&self) -> UIPrinterJobTypes;

        #[method_id(@__retain_semantics Other makeAndModel)]
        pub unsafe fn makeAndModel(&self) -> Option<Id<NSString>>;

        #[method(supportsColor)]
        pub unsafe fn supportsColor(&self) -> bool;

        #[method(supportsDuplex)]
        pub unsafe fn supportsDuplex(&self) -> bool;

        #[cfg(feature = "block2")]
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
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
