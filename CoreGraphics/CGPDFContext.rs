//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C-unwind" {
    #[cfg(all(
        feature = "CGContext",
        feature = "CGDataConsumer",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGPDFContextCreate(
        consumer: CGDataConsumerRef,
        media_box: *mut CGRect,
        auxiliary_info: CFDictionaryRef,
    ) -> CGContextRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextCreateWithURL(
        url: CFURLRef,
        media_box: *mut CGRect,
        auxiliary_info: CFDictionaryRef,
    ) -> CGContextRef;
}

extern "C-unwind" {
    #[cfg(feature = "CGContext")]
    pub fn CGPDFContextClose(context: CGContextRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextBeginPage(context: CGContextRef, page_info: CFDictionaryRef);
}

extern "C-unwind" {
    #[cfg(feature = "CGContext")]
    pub fn CGPDFContextEndPage(context: CGContextRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextAddDocumentMetadata(context: CGContextRef, metadata: CFDataRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "CGPDFDictionary"))]
    pub fn CGPDFContextSetParentTree(
        context: CGContextRef,
        parent_tree_dictionary: CGPDFDictionaryRef,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "CGPDFDictionary"))]
    pub fn CGPDFContextSetIDTree(context: CGContextRef, id_tree_dictionary: CGPDFDictionaryRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextSetPageTagStructureTree(
        context: CGContextRef,
        page_tag_structure_tree_dictionary: CFDictionaryRef,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextSetURLForRect(context: CGContextRef, url: CFURLRef, rect: CGRect);
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextAddDestinationAtPoint(
        context: CGContextRef,
        name: CFStringRef,
        point: CGPoint,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextSetDestinationForRect(
        context: CGContextRef,
        name: CFStringRef,
        rect: CGRect,
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextmediabox?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextMediaBox: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextcropbox?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextCropBox: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextbleedbox?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextBleedBox: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontexttrimbox?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextTrimBox: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextartbox?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextArtBox: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontexttitle?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextTitle: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextauthor?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextAuthor: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextsubject?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextSubject: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextkeywords?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextKeywords: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextcreator?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextCreator: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextownerpassword?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextOwnerPassword: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextuserpassword?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextUserPassword: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextencryptionkeylength?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextEncryptionKeyLength: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextallowsprinting?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextAllowsPrinting: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextallowscopying?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextAllowsCopying: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextoutputintent?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextOutputIntent: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfxoutputintentsubtype?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFXOutputIntentSubtype: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfxoutputconditionidentifier?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFXOutputConditionIdentifier: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfxoutputcondition?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFXOutputCondition: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfxregistryname?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFXRegistryName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfxinfo?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFXInfo: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfxdestinationoutputprofile?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFXDestinationOutputProfile: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextoutputintents?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextOutputIntents: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextaccesspermissions?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextAccessPermissions: CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextSetOutline(context: CGContextRef, outline: CFDictionaryRef);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextcreatelinearizedpdf?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextCreateLinearizedPDF: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdfcontextcreatepdfa?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFContextCreatePDFA: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdftagtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGPDFTagType(pub i32);
impl CGPDFTagType {
    #[doc(alias = "CGPDFTagTypeDocument")]
    pub const Document: Self = Self(100);
    #[doc(alias = "CGPDFTagTypePart")]
    pub const Part: Self = Self(101);
    #[doc(alias = "CGPDFTagTypeArt")]
    pub const Art: Self = Self(102);
    #[doc(alias = "CGPDFTagTypeSection")]
    pub const Section: Self = Self(103);
    #[doc(alias = "CGPDFTagTypeDiv")]
    pub const Div: Self = Self(104);
    #[doc(alias = "CGPDFTagTypeBlockQuote")]
    pub const BlockQuote: Self = Self(105);
    #[doc(alias = "CGPDFTagTypeCaption")]
    pub const Caption: Self = Self(106);
    #[doc(alias = "CGPDFTagTypeTOC")]
    pub const TOC: Self = Self(107);
    #[doc(alias = "CGPDFTagTypeTOCI")]
    pub const TOCI: Self = Self(108);
    #[doc(alias = "CGPDFTagTypeIndex")]
    pub const Index: Self = Self(109);
    #[doc(alias = "CGPDFTagTypeNonStructure")]
    pub const NonStructure: Self = Self(110);
    #[doc(alias = "CGPDFTagTypePrivate")]
    pub const Private: Self = Self(111);
    #[doc(alias = "CGPDFTagTypeParagraph")]
    pub const Paragraph: Self = Self(200);
    #[doc(alias = "CGPDFTagTypeHeader")]
    pub const Header: Self = Self(201);
    #[doc(alias = "CGPDFTagTypeHeader1")]
    pub const Header1: Self = Self(202);
    #[doc(alias = "CGPDFTagTypeHeader2")]
    pub const Header2: Self = Self(203);
    #[doc(alias = "CGPDFTagTypeHeader3")]
    pub const Header3: Self = Self(204);
    #[doc(alias = "CGPDFTagTypeHeader4")]
    pub const Header4: Self = Self(205);
    #[doc(alias = "CGPDFTagTypeHeader5")]
    pub const Header5: Self = Self(206);
    #[doc(alias = "CGPDFTagTypeHeader6")]
    pub const Header6: Self = Self(207);
    #[doc(alias = "CGPDFTagTypeList")]
    pub const List: Self = Self(300);
    #[doc(alias = "CGPDFTagTypeListItem")]
    pub const ListItem: Self = Self(301);
    #[doc(alias = "CGPDFTagTypeLabel")]
    pub const Label: Self = Self(302);
    #[doc(alias = "CGPDFTagTypeListBody")]
    pub const ListBody: Self = Self(303);
    #[doc(alias = "CGPDFTagTypeTable")]
    pub const Table: Self = Self(400);
    #[doc(alias = "CGPDFTagTypeTableRow")]
    pub const TableRow: Self = Self(401);
    #[doc(alias = "CGPDFTagTypeTableHeaderCell")]
    pub const TableHeaderCell: Self = Self(402);
    #[doc(alias = "CGPDFTagTypeTableDataCell")]
    pub const TableDataCell: Self = Self(403);
    #[doc(alias = "CGPDFTagTypeTableHeader")]
    pub const TableHeader: Self = Self(404);
    #[doc(alias = "CGPDFTagTypeTableBody")]
    pub const TableBody: Self = Self(405);
    #[doc(alias = "CGPDFTagTypeTableFooter")]
    pub const TableFooter: Self = Self(406);
    #[doc(alias = "CGPDFTagTypeSpan")]
    pub const Span: Self = Self(500);
    #[doc(alias = "CGPDFTagTypeQuote")]
    pub const Quote: Self = Self(501);
    #[doc(alias = "CGPDFTagTypeNote")]
    pub const Note: Self = Self(502);
    #[doc(alias = "CGPDFTagTypeReference")]
    pub const Reference: Self = Self(503);
    #[doc(alias = "CGPDFTagTypeBibliography")]
    pub const Bibliography: Self = Self(504);
    #[doc(alias = "CGPDFTagTypeCode")]
    pub const Code: Self = Self(505);
    #[doc(alias = "CGPDFTagTypeLink")]
    pub const Link: Self = Self(506);
    #[doc(alias = "CGPDFTagTypeAnnotation")]
    pub const Annotation: Self = Self(507);
    #[doc(alias = "CGPDFTagTypeRuby")]
    pub const Ruby: Self = Self(600);
    #[doc(alias = "CGPDFTagTypeRubyBaseText")]
    pub const RubyBaseText: Self = Self(601);
    #[doc(alias = "CGPDFTagTypeRubyAnnotationText")]
    pub const RubyAnnotationText: Self = Self(602);
    #[doc(alias = "CGPDFTagTypeRubyPunctuation")]
    pub const RubyPunctuation: Self = Self(603);
    #[doc(alias = "CGPDFTagTypeWarichu")]
    pub const Warichu: Self = Self(604);
    #[doc(alias = "CGPDFTagTypeWarichuText")]
    pub const WarichuText: Self = Self(605);
    #[doc(alias = "CGPDFTagTypeWarichuPunctiation")]
    pub const WarichuPunctiation: Self = Self(606);
    #[doc(alias = "CGPDFTagTypeFigure")]
    pub const Figure: Self = Self(700);
    #[doc(alias = "CGPDFTagTypeFormula")]
    pub const Formula: Self = Self(701);
    #[doc(alias = "CGPDFTagTypeForm")]
    pub const Form: Self = Self(702);
    #[doc(alias = "CGPDFTagTypeObject")]
    pub const Object: Self = Self(800);
}

unsafe impl Encode for CGPDFTagType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CGPDFTagType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn CGPDFTagTypeGetName(tag_type: CGPDFTagType) -> *mut c_char;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdftagproperty?language=objc)
// NS_TYPED_ENUM
#[cfg(feature = "objc2-core-foundation")]
pub type CGPDFTagProperty = CFStringRef;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdftagpropertyactualtext?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFTagPropertyActualText: CGPDFTagProperty;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdftagpropertyalternativetext?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFTagPropertyAlternativeText: CGPDFTagProperty;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdftagpropertytitletext?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFTagPropertyTitleText: CGPDFTagProperty;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgpdftagpropertylanguagetext?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGPDFTagPropertyLanguageText: CGPDFTagProperty;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGContext", feature = "objc2-core-foundation"))]
    pub fn CGPDFContextBeginTag(
        context: CGContextRef,
        tag_type: CGPDFTagType,
        tag_properties: CFDictionaryRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CGContext")]
    pub fn CGPDFContextEndTag(context: CGContextRef);
}
