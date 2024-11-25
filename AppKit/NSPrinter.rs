//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintertablestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrinterTableStatus(pub NSUInteger);
impl NSPrinterTableStatus {
    pub const NSPrinterTableOK: Self = Self(0);
    pub const NSPrinterTableNotFound: Self = Self(1);
    pub const NSPrinterTableError: Self = Self(2);
}

unsafe impl Encode for NSPrinterTableStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPrinterTableStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintertypename?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPrinterTypeName = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprinterpapername?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPrinterPaperName = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprinter?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPrinter;
);

unsafe impl NSCoding for NSPrinter {}

unsafe impl NSCopying for NSPrinter {}

unsafe impl CopyingHelper for NSPrinter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPrinter {}

extern_methods!(
    unsafe impl NSPrinter {
        #[method_id(@__retain_semantics Other printerNames)]
        pub unsafe fn printerNames() -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other printerTypes)]
        pub unsafe fn printerTypes() -> Retained<NSArray<NSPrinterTypeName>>;

        #[method_id(@__retain_semantics Other printerWithName:)]
        pub unsafe fn printerWithName(name: &NSString) -> Option<Retained<NSPrinter>>;

        #[method_id(@__retain_semantics Other printerWithType:)]
        pub unsafe fn printerWithType(r#type: &NSPrinterTypeName) -> Option<Retained<NSPrinter>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<NSPrinterTypeName>;

        #[method(languageLevel)]
        pub unsafe fn languageLevel(&self) -> NSInteger;

        #[method(pageSizeForPaper:)]
        pub unsafe fn pageSizeForPaper(&self, paper_name: &NSPrinterPaperName) -> NSSize;

        #[cfg(feature = "NSGraphics")]
        #[method_id(@__retain_semantics Other deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Retained<NSDictionary<NSDeviceDescriptionKey, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPrinter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPrinter {
        #[deprecated]
        #[method(statusForTable:)]
        pub unsafe fn statusForTable(&self, table_name: &NSString) -> NSPrinterTableStatus;

        #[deprecated]
        #[method(isKey:inTable:)]
        pub unsafe fn isKey_inTable(&self, key: Option<&NSString>, table: &NSString) -> bool;

        #[deprecated]
        #[method(booleanForKey:inTable:)]
        pub unsafe fn booleanForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> bool;

        #[deprecated]
        #[method(floatForKey:inTable:)]
        pub unsafe fn floatForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> c_float;

        #[deprecated]
        #[method(intForKey:inTable:)]
        pub unsafe fn intForKey_inTable(&self, key: Option<&NSString>, table: &NSString) -> c_int;

        #[deprecated]
        #[method(rectForKey:inTable:)]
        pub unsafe fn rectForKey_inTable(&self, key: Option<&NSString>, table: &NSString)
            -> NSRect;

        #[deprecated]
        #[method(sizeForKey:inTable:)]
        pub unsafe fn sizeForKey_inTable(&self, key: Option<&NSString>, table: &NSString)
            -> NSSize;

        #[deprecated]
        #[method_id(@__retain_semantics Other stringForKey:inTable:)]
        pub unsafe fn stringForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other stringListForKey:inTable:)]
        pub unsafe fn stringListForKey_inTable(
            &self,
            key: Option<&NSString>,
            table: &NSString,
        ) -> Option<Retained<NSArray>>;

        #[deprecated]
        #[method(imageRectForPaper:)]
        pub unsafe fn imageRectForPaper(&self, paper_name: Option<&NSString>) -> NSRect;

        #[deprecated]
        #[method(acceptsBinary)]
        pub unsafe fn acceptsBinary(&self) -> bool;

        #[deprecated]
        #[method(isColor)]
        pub unsafe fn isColor(&self) -> bool;

        #[deprecated]
        #[method(isFontAvailable:)]
        pub unsafe fn isFontAvailable(&self, face_name: Option<&NSString>) -> bool;

        #[deprecated]
        #[method(isOutputStackInReverseOrder)]
        pub unsafe fn isOutputStackInReverseOrder(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other printerWithName:domain:includeUnavailable:)]
        pub unsafe fn printerWithName_domain_includeUnavailable(
            name: &NSString,
            domain: Option<&NSString>,
            flag: bool,
        ) -> Option<Retained<NSPrinter>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other host)]
        pub unsafe fn host(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other note)]
        pub unsafe fn note(&self) -> Retained<NSString>;
    }
);
