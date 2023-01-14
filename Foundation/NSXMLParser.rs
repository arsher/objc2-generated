//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSXMLParserExternalEntityResolvingPolicy {
        NSXMLParserResolveExternalEntitiesNever = 0,
        NSXMLParserResolveExternalEntitiesNoNetwork = 1,
        NSXMLParserResolveExternalEntitiesSameOriginOnly = 2,
        NSXMLParserResolveExternalEntitiesAlways = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXMLParser")]
    pub struct NSXMLParser;

    #[cfg(feature = "Foundation_NSXMLParser")]
    unsafe impl ClassType for NSXMLParser {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLParser")]
    unsafe impl NSXMLParser {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSInputStream")]
        #[method_id(@__retain_semantics Init initWithStream:)]
        pub unsafe fn initWithStream(
            this: Option<Allocated<Self>>,
            stream: &NSInputStream,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSXMLParserDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSXMLParserDelegate>);

        #[method(shouldProcessNamespaces)]
        pub unsafe fn shouldProcessNamespaces(&self) -> bool;

        #[method(setShouldProcessNamespaces:)]
        pub unsafe fn setShouldProcessNamespaces(&self, shouldProcessNamespaces: bool);

        #[method(shouldReportNamespacePrefixes)]
        pub unsafe fn shouldReportNamespacePrefixes(&self) -> bool;

        #[method(setShouldReportNamespacePrefixes:)]
        pub unsafe fn setShouldReportNamespacePrefixes(&self, shouldReportNamespacePrefixes: bool);

        #[method(externalEntityResolvingPolicy)]
        pub unsafe fn externalEntityResolvingPolicy(
            &self,
        ) -> NSXMLParserExternalEntityResolvingPolicy;

        #[method(setExternalEntityResolvingPolicy:)]
        pub unsafe fn setExternalEntityResolvingPolicy(
            &self,
            externalEntityResolvingPolicy: NSXMLParserExternalEntityResolvingPolicy,
        );

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other allowedExternalEntityURLs)]
        pub unsafe fn allowedExternalEntityURLs(&self) -> Option<Id<NSSet<NSURL>, Shared>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSURL"))]
        #[method(setAllowedExternalEntityURLs:)]
        pub unsafe fn setAllowedExternalEntityURLs(
            &self,
            allowedExternalEntityURLs: Option<&NSSet<NSURL>>,
        );

        #[method(parse)]
        pub unsafe fn parse(&self) -> bool;

        #[method(abortParsing)]
        pub unsafe fn abortParsing(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other parserError)]
        pub unsafe fn parserError(&self) -> Option<Id<NSError, Shared>>;

        #[method(shouldResolveExternalEntities)]
        pub unsafe fn shouldResolveExternalEntities(&self) -> bool;

        #[method(setShouldResolveExternalEntities:)]
        pub unsafe fn setShouldResolveExternalEntities(&self, shouldResolveExternalEntities: bool);
    }
);

extern_methods!(
    /// NSXMLParserLocatorAdditions
    #[cfg(feature = "Foundation_NSXMLParser")]
    unsafe impl NSXMLParser {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>>;

        #[method(lineNumber)]
        pub unsafe fn lineNumber(&self) -> NSInteger;

        #[method(columnNumber)]
        pub unsafe fn columnNumber(&self) -> NSInteger;
    }
);

extern_protocol!(
    pub struct NSXMLParserDelegate;

    unsafe impl ProtocolType for NSXMLParserDelegate {
        #[cfg(feature = "Foundation_NSXMLParser")]
        #[optional]
        #[method(parserDidStartDocument:)]
        pub unsafe fn parserDidStartDocument(&self, parser: &NSXMLParser);

        #[cfg(feature = "Foundation_NSXMLParser")]
        #[optional]
        #[method(parserDidEndDocument:)]
        pub unsafe fn parserDidEndDocument(&self, parser: &NSXMLParser);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundNotationDeclarationWithName:publicID:systemID:)]
        pub unsafe fn parser_foundNotationDeclarationWithName_publicID_systemID(
            &self,
            parser: &NSXMLParser,
            name: &NSString,
            publicID: Option<&NSString>,
            systemID: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundUnparsedEntityDeclarationWithName:publicID:systemID:notationName:)]
        pub unsafe fn parser_foundUnparsedEntityDeclarationWithName_publicID_systemID_notationName(
            &self,
            parser: &NSXMLParser,
            name: &NSString,
            publicID: Option<&NSString>,
            systemID: Option<&NSString>,
            notationName: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundAttributeDeclarationWithName:forElement:type:defaultValue:)]
        pub unsafe fn parser_foundAttributeDeclarationWithName_forElement_type_defaultValue(
            &self,
            parser: &NSXMLParser,
            attributeName: &NSString,
            elementName: &NSString,
            r#type: Option<&NSString>,
            defaultValue: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundElementDeclarationWithName:model:)]
        pub unsafe fn parser_foundElementDeclarationWithName_model(
            &self,
            parser: &NSXMLParser,
            elementName: &NSString,
            model: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundInternalEntityDeclarationWithName:value:)]
        pub unsafe fn parser_foundInternalEntityDeclarationWithName_value(
            &self,
            parser: &NSXMLParser,
            name: &NSString,
            value: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundExternalEntityDeclarationWithName:publicID:systemID:)]
        pub unsafe fn parser_foundExternalEntityDeclarationWithName_publicID_systemID(
            &self,
            parser: &NSXMLParser,
            name: &NSString,
            publicID: Option<&NSString>,
            systemID: Option<&NSString>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSXMLParser"
        ))]
        #[optional]
        #[method(parser:didStartElement:namespaceURI:qualifiedName:attributes:)]
        pub unsafe fn parser_didStartElement_namespaceURI_qualifiedName_attributes(
            &self,
            parser: &NSXMLParser,
            elementName: &NSString,
            namespaceURI: Option<&NSString>,
            qName: Option<&NSString>,
            attributeDict: &NSDictionary<NSString, NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:didEndElement:namespaceURI:qualifiedName:)]
        pub unsafe fn parser_didEndElement_namespaceURI_qualifiedName(
            &self,
            parser: &NSXMLParser,
            elementName: &NSString,
            namespaceURI: Option<&NSString>,
            qName: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:didStartMappingPrefix:toURI:)]
        pub unsafe fn parser_didStartMappingPrefix_toURI(
            &self,
            parser: &NSXMLParser,
            prefix: &NSString,
            namespaceURI: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:didEndMappingPrefix:)]
        pub unsafe fn parser_didEndMappingPrefix(&self, parser: &NSXMLParser, prefix: &NSString);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundCharacters:)]
        pub unsafe fn parser_foundCharacters(&self, parser: &NSXMLParser, string: &NSString);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundIgnorableWhitespace:)]
        pub unsafe fn parser_foundIgnorableWhitespace(
            &self,
            parser: &NSXMLParser,
            whitespaceString: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundProcessingInstructionWithTarget:data:)]
        pub unsafe fn parser_foundProcessingInstructionWithTarget_data(
            &self,
            parser: &NSXMLParser,
            target: &NSString,
            data: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundComment:)]
        pub unsafe fn parser_foundComment(&self, parser: &NSXMLParser, comment: &NSString);

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:foundCDATA:)]
        pub unsafe fn parser_foundCDATA(&self, parser: &NSXMLParser, CDATABlock: &NSData);

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "Foundation_NSXMLParser"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other parser:resolveExternalEntityName:systemID:)]
        pub unsafe fn parser_resolveExternalEntityName_systemID(
            &self,
            parser: &NSXMLParser,
            name: &NSString,
            systemID: Option<&NSString>,
        ) -> Option<Id<NSData, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:parseErrorOccurred:)]
        pub unsafe fn parser_parseErrorOccurred(&self, parser: &NSXMLParser, parseError: &NSError);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSXMLParser"))]
        #[optional]
        #[method(parser:validationErrorOccurred:)]
        pub unsafe fn parser_validationErrorOccurred(
            &self,
            parser: &NSXMLParser,
            validationError: &NSError,
        );
    }
);

extern_static!(NSXMLParserErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSXMLParserError {
        NSXMLParserInternalError = 1,
        NSXMLParserOutOfMemoryError = 2,
        NSXMLParserDocumentStartError = 3,
        NSXMLParserEmptyDocumentError = 4,
        NSXMLParserPrematureDocumentEndError = 5,
        NSXMLParserInvalidHexCharacterRefError = 6,
        NSXMLParserInvalidDecimalCharacterRefError = 7,
        NSXMLParserInvalidCharacterRefError = 8,
        NSXMLParserInvalidCharacterError = 9,
        NSXMLParserCharacterRefAtEOFError = 10,
        NSXMLParserCharacterRefInPrologError = 11,
        NSXMLParserCharacterRefInEpilogError = 12,
        NSXMLParserCharacterRefInDTDError = 13,
        NSXMLParserEntityRefAtEOFError = 14,
        NSXMLParserEntityRefInPrologError = 15,
        NSXMLParserEntityRefInEpilogError = 16,
        NSXMLParserEntityRefInDTDError = 17,
        NSXMLParserParsedEntityRefAtEOFError = 18,
        NSXMLParserParsedEntityRefInPrologError = 19,
        NSXMLParserParsedEntityRefInEpilogError = 20,
        NSXMLParserParsedEntityRefInInternalSubsetError = 21,
        NSXMLParserEntityReferenceWithoutNameError = 22,
        NSXMLParserEntityReferenceMissingSemiError = 23,
        NSXMLParserParsedEntityRefNoNameError = 24,
        NSXMLParserParsedEntityRefMissingSemiError = 25,
        NSXMLParserUndeclaredEntityError = 26,
        NSXMLParserUnparsedEntityError = 28,
        NSXMLParserEntityIsExternalError = 29,
        NSXMLParserEntityIsParameterError = 30,
        NSXMLParserUnknownEncodingError = 31,
        NSXMLParserEncodingNotSupportedError = 32,
        NSXMLParserStringNotStartedError = 33,
        NSXMLParserStringNotClosedError = 34,
        NSXMLParserNamespaceDeclarationError = 35,
        NSXMLParserEntityNotStartedError = 36,
        NSXMLParserEntityNotFinishedError = 37,
        NSXMLParserLessThanSymbolInAttributeError = 38,
        NSXMLParserAttributeNotStartedError = 39,
        NSXMLParserAttributeNotFinishedError = 40,
        NSXMLParserAttributeHasNoValueError = 41,
        NSXMLParserAttributeRedefinedError = 42,
        NSXMLParserLiteralNotStartedError = 43,
        NSXMLParserLiteralNotFinishedError = 44,
        NSXMLParserCommentNotFinishedError = 45,
        NSXMLParserProcessingInstructionNotStartedError = 46,
        NSXMLParserProcessingInstructionNotFinishedError = 47,
        NSXMLParserNotationNotStartedError = 48,
        NSXMLParserNotationNotFinishedError = 49,
        NSXMLParserAttributeListNotStartedError = 50,
        NSXMLParserAttributeListNotFinishedError = 51,
        NSXMLParserMixedContentDeclNotStartedError = 52,
        NSXMLParserMixedContentDeclNotFinishedError = 53,
        NSXMLParserElementContentDeclNotStartedError = 54,
        NSXMLParserElementContentDeclNotFinishedError = 55,
        NSXMLParserXMLDeclNotStartedError = 56,
        NSXMLParserXMLDeclNotFinishedError = 57,
        NSXMLParserConditionalSectionNotStartedError = 58,
        NSXMLParserConditionalSectionNotFinishedError = 59,
        NSXMLParserExternalSubsetNotFinishedError = 60,
        NSXMLParserDOCTYPEDeclNotFinishedError = 61,
        NSXMLParserMisplacedCDATAEndStringError = 62,
        NSXMLParserCDATANotFinishedError = 63,
        NSXMLParserMisplacedXMLDeclarationError = 64,
        NSXMLParserSpaceRequiredError = 65,
        NSXMLParserSeparatorRequiredError = 66,
        NSXMLParserNMTOKENRequiredError = 67,
        NSXMLParserNAMERequiredError = 68,
        NSXMLParserPCDATARequiredError = 69,
        NSXMLParserURIRequiredError = 70,
        NSXMLParserPublicIdentifierRequiredError = 71,
        NSXMLParserLTRequiredError = 72,
        NSXMLParserGTRequiredError = 73,
        NSXMLParserLTSlashRequiredError = 74,
        NSXMLParserEqualExpectedError = 75,
        NSXMLParserTagNameMismatchError = 76,
        NSXMLParserUnfinishedTagError = 77,
        NSXMLParserStandaloneValueError = 78,
        NSXMLParserInvalidEncodingNameError = 79,
        NSXMLParserCommentContainsDoubleHyphenError = 80,
        NSXMLParserInvalidEncodingError = 81,
        NSXMLParserExternalStandaloneEntityError = 82,
        NSXMLParserInvalidConditionalSectionError = 83,
        NSXMLParserEntityValueRequiredError = 84,
        NSXMLParserNotWellBalancedError = 85,
        NSXMLParserExtraContentError = 86,
        NSXMLParserInvalidCharacterInEntityError = 87,
        NSXMLParserParsedEntityRefInInternalError = 88,
        NSXMLParserEntityRefLoopError = 89,
        NSXMLParserEntityBoundaryError = 90,
        NSXMLParserInvalidURIError = 91,
        NSXMLParserURIFragmentError = 92,
        NSXMLParserNoDTDError = 94,
        NSXMLParserDelegateAbortedParseError = 512,
    }
);
