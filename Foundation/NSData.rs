//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataReadingOptions(pub NSUInteger);
impl NSDataReadingOptions {
    pub const NSDataReadingMappedIfSafe: Self = Self(1 << 0);
    pub const NSDataReadingUncached: Self = Self(1 << 1);
    pub const NSDataReadingMappedAlways: Self = Self(1 << 3);
    #[deprecated]
    pub const NSDataReadingMapped: Self = Self(NSDataReadingOptions::NSDataReadingMappedIfSafe.0);
    #[deprecated]
    pub const NSMappedRead: Self = Self(NSDataReadingOptions::NSDataReadingMapped.0);
    #[deprecated]
    pub const NSUncachedRead: Self = Self(NSDataReadingOptions::NSDataReadingUncached.0);
}

unsafe impl Encode for NSDataReadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataReadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataWritingOptions(pub NSUInteger);
impl NSDataWritingOptions {
    pub const NSDataWritingAtomic: Self = Self(1 << 0);
    pub const NSDataWritingWithoutOverwriting: Self = Self(1 << 1);
    pub const NSDataWritingFileProtectionNone: Self = Self(0x10000000);
    pub const NSDataWritingFileProtectionComplete: Self = Self(0x20000000);
    pub const NSDataWritingFileProtectionCompleteUnlessOpen: Self = Self(0x30000000);
    pub const NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication: Self =
        Self(0x40000000);
    pub const NSDataWritingFileProtectionCompleteWhenUserInactive: Self = Self(0x50000000);
    pub const NSDataWritingFileProtectionMask: Self = Self(0xf0000000);
    #[deprecated]
    pub const NSAtomicWrite: Self = Self(NSDataWritingOptions::NSDataWritingAtomic.0);
}

unsafe impl Encode for NSDataWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataSearchOptions(pub NSUInteger);
impl NSDataSearchOptions {
    pub const NSDataSearchBackwards: Self = Self(1 << 0);
    pub const NSDataSearchAnchored: Self = Self(1 << 1);
}

unsafe impl Encode for NSDataSearchOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataSearchOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataBase64EncodingOptions(pub NSUInteger);
impl NSDataBase64EncodingOptions {
    pub const NSDataBase64Encoding64CharacterLineLength: Self = Self(1 << 0);
    pub const NSDataBase64Encoding76CharacterLineLength: Self = Self(1 << 1);
    pub const NSDataBase64EncodingEndLineWithCarriageReturn: Self = Self(1 << 4);
    pub const NSDataBase64EncodingEndLineWithLineFeed: Self = Self(1 << 5);
}

unsafe impl Encode for NSDataBase64EncodingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataBase64EncodingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataBase64DecodingOptions(pub NSUInteger);
impl NSDataBase64DecodingOptions {
    pub const NSDataBase64DecodingIgnoreUnknownCharacters: Self = Self(1 << 0);
}

unsafe impl Encode for NSDataBase64DecodingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDataBase64DecodingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSData;

    unsafe impl ClassType for NSData {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableData>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSData {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSData {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSData {}

unsafe impl NSObjectProtocol for NSData {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSData {}

extern_methods!(
    unsafe impl NSData {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSData {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for NSData {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedData
    unsafe impl NSData {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString>;

        #[method(getBytes:length:)]
        pub unsafe fn getBytes_length(&self, buffer: NonNull<c_void>, length: NSUInteger);

        #[cfg(feature = "NSRange")]
        #[method(getBytes:range:)]
        pub unsafe fn getBytes_range(&self, buffer: NonNull<c_void>, range: NSRange);

        #[method(isEqualToData:)]
        pub unsafe fn isEqualToData(&self, other: &NSData) -> bool;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other subdataWithRange:)]
        pub unsafe fn subdataWithRange(&self, range: NSRange) -> Id<NSData>;

        #[cfg(feature = "NSString")]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            use_auxiliary_file: bool,
        ) -> bool;

        #[cfg(feature = "NSURL")]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method(writeToFile:options:error:_)]
        pub unsafe fn writeToFile_options_error(
            &self,
            path: &NSString,
            write_options_mask: NSDataWritingOptions,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method(writeToURL:options:error:_)]
        pub unsafe fn writeToURL_options_error(
            &self,
            url: &NSURL,
            write_options_mask: NSDataWritingOptions,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "NSRange")]
        #[method(rangeOfData:options:range:)]
        pub unsafe fn rangeOfData_options_range(
            &self,
            data_to_find: &NSData,
            mask: NSDataSearchOptions,
            search_range: NSRange,
        ) -> NSRange;

        #[cfg(all(feature = "NSRange", feature = "block2"))]
        #[method(enumerateByteRangesUsingBlock:)]
        pub unsafe fn enumerateByteRangesUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<c_void>, NSRange, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// NSDataCreation
    unsafe impl NSData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(bytes: *mut c_void, length: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Allocated<Self>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&block2::Block<dyn Fn(NonNull<c_void>, NSUInteger)>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Allocated<Self>,
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL)
            -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub fn initWithData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataCreation
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(bytes: *mut c_void, length: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Allocated<Self>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&block2::Block<dyn Fn(NonNull<c_void>, NSUInteger)>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSError", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Allocated<Self>,
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL)
            -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Id<Self>;
    }
);

extern_methods!(
    /// NSDataBase64Encoding
    unsafe impl NSData {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Allocated<Self>,
            base64_string: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other base64EncodedStringWithOptions:)]
        pub unsafe fn base64EncodedStringWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Id<NSString>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Allocated<Self>,
            base64_data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other base64EncodedDataWithOptions:)]
        pub unsafe fn base64EncodedDataWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataBase64Encoding
    unsafe impl NSMutableData {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Allocated<Self>,
            base64_string: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Allocated<Self>,
            base64_data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDataCompressionAlgorithm(pub NSInteger);
impl NSDataCompressionAlgorithm {
    #[doc(alias = "NSDataCompressionAlgorithmLZFSE")]
    pub const LZFSE: Self = Self(0);
    #[doc(alias = "NSDataCompressionAlgorithmLZ4")]
    pub const LZ4: Self = Self(1);
    #[doc(alias = "NSDataCompressionAlgorithmLZMA")]
    pub const LZMA: Self = Self(2);
    #[doc(alias = "NSDataCompressionAlgorithmZlib")]
    pub const Zlib: Self = Self(3);
}

unsafe impl Encode for NSDataCompressionAlgorithm {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDataCompressionAlgorithm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSDataCompression
    unsafe impl NSData {
        #[cfg(feature = "NSError")]
        #[method_id(@__retain_semantics Other decompressedDataUsingAlgorithm:error:_)]
        pub unsafe fn decompressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "NSError")]
        #[method_id(@__retain_semantics Other compressedDataUsingAlgorithm:error:_)]
        pub unsafe fn compressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSData {
        #[deprecated = "This method is unsafe because it could potentially cause buffer overruns. Use -getBytes:length: instead."]
        #[method(getBytes:)]
        pub unsafe fn getBytes(&self, buffer: NonNull<c_void>);

        #[cfg(feature = "NSString")]
        #[deprecated = "Use +dataWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Other dataWithContentsOfMappedFile:)]
        pub unsafe fn dataWithContentsOfMappedFile(path: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -initWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use initWithBase64EncodedString:options: instead"]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Allocated<Self>,
            base64_string: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use base64EncodedStringWithOptions: instead"]
        #[method_id(@__retain_semantics Other base64Encoding)]
        pub unsafe fn base64Encoding(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDeprecated
    unsafe impl NSMutableData {
        #[cfg(feature = "NSString")]
        #[deprecated = "Use -initWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use initWithBase64EncodedString:options: instead"]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Allocated<Self>,
            base64_string: &NSString,
        ) -> Option<Id<Self>>;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableData;

    unsafe impl ClassType for NSMutableData {
        #[inherits(NSObject)]
        type Super = NSData;
        type Mutability = MutableWithImmutableSuperclass<NSData>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSMutableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSMutableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSMutableData {}

unsafe impl NSObjectProtocol for NSMutableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSMutableData {}

extern_methods!(
    unsafe impl NSMutableData {
        #[method(setLength:)]
        pub fn setLength(&mut self, length: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for NSMutableData {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableData
    unsafe impl NSMutableData {
        #[method(appendBytes:length:)]
        pub unsafe fn appendBytes_length(&mut self, bytes: NonNull<c_void>, length: NSUInteger);

        #[method(appendData:)]
        pub unsafe fn appendData(&mut self, other: &NSData);

        #[method(increaseLengthBy:)]
        pub unsafe fn increaseLengthBy(&mut self, extra_length: NSUInteger);

        #[cfg(feature = "NSRange")]
        #[method(replaceBytesInRange:withBytes:)]
        pub unsafe fn replaceBytesInRange_withBytes(
            &mut self,
            range: NSRange,
            bytes: NonNull<c_void>,
        );

        #[cfg(feature = "NSRange")]
        #[method(resetBytesInRange:)]
        pub unsafe fn resetBytesInRange(&mut self, range: NSRange);

        #[method(setData:)]
        pub unsafe fn setData(&mut self, data: &NSData);

        #[cfg(feature = "NSRange")]
        #[method(replaceBytesInRange:withBytes:length:)]
        pub unsafe fn replaceBytesInRange_withBytes_length(
            &mut self,
            range: NSRange,
            replacement_bytes: *mut c_void,
            replacement_length: NSUInteger,
        );
    }
);

extern_methods!(
    /// NSMutableDataCreation
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other dataWithCapacity:)]
        pub fn dataWithCapacity(a_num_items: NSUInteger) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other dataWithLength:)]
        pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub fn initWithCapacity(this: Allocated<Self>, capacity: NSUInteger) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithLength:)]
        pub unsafe fn initWithLength(this: Allocated<Self>, length: NSUInteger)
            -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// NSMutableDataCompression
    unsafe impl NSMutableData {
        #[cfg(feature = "NSError")]
        #[method(decompressUsingAlgorithm:error:_)]
        pub unsafe fn decompressUsingAlgorithm_error(
            &mut self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "NSError")]
        #[method(compressUsingAlgorithm:error:_)]
        pub unsafe fn compressUsingAlgorithm_error(
            &mut self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPurgeableData;

    unsafe impl ClassType for NSPurgeableData {
        #[inherits(NSData, NSObject)]
        type Super = NSMutableData;
        type Mutability = Mutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSPurgeableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSDiscardableContent for NSPurgeableData {}

unsafe impl NSObjectProtocol for NSPurgeableData {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSPurgeableData {}

extern_methods!(
    unsafe impl NSPurgeableData {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPurgeableData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
