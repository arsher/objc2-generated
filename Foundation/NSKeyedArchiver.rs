//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinvalidarchiveoperationexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSInvalidArchiveOperationException: &'static NSExceptionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinvalidunarchiveoperationexception?language=objc)
    #[cfg(all(feature = "NSObjCRuntime", feature = "NSString"))]
    pub static NSInvalidUnarchiveOperationException: &'static NSExceptionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyedarchiverootobjectkey?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSKeyedArchiveRootObjectKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyedarchiver?language=objc)
    #[unsafe(super(NSCoder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCoder")]
    pub struct NSKeyedArchiver;
);

#[cfg(feature = "NSCoder")]
unsafe impl NSObjectProtocol for NSKeyedArchiver {}

extern_methods!(
    #[cfg(feature = "NSCoder")]
    unsafe impl NSKeyedArchiver {
        /// Initializes the receiver for encoding an archive, optionally disabling secure coding.
        ///
        /// If
        /// `NSSecureCoding`cannot be used,
        /// `requiresSecureCoding`may be turned off here; for improved security, however,
        /// `requiresSecureCoding`should be left enabled whenever possible.
        /// `requiresSecureCoding`ensures that all encoded objects conform to
        /// `NSSecureCoding,`preventing the possibility of encoding objects which cannot be decoded later.
        ///
        /// To produce archives whose structure matches those previously encoded using
        /// `+archivedDataWithRootObject,`encode the top-level object in your archive for the
        /// `NSKeyedArchiveRootObjectKey.`
        #[method_id(@__retain_semantics Init initRequiringSecureCoding:)]
        pub unsafe fn initRequiringSecureCoding(
            this: Allocated<Self>,
            requires_secure_coding: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        /// Returns an
        /// `NSData`object containing the encoded form of the object graph whose root object is given, optionally disabling secure coding.
        ///
        /// If
        /// `NSSecureCoding`cannot be used,
        /// `requiresSecureCoding`may be turned off here; for improved security, however,
        /// `requiresSecureCoding`should be left enabled whenever possible.
        /// `requiresSecureCoding`ensures that all encoded objects conform to
        /// `NSSecureCoding,`preventing the possibility of encoding objects which cannot be decoded later.
        ///
        /// If the object graph cannot be encoded, returns
        /// `nil`and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other archivedDataWithRootObject:requiringSecureCoding:error:_)]
        pub unsafe fn archivedDataWithRootObject_requiringSecureCoding_error(
            object: &AnyObject,
            requires_secure_coding: bool,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        /// Initialize the archiver with empty data, ready for writing.
        #[deprecated = "Use -initRequiringSecureCoding: instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use -initRequiringSecureCoding: instead"]
        #[method_id(@__retain_semantics Init initForWritingWithMutableData:)]
        pub unsafe fn initForWritingWithMutableData(
            this: Allocated<Self>,
            data: &NSMutableData,
        ) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use +archivedDataWithRootObject:requiringSecureCoding:error: instead"]
        #[method_id(@__retain_semantics Other archivedDataWithRootObject:)]
        pub unsafe fn archivedDataWithRootObject(root_object: &AnyObject) -> Retained<NSData>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use +archivedDataWithRootObject:requiringSecureCoding:error: and -writeToURL:options:error: instead"]
        #[method(archiveRootObject:toFile:)]
        pub unsafe fn archiveRootObject_toFile(root_object: &AnyObject, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSKeyedArchiverDelegate>>>;

        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSKeyedArchiverDelegate>>,
        );

        #[cfg(feature = "NSPropertyList")]
        #[method(outputFormat)]
        pub unsafe fn outputFormat(&self) -> NSPropertyListFormat;

        #[cfg(feature = "NSPropertyList")]
        /// Setter for [`outputFormat`][Self::outputFormat].
        #[method(setOutputFormat:)]
        pub unsafe fn setOutputFormat(&self, output_format: NSPropertyListFormat);

        #[cfg(feature = "NSData")]
        /// If encoding has not yet finished, then invoking this property will call finishEncoding and return the data. If you initialized the keyed archiver with a specific mutable data instance, then it will be returned from this property after finishEncoding is called.
        #[method_id(@__retain_semantics Other encodedData)]
        pub unsafe fn encodedData(&self) -> Retained<NSData>;

        #[method(finishEncoding)]
        pub unsafe fn finishEncoding(&self);

        #[cfg(feature = "NSString")]
        #[method(setClassName:forClass:)]
        pub unsafe fn setClassName_forClass_class(coded_name: Option<&NSString>, cls: &AnyClass);

        #[cfg(feature = "NSString")]
        #[method(setClassName:forClass:)]
        pub unsafe fn setClassName_forClass(&self, coded_name: Option<&NSString>, cls: &AnyClass);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other classNameForClass:)]
        pub unsafe fn classNameForClass_class(cls: &AnyClass) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other classNameForClass:)]
        pub unsafe fn classNameForClass(&self, cls: &AnyClass) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&AnyObject>, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&AnyObject>,
            key: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[cfg(feature = "NSString")]
        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *const u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        /// Setter for [`requiresSecureCoding`][Self::requiresSecureCoding].
        #[method(setRequiresSecureCoding:)]
        pub unsafe fn setRequiresSecureCoding(&self, requires_secure_coding: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCoder")]
    unsafe impl NSKeyedArchiver {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyedunarchiver?language=objc)
    #[unsafe(super(NSCoder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCoder")]
    pub struct NSKeyedUnarchiver;
);

#[cfg(feature = "NSCoder")]
unsafe impl NSObjectProtocol for NSKeyedUnarchiver {}

extern_methods!(
    #[cfg(feature = "NSCoder")]
    unsafe impl NSKeyedUnarchiver {
        #[cfg(all(feature = "NSData", feature = "NSError"))]
        /// Initializes the receiver for decoding an archive previously encoded by
        /// `NSKeyedUnarchiver.`
        /// Enables
        /// `requiresSecureCoding`by default. If
        /// `NSSecureCoding`cannot be used,
        /// `requiresSecureCoding`may be turned off manually; for improved security,
        /// `requiresSecureCoding`should be left enabled whenever possible.
        ///
        /// Sets the unarchiver's
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Init initForReadingFromData:error:_)]
        pub unsafe fn initForReadingFromData_error(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        /// Decodes the root object of the given class from the given archive, previously encoded by
        /// `NSKeyedArchiver.`
        /// Enables
        /// `requiresSecureCoding`and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid or cannot be decoded, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other unarchivedObjectOfClass:fromData:error:_)]
        pub unsafe fn unarchivedObjectOfClass_fromData_error(
            cls: &AnyClass,
            data: &NSData,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(feature = "NSArray", feature = "NSData", feature = "NSError"))]
        /// Decodes the
        /// `NSArray`root object from
        /// `data`which should be an
        /// `NSArray<cls>`containing the given non-collection class (no nested arrays or arrays of dictionaries, etc) from the given archive, previously encoded by
        /// `NSKeyedArchiver.`Enables
        /// `requiresSecureCoding`and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid or cannot be decoded, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other unarchivedArrayOfObjectsOfClass:fromData:error:_)]
        pub unsafe fn unarchivedArrayOfObjectsOfClass_fromData_error(
            cls: &AnyClass,
            data: &NSData,
        ) -> Result<Retained<NSArray>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSDictionary", feature = "NSError"))]
        /// Decodes the
        /// `NSDictionary`root object from
        /// `data`which should be an
        /// `NSDictionary<keyCls,objectCls>`with keys of type given in
        /// `keyCls`and objects of the given non-collection class
        /// `objectCls`(no nested dictionaries or other dictionaries contained in the dictionary, etc) from the given archive, previously encoded by
        /// `NSKeyedArchiver.`
        /// Enables
        /// `requiresSecureCoding`and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid or cannot be decoded, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other unarchivedDictionaryWithKeysOfClass:objectsOfClass:fromData:error:_)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClass_objectsOfClass_fromData_error(
            key_cls: &AnyClass,
            value_cls: &AnyClass,
            data: &NSData,
        ) -> Result<Retained<NSDictionary>, Retained<NSError>>;

        #[cfg(all(feature = "NSData", feature = "NSError", feature = "NSSet"))]
        /// Decodes the root object of one of the given classes from the given archive, previously encoded by
        /// `NSKeyedArchiver.`
        /// Enables
        /// `requiresSecureCoding`and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid or cannot be decoded, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other unarchivedObjectOfClasses:fromData:error:_)]
        pub unsafe fn unarchivedObjectOfClasses_fromData_error(
            classes: &NSSet<AnyClass>,
            data: &NSData,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSData",
            feature = "NSError",
            feature = "NSSet"
        ))]
        /// Decodes the
        /// `NSArray`root object from
        /// `data`which should be an
        /// `NSArray,`containing the given non-collection classes in
        /// `classes`(no nested arrays or arrays of dictionaries, etc) from the given archive, previously encoded by
        /// `NSKeyedArchiver.`
        /// Enables
        /// `requiresSecureCoding`and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid or cannot be decoded, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other unarchivedArrayOfObjectsOfClasses:fromData:error:_)]
        pub unsafe fn unarchivedArrayOfObjectsOfClasses_fromData_error(
            classes: &NSSet<AnyClass>,
            data: &NSData,
        ) -> Result<Retained<NSArray>, Retained<NSError>>;

        #[cfg(all(
            feature = "NSData",
            feature = "NSDictionary",
            feature = "NSError",
            feature = "NSSet"
        ))]
        /// Decodes the
        /// `NSDictionary`root object from
        /// `data`which should be an
        /// `NSDictionary,`with keys of the types given in
        /// `keyClasses`and objects of the given non-collection classes in
        /// `objectClasses`(no nested dictionaries or other dictionaries contained in the dictionary, etc) from the given archive, previously encoded by
        /// `NSKeyedArchiver.`
        /// Enables
        /// `requiresSecureCoding`and sets the
        /// `decodingFailurePolicy`to
        /// `NSDecodingFailurePolicySetErrorAndReturn.`
        /// Returns
        /// `nil`if the given data is not valid or cannot be decoded, and sets the
        /// `error`out parameter.
        #[method_id(@__retain_semantics Other unarchivedDictionaryWithKeysOfClasses:objectsOfClasses:fromData:error:_)]
        pub unsafe fn unarchivedDictionaryWithKeysOfClasses_objectsOfClasses_fromData_error(
            key_classes: &NSSet<AnyClass>,
            value_classes: &NSSet<AnyClass>,
            data: &NSData,
        ) -> Result<Retained<NSDictionary>, Retained<NSError>>;

        #[deprecated = "Use -initForReadingFromData:error: instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use -initForReadingFromData:error: instead"]
        #[method_id(@__retain_semantics Init initForReadingWithData:)]
        pub unsafe fn initForReadingWithData(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use +unarchivedObjectOfClass:fromData:error: instead"]
        #[method_id(@__retain_semantics Other unarchiveObjectWithData:)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSData", feature = "NSError"))]
        #[deprecated = "Use +unarchivedObjectOfClass:fromData:error: instead"]
        #[method_id(@__retain_semantics Other unarchiveTopLevelObjectWithData:error:_)]
        pub unsafe fn unarchiveTopLevelObjectWithData_error(
            data: &NSData,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use +unarchivedObjectOfClass:fromData:error: instead"]
        #[method_id(@__retain_semantics Other unarchiveObjectWithFile:)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSKeyedUnarchiverDelegate>>>;

        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSKeyedUnarchiverDelegate>>,
        );

        #[method(finishDecoding)]
        pub unsafe fn finishDecoding(&self);

        #[cfg(feature = "NSString")]
        #[method(setClass:forClassName:)]
        pub unsafe fn setClass_forClassName_class(cls: Option<&AnyClass>, coded_name: &NSString);

        #[cfg(feature = "NSString")]
        #[method(setClass:forClassName:)]
        pub unsafe fn setClass_forClassName(&self, cls: Option<&AnyClass>, coded_name: &NSString);

        #[cfg(feature = "NSString")]
        #[method(classForClassName:)]
        pub unsafe fn classForClassName_class(coded_name: &NSString) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSString")]
        #[method(classForClassName:)]
        pub unsafe fn classForClassName(&self, coded_name: &NSString) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSString")]
        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[cfg(feature = "NSString")]
        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[cfg(feature = "NSString")]
        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[cfg(feature = "NSString")]
        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[cfg(feature = "NSString")]
        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[cfg(feature = "NSString")]
        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *const u8;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        /// Setter for [`requiresSecureCoding`][Self::requiresSecureCoding].
        #[method(setRequiresSecureCoding:)]
        pub unsafe fn setRequiresSecureCoding(&self, requires_secure_coding: bool);

        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        /// Setter for [`decodingFailurePolicy`][Self::decodingFailurePolicy].
        #[method(setDecodingFailurePolicy:)]
        pub unsafe fn setDecodingFailurePolicy(
            &self,
            decoding_failure_policy: NSDecodingFailurePolicy,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCoder")]
    unsafe impl NSKeyedUnarchiver {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyedarchiverdelegate?language=objc)
    pub unsafe trait NSKeyedArchiverDelegate: NSObjectProtocol {
        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method_id(@__retain_semantics Other archiver:willEncodeObject:)]
        unsafe fn archiver_willEncodeObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: &AnyObject,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(archiver:didEncodeObject:)]
        unsafe fn archiver_didEncodeObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: Option<&AnyObject>,
        );

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(archiver:willReplaceObject:withObject:)]
        unsafe fn archiver_willReplaceObject_withObject(
            &self,
            archiver: &NSKeyedArchiver,
            object: Option<&AnyObject>,
            new_object: Option<&AnyObject>,
        );

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(archiverWillFinish:)]
        unsafe fn archiverWillFinish(&self, archiver: &NSKeyedArchiver);

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(archiverDidFinish:)]
        unsafe fn archiverDidFinish(&self, archiver: &NSKeyedArchiver);
    }

    unsafe impl ProtocolType for dyn NSKeyedArchiverDelegate {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nskeyedunarchiverdelegate?language=objc)
    pub unsafe trait NSKeyedUnarchiverDelegate: NSObjectProtocol {
        #[cfg(all(feature = "NSArray", feature = "NSCoder", feature = "NSString"))]
        #[optional]
        #[method(unarchiver:cannotDecodeObjectOfClassName:originalClasses:)]
        unsafe fn unarchiver_cannotDecodeObjectOfClassName_originalClasses(
            &self,
            unarchiver: &NSKeyedUnarchiver,
            name: &NSString,
            class_names: &NSArray<NSString>,
        ) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(unarchiver:willReplaceObject:withObject:)]
        unsafe fn unarchiver_willReplaceObject_withObject(
            &self,
            unarchiver: &NSKeyedUnarchiver,
            object: &AnyObject,
            new_object: &AnyObject,
        );

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(unarchiverWillFinish:)]
        unsafe fn unarchiverWillFinish(&self, unarchiver: &NSKeyedUnarchiver);

        #[cfg(feature = "NSCoder")]
        #[optional]
        #[method(unarchiverDidFinish:)]
        unsafe fn unarchiverDidFinish(&self, unarchiver: &NSKeyedUnarchiver);
    }

    unsafe impl ProtocolType for dyn NSKeyedUnarchiverDelegate {}
);

extern_category!(
    /// Category "NSKeyedArchiverObjectSubstitution" on [`NSObject`].
    #[doc(alias = "NSKeyedArchiverObjectSubstitution")]
    pub unsafe trait NSObjectNSKeyedArchiverObjectSubstitution {
        #[method(classForKeyedArchiver)]
        unsafe fn classForKeyedArchiver(&self) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Other replacementObjectForKeyedArchiver:)]
        unsafe fn replacementObjectForKeyedArchiver(
            &self,
            archiver: &NSKeyedArchiver,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other classFallbacksForKeyedArchiver)]
        unsafe fn classFallbacksForKeyedArchiver() -> Retained<NSArray<NSString>>;
    }

    unsafe impl NSObjectNSKeyedArchiverObjectSubstitution for NSObject {}
);

extern_category!(
    /// Category "NSKeyedUnarchiverObjectSubstitution" on [`NSObject`].
    #[doc(alias = "NSKeyedUnarchiverObjectSubstitution")]
    pub unsafe trait NSObjectNSKeyedUnarchiverObjectSubstitution {
        #[method(classForKeyedUnarchiver)]
        unsafe fn classForKeyedUnarchiver() -> &'static AnyClass;
    }

    unsafe impl NSObjectNSKeyedUnarchiverObjectSubstitution for NSObject {}
);
