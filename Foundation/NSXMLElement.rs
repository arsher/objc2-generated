//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXMLElement")]
    pub struct NSXMLElement;

    #[cfg(feature = "Foundation_NSXMLElement")]
    unsafe impl ClassType for NSXMLElement {
        #[inherits(NSObject)]
        type Super = NSXMLNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXMLElement")]
unsafe impl NSCopying for NSXMLElement {}

#[cfg(feature = "Foundation_NSXMLElement")]
unsafe impl NSObjectProtocol for NSXMLElement {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLElement")]
    unsafe impl NSXMLElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(this: Option<Allocated<Self>>, name: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:URI:)]
        pub unsafe fn initWithName_URI(
            this: Option<Allocated<Self>>,
            name: &NSString,
            uri: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:stringValue:)]
        pub unsafe fn initWithName_stringValue(
            this: Option<Allocated<Self>>,
            name: &NSString,
            string: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithXMLString:error:_)]
        pub unsafe fn initWithXMLString_error(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other elementsForName:)]
        pub unsafe fn elementsForName(&self, name: &NSString) -> Id<NSArray<NSXMLElement>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other elementsForLocalName:URI:)]
        pub unsafe fn elementsForLocalName_URI(
            &self,
            local_name: &NSString,
            uri: Option<&NSString>,
        ) -> Id<NSArray<NSXMLElement>>;

        #[method(addAttribute:)]
        pub unsafe fn addAttribute(&self, attribute: &NSXMLNode);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeAttributeForName:)]
        pub unsafe fn removeAttributeForName(&self, name: &NSString);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Option<Id<NSArray<NSXMLNode>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: Option<&NSArray<NSXMLNode>>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setAttributesWithDictionary:)]
        pub unsafe fn setAttributesWithDictionary(
            &self,
            attributes: &NSDictionary<NSString, NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attributeForName:)]
        pub unsafe fn attributeForName(&self, name: &NSString) -> Option<Id<NSXMLNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attributeForLocalName:URI:)]
        pub unsafe fn attributeForLocalName_URI(
            &self,
            local_name: &NSString,
            uri: Option<&NSString>,
        ) -> Option<Id<NSXMLNode>>;

        #[method(addNamespace:)]
        pub unsafe fn addNamespace(&self, a_namespace: &NSXMLNode);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeNamespaceForPrefix:)]
        pub unsafe fn removeNamespaceForPrefix(&self, name: &NSString);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other namespaces)]
        pub unsafe fn namespaces(&self) -> Option<Id<NSArray<NSXMLNode>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setNamespaces:)]
        pub unsafe fn setNamespaces(&self, namespaces: Option<&NSArray<NSXMLNode>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other namespaceForPrefix:)]
        pub unsafe fn namespaceForPrefix(&self, name: &NSString) -> Option<Id<NSXMLNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other resolveNamespaceForName:)]
        pub unsafe fn resolveNamespaceForName(&self, name: &NSString) -> Option<Id<NSXMLNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other resolvePrefixForNamespaceURI:)]
        pub unsafe fn resolvePrefixForNamespaceURI(
            &self,
            namespace_uri: &NSString,
        ) -> Option<Id<NSString>>;

        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );

        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);

        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);

        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);

        #[method(normalizeAdjacentTextNodesPreservingCDATA:)]
        pub unsafe fn normalizeAdjacentTextNodesPreservingCDATA(&self, preserve: bool);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSXMLElement")]
    unsafe impl NSXMLElement {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated]
        #[method(setAttributesAsDictionary:)]
        pub unsafe fn setAttributesAsDictionary(&self, attributes: &NSDictionary);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "Foundation_NSXMLElement")]
    unsafe impl NSXMLElement {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(this: Option<Allocated<Self>>, kind: NSXMLNodeKind) -> Id<Self>;
    }
);
