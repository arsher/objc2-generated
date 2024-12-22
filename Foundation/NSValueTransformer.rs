//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsvaluetransformername?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSValueTransformerName = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnegatebooleantransformername?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNegateBooleanTransformerName: &'static NSValueTransformerName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsisniltransformername?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIsNilTransformerName: &'static NSValueTransformerName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsisnotniltransformername?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIsNotNilTransformerName: &'static NSValueTransformerName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsunarchivefromdatatransformername?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSUnarchiveFromDataTransformerName: &'static NSValueTransformerName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyedunarchivefromdatatransformername?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSKeyedUnarchiveFromDataTransformerName: &'static NSValueTransformerName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nssecureunarchivefromdatatransformername?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSSecureUnarchiveFromDataTransformerName: &'static NSValueTransformerName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsvaluetransformer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSValueTransformer;
);

unsafe impl NSObjectProtocol for NSValueTransformer {}

extern_methods!(
    unsafe impl NSValueTransformer {
        #[cfg(feature = "NSString")]
        #[method(setValueTransformer:forName:)]
        pub unsafe fn setValueTransformer_forName(
            transformer: Option<&NSValueTransformer>,
            name: &NSValueTransformerName,
        );

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other valueTransformerForName:)]
        pub unsafe fn valueTransformerForName(
            name: &NSValueTransformerName,
        ) -> Option<Retained<NSValueTransformer>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other valueTransformerNames)]
        pub unsafe fn valueTransformerNames() -> Retained<NSArray<NSValueTransformerName>>;

        #[method(transformedValueClass)]
        pub unsafe fn transformedValueClass() -> &'static AnyClass;

        #[method(allowsReverseTransformation)]
        pub unsafe fn allowsReverseTransformation() -> bool;

        #[method_id(@__retain_semantics Other transformedValue:)]
        pub unsafe fn transformedValue(
            &self,
            value: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other reverseTransformedValue:)]
        pub unsafe fn reverseTransformedValue(
            &self,
            value: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSValueTransformer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A value transformer which transforms values to and from
    /// `NSData`by archiving and unarchiving using secure coding.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nssecureunarchivefromdatatransformer?language=objc)
    #[unsafe(super(NSValueTransformer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSecureUnarchiveFromDataTransformer;
);

unsafe impl NSObjectProtocol for NSSecureUnarchiveFromDataTransformer {}

extern_methods!(
    unsafe impl NSSecureUnarchiveFromDataTransformer {
        #[cfg(feature = "NSArray")]
        /// The list of allowable classes which the top-level object in the archive must conform to on encoding and decoding.
        ///
        /// Returns the result of
        /// `+transformedValueClass`if not
        /// `Nil;`otherwise, currently returns
        /// `NSArray,``NSDictionary,``NSSet,``NSString,``NSNumber,``NSDate,``NSData,``NSURL,``NSUUID,`and
        /// `NSNull.`
        /// Can be overridden by subclasses to provide an expanded or different set of allowed transformation classes.
        #[method_id(@__retain_semantics Other allowedTopLevelClasses)]
        pub unsafe fn allowedTopLevelClasses() -> Retained<NSArray<AnyClass>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSecureUnarchiveFromDataTransformer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
