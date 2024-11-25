//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtype?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPasteboardType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypestring?language=objc)
    pub static NSPasteboardTypeString: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypepdf?language=objc)
    pub static NSPasteboardTypePDF: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypetiff?language=objc)
    pub static NSPasteboardTypeTIFF: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypepng?language=objc)
    pub static NSPasteboardTypePNG: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypertf?language=objc)
    pub static NSPasteboardTypeRTF: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypertfd?language=objc)
    pub static NSPasteboardTypeRTFD: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypehtml?language=objc)
    pub static NSPasteboardTypeHTML: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypetabulartext?language=objc)
    pub static NSPasteboardTypeTabularText: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypefont?language=objc)
    pub static NSPasteboardTypeFont: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtyperuler?language=objc)
    pub static NSPasteboardTypeRuler: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypecolor?language=objc)
    pub static NSPasteboardTypeColor: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypesound?language=objc)
    pub static NSPasteboardTypeSound: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypemultipletextselection?language=objc)
    pub static NSPasteboardTypeMultipleTextSelection: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypetextfinderoptions?language=objc)
    pub static NSPasteboardTypeTextFinderOptions: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypeurl?language=objc)
    pub static NSPasteboardTypeURL: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypefileurl?language=objc)
    pub static NSPasteboardTypeFileURL: &'static NSPasteboardType;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardname?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPasteboardName = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardnamegeneral?language=objc)
    pub static NSPasteboardNameGeneral: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardnamefont?language=objc)
    pub static NSPasteboardNameFont: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardnameruler?language=objc)
    pub static NSPasteboardNameRuler: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardnamefind?language=objc)
    pub static NSPasteboardNameFind: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardnamedrag?language=objc)
    pub static NSPasteboardNameDrag: &'static NSPasteboardName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardcontentsoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPasteboardContentsOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPasteboardContentsOptions: NSUInteger {
        const NSPasteboardContentsCurrentHostOnly = 1<<0;
    }
}

unsafe impl Encode for NSPasteboardContentsOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPasteboardContentsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardreadingoptionkey?language=objc)
// NS_TYPED_ENUM
pub type NSPasteboardReadingOptionKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardurlreadingfileurlsonlykey?language=objc)
    pub static NSPasteboardURLReadingFileURLsOnlyKey: &'static NSPasteboardReadingOptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardurlreadingcontentsconformtotypeskey?language=objc)
    pub static NSPasteboardURLReadingContentsConformToTypesKey:
        &'static NSPasteboardReadingOptionKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboard?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPasteboard;
);

unsafe impl NSObjectProtocol for NSPasteboard {}

extern_methods!(
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Retained<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithName:)]
        pub unsafe fn pasteboardWithName(name: &NSPasteboardName) -> Retained<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Retained<NSPasteboard>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSPasteboardName>;

        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;

        #[method(prepareForNewContentsWithOptions:)]
        pub unsafe fn prepareForNewContentsWithOptions(
            &self,
            options: NSPasteboardContentsOptions,
        ) -> NSInteger;

        #[method(clearContents)]
        pub unsafe fn clearContents(&self) -> NSInteger;

        #[method(writeObjects:)]
        pub unsafe fn writeObjects(
            &self,
            objects: &NSArray<ProtocolObject<dyn NSPasteboardWriting>>,
        ) -> bool;

        #[method_id(@__retain_semantics Other readObjectsForClasses:options:)]
        pub unsafe fn readObjectsForClasses_options(
            &self,
            class_array: &NSArray<AnyClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, AnyObject>>,
        ) -> Option<Retained<NSArray>>;

        #[cfg(feature = "NSPasteboardItem")]
        #[method_id(@__retain_semantics Other pasteboardItems)]
        pub unsafe fn pasteboardItems(&self) -> Option<Retained<NSArray<NSPasteboardItem>>>;

        #[cfg(feature = "NSPasteboardItem")]
        #[method(indexOfPasteboardItem:)]
        pub unsafe fn indexOfPasteboardItem(
            &self,
            pasteboard_item: &NSPasteboardItem,
        ) -> NSUInteger;

        #[method(canReadItemWithDataConformingToTypes:)]
        pub unsafe fn canReadItemWithDataConformingToTypes(
            &self,
            types: &NSArray<NSString>,
        ) -> bool;

        #[method(canReadObjectForClasses:options:)]
        pub unsafe fn canReadObjectForClasses_options(
            &self,
            class_array: &NSArray<AnyClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, AnyObject>>,
        ) -> bool;

        #[method(declareTypes:owner:)]
        pub unsafe fn declareTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&AnyObject>,
        ) -> NSInteger;

        #[method(addTypes:owner:)]
        pub unsafe fn addTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&AnyObject>,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Option<Retained<NSArray<NSPasteboardType>>>;

        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Retained<NSPasteboardType>>;

        #[method(setData:forType:)]
        pub unsafe fn setData_forType(
            &self,
            data: Option<&NSData>,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            plist: &AnyObject,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(&self, data_type: &NSPasteboardType) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub fn propertyListForType(
            &self,
            data_type: &NSPasteboardType,
        ) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(
            &self,
            data_type: &NSPasteboardType,
        ) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// FilterServices
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other typesFilterableTo:)]
        pub unsafe fn typesFilterableTo(
            r#type: &NSPasteboardType,
        ) -> Retained<NSArray<NSPasteboardType>>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringFile:)]
        pub unsafe fn pasteboardByFilteringFile(filename: &NSString) -> Retained<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringData:ofType:)]
        pub unsafe fn pasteboardByFilteringData_ofType(
            data: &NSData,
            r#type: &NSPasteboardType,
        ) -> Retained<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringTypesInPasteboard:)]
        pub unsafe fn pasteboardByFilteringTypesInPasteboard(
            pboard: &NSPasteboard,
        ) -> Retained<NSPasteboard>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypeowner?language=objc)
    pub unsafe trait NSPasteboardTypeOwner: NSObjectProtocol {
        #[method(pasteboard:provideDataForType:)]
        unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            r#type: &NSPasteboardType,
        );

        #[optional]
        #[method(pasteboardChangedOwner:)]
        unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }

    unsafe impl ProtocolType for dyn NSPasteboardTypeOwner {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardwritingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPasteboardWritingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPasteboardWritingOptions: NSUInteger {
        const NSPasteboardWritingPromised = 1<<9;
    }
}

unsafe impl Encode for NSPasteboardWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPasteboardWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardwriting?language=objc)
    pub unsafe trait NSPasteboardWriting: NSObjectProtocol {
        #[method_id(@__retain_semantics Other writableTypesForPasteboard:)]
        unsafe fn writableTypesForPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Retained<NSArray<NSPasteboardType>>;

        #[optional]
        #[method(writingOptionsForType:pasteboard:)]
        unsafe fn writingOptionsForType_pasteboard(
            &self,
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardWritingOptions;

        #[method_id(@__retain_semantics Other pasteboardPropertyListForType:)]
        unsafe fn pasteboardPropertyListForType(
            &self,
            r#type: &NSPasteboardType,
        ) -> Option<Retained<AnyObject>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardWriting {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardreadingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPasteboardReadingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSPasteboardReadingOptions: NSUInteger {
        const NSPasteboardReadingAsData = 0;
        const NSPasteboardReadingAsString = 1<<0;
        const NSPasteboardReadingAsPropertyList = 1<<1;
        const NSPasteboardReadingAsKeyedArchive = 1<<2;
    }
}

unsafe impl Encode for NSPasteboardReadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPasteboardReadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardreading?language=objc)
    pub unsafe trait NSPasteboardReading: NSObjectProtocol {
        #[method_id(@__retain_semantics Other readableTypesForPasteboard:)]
        unsafe fn readableTypesForPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Retained<NSArray<NSPasteboardType>>;

        #[optional]
        #[method(readingOptionsForType:pasteboard:)]
        unsafe fn readingOptionsForType_pasteboard(
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardReadingOptions;

        #[optional]
        #[method_id(@__retain_semantics Init initWithPasteboardPropertyList:ofType:)]
        unsafe fn initWithPasteboardPropertyList_ofType(
            this: Allocated<Self>,
            property_list: &AnyObject,
            r#type: &NSPasteboardType,
        ) -> Option<Retained<Self>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardReading {}
);

extern_category!(
    /// Category "NSPasteboardSupport" on [`NSURL`].
    #[doc(alias = "NSPasteboardSupport")]
    pub unsafe trait NSURLNSPasteboardSupport {
        #[method_id(@__retain_semantics Other URLFromPasteboard:)]
        unsafe fn URLFromPasteboard(paste_board: &NSPasteboard) -> Option<Retained<NSURL>>;

        #[method(writeToPasteboard:)]
        unsafe fn writeToPasteboard(&self, paste_board: &NSPasteboard);
    }

    unsafe impl NSURLNSPasteboardSupport for NSURL {}
);

unsafe impl NSPasteboardReading for NSURL {}

unsafe impl NSPasteboardWriting for NSURL {}

unsafe impl NSPasteboardReading for NSString {}

unsafe impl NSPasteboardWriting for NSString {}

extern_methods!(
    /// NSFileContents
    unsafe impl NSPasteboard {
        #[method(writeFileContents:)]
        pub unsafe fn writeFileContents(&self, filename: &NSString) -> bool;

        #[method_id(@__retain_semantics Other readFileContentsType:toFile:)]
        pub unsafe fn readFileContentsType_toFile(
            &self,
            r#type: Option<&NSPasteboardType>,
            filename: &NSString,
        ) -> Option<Retained<NSString>>;

        #[method(writeFileWrapper:)]
        pub unsafe fn writeFileWrapper(&self, wrapper: &NSFileWrapper) -> bool;

        #[method_id(@__retain_semantics Other readFileWrapper)]
        pub unsafe fn readFileWrapper(&self) -> Option<Retained<NSFileWrapper>>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilecontentspboardtype?language=objc)
    pub static NSFileContentsPboardType: &'static NSPasteboardType;
}

extern "C-unwind" {
    pub fn NSCreateFilenamePboardType(file_type: &NSString) -> *mut NSPasteboardType;
}

extern "C-unwind" {
    pub fn NSCreateFileContentsPboardType(file_type: &NSString) -> *mut NSPasteboardType;
}

extern "C-unwind" {
    pub fn NSGetFileType(pboard_type: &NSPasteboardType) -> *mut NSString;
}

extern "C-unwind" {
    pub fn NSGetFileTypes(pboard_types: &NSArray<NSPasteboardType>) -> *mut NSArray<NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstringpboardtype?language=objc)
    pub static NSStringPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilenamespboardtype?language=objc)
    pub static NSFilenamesPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstiffpboardtype?language=objc)
    pub static NSTIFFPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrtfpboardtype?language=objc)
    pub static NSRTFPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstabulartextpboardtype?language=objc)
    pub static NSTabularTextPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpboardtype?language=objc)
    pub static NSFontPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerpboardtype?language=objc)
    pub static NSRulerPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorpboardtype?language=objc)
    pub static NSColorPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrtfdpboardtype?language=objc)
    pub static NSRTFDPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nshtmlpboardtype?language=objc)
    pub static NSHTMLPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsurlpboardtype?language=objc)
    pub static NSURLPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspdfpboardtype?language=objc)
    pub static NSPDFPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmultipletextselectionpboardtype?language=objc)
    pub static NSMultipleTextSelectionPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspostscriptpboardtype?language=objc)
    pub static NSPostScriptPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsvcardpboardtype?language=objc)
    pub static NSVCardPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsinktextpboardtype?language=objc)
    pub static NSInkTextPboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilespromisepboardtype?language=objc)
    pub static NSFilesPromisePboardType: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspasteboardtypefindpanelsearchoptions?language=objc)
    pub static NSPasteboardTypeFindPanelSearchOptions: &'static NSPasteboardType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgeneralpboard?language=objc)
    pub static NSGeneralPboard: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfontpboard?language=objc)
    pub static NSFontPboard: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrulerpboard?language=objc)
    pub static NSRulerPboard: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfindpboard?language=objc)
    pub static NSFindPboard: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdragpboard?language=objc)
    pub static NSDragPboard: &'static NSPasteboardName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspictpboardtype?language=objc)
    pub static NSPICTPboardType: &'static NSPasteboardType;
}
