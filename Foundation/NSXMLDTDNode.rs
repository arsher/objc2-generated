//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsxmldtdnodekind?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSXMLDTDNodeKind(pub NSUInteger);
impl NSXMLDTDNodeKind {
    pub const NSXMLEntityGeneralKind: Self = Self(1);
    pub const NSXMLEntityParsedKind: Self = Self(2);
    pub const NSXMLEntityUnparsedKind: Self = Self(3);
    pub const NSXMLEntityParameterKind: Self = Self(4);
    pub const NSXMLEntityPredefined: Self = Self(5);
    pub const NSXMLAttributeCDATAKind: Self = Self(6);
    pub const NSXMLAttributeIDKind: Self = Self(7);
    pub const NSXMLAttributeIDRefKind: Self = Self(8);
    pub const NSXMLAttributeIDRefsKind: Self = Self(9);
    pub const NSXMLAttributeEntityKind: Self = Self(10);
    pub const NSXMLAttributeEntitiesKind: Self = Self(11);
    pub const NSXMLAttributeNMTokenKind: Self = Self(12);
    pub const NSXMLAttributeNMTokensKind: Self = Self(13);
    pub const NSXMLAttributeEnumerationKind: Self = Self(14);
    pub const NSXMLAttributeNotationKind: Self = Self(15);
    pub const NSXMLElementDeclarationUndefinedKind: Self = Self(16);
    pub const NSXMLElementDeclarationEmptyKind: Self = Self(17);
    pub const NSXMLElementDeclarationAnyKind: Self = Self(18);
    pub const NSXMLElementDeclarationMixedKind: Self = Self(19);
    pub const NSXMLElementDeclarationElementKind: Self = Self(20);
}

unsafe impl Encode for NSXMLDTDNodeKind {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSXMLDTDNodeKind {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsxmldtdnode?language=objc)
    #[unsafe(super(NSXMLNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSXMLNode")]
    pub struct NSXMLDTDNode;
);

#[cfg(all(feature = "NSObject", feature = "NSXMLNode"))]
unsafe impl NSCopying for NSXMLDTDNode {}

#[cfg(all(feature = "NSObject", feature = "NSXMLNode"))]
unsafe impl CopyingHelper for NSXMLDTDNode {
    type Result = Self;
}

#[cfg(feature = "NSXMLNode")]
unsafe impl NSObjectProtocol for NSXMLDTDNode {}

extern_methods!(
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLDTDNode {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithXMLString:)]
        pub unsafe fn initWithXMLString(
            this: Allocated<Self>,
            string: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSXMLNodeOptions")]
        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Allocated<Self>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(DTDKind)]
        pub unsafe fn DTDKind(&self) -> NSXMLDTDNodeKind;

        #[method(setDTDKind:)]
        pub unsafe fn setDTDKind(&self, dtd_kind: NSXMLDTDNodeKind);

        #[method(isExternal)]
        pub unsafe fn isExternal(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other publicID)]
        pub unsafe fn publicID(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, public_id: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other systemID)]
        pub unsafe fn systemID(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, system_id: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other notationName)]
        pub unsafe fn notationName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setNotationName:)]
        pub unsafe fn setNotationName(&self, notation_name: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLDTDNode {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(this: Allocated<Self>, kind: NSXMLNodeKind) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSXMLNode")]
    unsafe impl NSXMLDTDNode {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
