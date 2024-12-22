//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// Structure containing the callbacks for keys of a CFDictionary.
/// Field: version The version number of the structure type being passed
/// in as a parameter to the CFDictionary creation functions.
/// This structure is version 0.
/// Field: retain The callback used to add a retain for the dictionary
/// on keys as they are used to put values into the dictionary.
/// This callback returns the value to use as the key in the
/// dictionary, which is usually the value parameter passed to
/// this callback, but may be a different value if a different
/// value should be used as the key. The dictionary's allocator
/// is passed as the first argument.
/// Field: release The callback used to remove a retain previously added
/// for the dictionary from keys as their values are removed from
/// the dictionary. The dictionary's allocator is passed as the
/// first argument.
/// Field: copyDescription The callback used to create a descriptive
/// string representation of each key in the dictionary. This
/// is used by the CFCopyDescription() function.
/// Field: equal The callback used to compare keys in the dictionary for
/// equality.
/// Field: hash The callback used to compute a hash code for keys as they
/// are used to access, add, or remove values in the dictionary.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryretaincallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFDictionaryRetainCallBack =
    Option<unsafe extern "C-unwind" fn(CFAllocatorRef, *const c_void) -> *const c_void>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryreleasecallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFDictionaryReleaseCallBack =
    Option<unsafe extern "C-unwind" fn(CFAllocatorRef, *const c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionarycopydescriptioncallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFDictionaryCopyDescriptionCallBack =
    Option<unsafe extern "C-unwind" fn(*const c_void) -> CFStringRef>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryequalcallback?language=objc)
pub type CFDictionaryEqualCallBack =
    Option<unsafe extern "C-unwind" fn(*const c_void, *const c_void) -> Boolean>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryhashcallback?language=objc)
#[cfg(feature = "CFBase")]
pub type CFDictionaryHashCallBack =
    Option<unsafe extern "C-unwind" fn(*const c_void) -> CFHashCode>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionarykeycallbacks?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFDictionaryKeyCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
    pub hash: CFDictionaryHashCallBack,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFDictionaryKeyCallBacks {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CFIndex>::ENCODING,
            <CFDictionaryRetainCallBack>::ENCODING,
            <CFDictionaryReleaseCallBack>::ENCODING,
            <CFDictionaryCopyDescriptionCallBack>::ENCODING,
            <CFDictionaryEqualCallBack>::ENCODING,
            <CFDictionaryHashCallBack>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFDictionaryKeyCallBacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// Predefined CFDictionaryKeyCallBacks structure containing a
    /// set of callbacks appropriate for use when the keys of a
    /// CFDictionary are all CFTypes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcftypedictionarykeycallbacks?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
}

extern "C" {
    /// Predefined CFDictionaryKeyCallBacks structure containing a
    /// set of callbacks appropriate for use when the keys of a
    /// CFDictionary are all CFStrings, which may be mutable and
    /// need to be copied in order to serve as constant keys for
    /// the values in the dictionary.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfcopystringdictionarykeycallbacks?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFCopyStringDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
}

/// Structure containing the callbacks for values of a CFDictionary.
/// Field: version The version number of the structure type being passed
/// in as a parameter to the CFDictionary creation functions.
/// This structure is version 0.
/// Field: retain The callback used to add a retain for the dictionary
/// on values as they are put into the dictionary.
/// This callback returns the value to use as the value in the
/// dictionary, which is usually the value parameter passed to
/// this callback, but may be a different value if a different
/// value should be added to the dictionary. The dictionary's
/// allocator is passed as the first argument.
/// Field: release The callback used to remove a retain previously added
/// for the dictionary from values as they are removed from
/// the dictionary. The dictionary's allocator is passed as the
/// first argument.
/// Field: copyDescription The callback used to create a descriptive
/// string representation of each value in the dictionary. This
/// is used by the CFCopyDescription() function.
/// Field: equal The callback used to compare values in the dictionary for
/// equality in some operations.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryvaluecallbacks?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFDictionaryValueCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFDictionaryValueCallBacks {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CFIndex>::ENCODING,
            <CFDictionaryRetainCallBack>::ENCODING,
            <CFDictionaryReleaseCallBack>::ENCODING,
            <CFDictionaryCopyDescriptionCallBack>::ENCODING,
            <CFDictionaryEqualCallBack>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFDictionaryValueCallBacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// Predefined CFDictionaryValueCallBacks structure containing a set
    /// of callbacks appropriate for use when the values in a CFDictionary
    /// are all CFTypes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcftypedictionaryvaluecallbacks?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;
}

/// Type of the callback function used by the apply functions of
/// CFDictionarys.
///
/// Parameter `key`: The current key for the value.
///
/// Parameter `value`: The current value from the dictionary.
///
/// Parameter `context`: The user-defined context parameter given to the apply
/// function.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryapplierfunction?language=objc)
pub type CFDictionaryApplierFunction =
    Option<unsafe extern "C-unwind" fn(*const c_void, *const c_void, *mut c_void)>;

/// This is the type of a reference to immutable CFDictionarys.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfdictionaryref?language=objc)
pub type CFDictionaryRef = *const c_void;

/// This is the type of a reference to mutable CFDictionarys.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmutabledictionaryref?language=objc)
pub type CFMutableDictionaryRef = *mut c_void;

extern "C-unwind" {
    /// Returns the type identifier of all CFDictionary instances.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// Creates a new immutable dictionary with the given values.
    ///
    /// Parameter `allocator`: The CFAllocator which should be used to allocate
    /// memory for the dictionary and its storage for values. This
    /// parameter may be NULL in which case the current default
    /// CFAllocator is used. If this reference is not a valid
    /// CFAllocator, the behavior is undefined.
    ///
    /// Parameter `keys`: A C array of the pointer-sized keys to be used for
    /// the parallel C array of values to be put into the dictionary.
    /// This parameter may be NULL if the numValues parameter is 0.
    /// This C array is not changed or freed by this function. If
    /// this parameter is not a valid pointer to a C array of at
    /// least numValues pointers, the behavior is undefined.
    ///
    /// Parameter `values`: A C array of the pointer-sized values to be in the
    /// dictionary. This parameter may be NULL if the numValues
    /// parameter is 0. This C array is not changed or freed by
    /// this function. If this parameter is not a valid pointer to
    /// a C array of at least numValues pointers, the behavior is
    /// undefined.
    ///
    /// Parameter `numValues`: The number of values to copy from the keys and
    /// values C arrays into the CFDictionary. This number will be
    /// the count of the dictionary. If this parameter is
    /// negative, or greater than the number of values actually
    /// in the keys or values C arrays, the behavior is undefined.
    ///
    /// Parameter `keyCallBacks`: A pointer to a CFDictionaryKeyCallBacks structure
    /// initialized with the callbacks for the dictionary to use on
    /// each key in the dictionary. The retain callback will be used
    /// within this function, for example, to retain all of the new
    /// keys from the keys C array. A copy of the contents of the
    /// callbacks structure is made, so that a pointer to a structure
    /// on the stack can be passed in, or can be reused for multiple
    /// dictionary creations. If the version field of this
    /// callbacks structure is not one of the defined ones for
    /// CFDictionary, the behavior is undefined. The retain field may
    /// be NULL, in which case the CFDictionary will do nothing to add
    /// a retain to the keys of the contained values. The release field
    /// may be NULL, in which case the CFDictionary will do nothing
    /// to remove the dictionary's retain (if any) on the keys when the
    /// dictionary is destroyed or a key-value pair is removed. If the
    /// copyDescription field is NULL, the dictionary will create a
    /// simple description for a key. If the equal field is NULL, the
    /// dictionary will use pointer equality to test for equality of
    /// keys. If the hash field is NULL, a key will be converted from
    /// a pointer to an integer to compute the hash code. This callbacks
    /// parameter itself may be NULL, which is treated as if a valid
    /// structure of version 0 with all fields NULL had been passed in.
    /// Otherwise, if any of the fields are not valid pointers to
    /// functions of the correct type, or this parameter is not a
    /// valid pointer to a CFDictionaryKeyCallBacks callbacks structure,
    /// the behavior is undefined. If any of the keys put into the
    /// dictionary is not one understood by one of the callback functions
    /// the behavior when that callback function is used is undefined.
    ///
    /// Parameter `valueCallBacks`: A pointer to a CFDictionaryValueCallBacks structure
    /// initialized with the callbacks for the dictionary to use on
    /// each value in the dictionary. The retain callback will be used
    /// within this function, for example, to retain all of the new
    /// values from the values C array. A copy of the contents of the
    /// callbacks structure is made, so that a pointer to a structure
    /// on the stack can be passed in, or can be reused for multiple
    /// dictionary creations. If the version field of this callbacks
    /// structure is not one of the defined ones for CFDictionary, the
    /// behavior is undefined. The retain field may be NULL, in which
    /// case the CFDictionary will do nothing to add a retain to values
    /// as they are put into the dictionary. The release field may be
    /// NULL, in which case the CFDictionary will do nothing to remove
    /// the dictionary's retain (if any) on the values when the
    /// dictionary is destroyed or a key-value pair is removed. If the
    /// copyDescription field is NULL, the dictionary will create a
    /// simple description for a value. If the equal field is NULL, the
    /// dictionary will use pointer equality to test for equality of
    /// values. This callbacks parameter itself may be NULL, which is
    /// treated as if a valid structure of version 0 with all fields
    /// NULL had been passed in. Otherwise,
    /// if any of the fields are not valid pointers to functions
    /// of the correct type, or this parameter is not a valid
    /// pointer to a CFDictionaryValueCallBacks callbacks structure,
    /// the behavior is undefined. If any of the values put into the
    /// dictionary is not one understood by one of the callback functions
    /// the behavior when that callback function is used is undefined.
    ///
    /// Returns: A reference to the new immutable CFDictionary.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryCreate(
        allocator: CFAllocatorRef,
        keys: *mut *const c_void,
        values: *mut *const c_void,
        num_values: CFIndex,
        key_call_backs: *const CFDictionaryKeyCallBacks,
        value_call_backs: *const CFDictionaryValueCallBacks,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    /// Creates a new immutable dictionary with the key-value pairs from
    /// the given dictionary.
    ///
    /// Parameter `allocator`: The CFAllocator which should be used to allocate
    /// memory for the dictionary and its storage for values. This
    /// parameter may be NULL in which case the current default
    /// CFAllocator is used. If this reference is not a valid
    /// CFAllocator, the behavior is undefined.
    ///
    /// Parameter `theDict`: The dictionary which is to be copied. The keys and values
    /// from the dictionary are copied as pointers into the new
    /// dictionary (that is, the values themselves are copied, not
    /// that which the values point to, if anything). However, the
    /// keys and values are also retained by the new dictionary using
    /// the retain function of the original dictionary.
    /// The count of the new dictionary will be the same as the
    /// given dictionary. The new dictionary uses the same callbacks
    /// as the dictionary to be copied. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Returns: A reference to the new immutable CFDictionary.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryCreateCopy(
        allocator: CFAllocatorRef,
        the_dict: CFDictionaryRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    /// Creates a new mutable dictionary.
    ///
    /// Parameter `allocator`: The CFAllocator which should be used to allocate
    /// memory for the dictionary and its storage for values. This
    /// parameter may be NULL in which case the current default
    /// CFAllocator is used. If this reference is not a valid
    /// CFAllocator, the behavior is undefined.
    ///
    /// Parameter `capacity`: A hint about the number of values that will be held
    /// by the CFDictionary. Pass 0 for no hint. The implementation may
    /// ignore this hint, or may use it to optimize various
    /// operations. A dictionary's actual capacity is only limited by
    /// address space and available memory constraints). If this
    /// parameter is negative, the behavior is undefined.
    ///
    /// Parameter `keyCallBacks`: A pointer to a CFDictionaryKeyCallBacks structure
    /// initialized with the callbacks for the dictionary to use on
    /// each key in the dictionary. A copy of the contents of the
    /// callbacks structure is made, so that a pointer to a structure
    /// on the stack can be passed in, or can be reused for multiple
    /// dictionary creations. If the version field of this
    /// callbacks structure is not one of the defined ones for
    /// CFDictionary, the behavior is undefined. The retain field may
    /// be NULL, in which case the CFDictionary will do nothing to add
    /// a retain to the keys of the contained values. The release field
    /// may be NULL, in which case the CFDictionary will do nothing
    /// to remove the dictionary's retain (if any) on the keys when the
    /// dictionary is destroyed or a key-value pair is removed. If the
    /// copyDescription field is NULL, the dictionary will create a
    /// simple description for a key. If the equal field is NULL, the
    /// dictionary will use pointer equality to test for equality of
    /// keys. If the hash field is NULL, a key will be converted from
    /// a pointer to an integer to compute the hash code. This callbacks
    /// parameter itself may be NULL, which is treated as if a valid
    /// structure of version 0 with all fields NULL had been passed in.
    /// Otherwise, if any of the fields are not valid pointers to
    /// functions of the correct type, or this parameter is not a
    /// valid pointer to a CFDictionaryKeyCallBacks callbacks structure,
    /// the behavior is undefined. If any of the keys put into the
    /// dictionary is not one understood by one of the callback functions
    /// the behavior when that callback function is used is undefined.
    ///
    /// Parameter `valueCallBacks`: A pointer to a CFDictionaryValueCallBacks structure
    /// initialized with the callbacks for the dictionary to use on
    /// each value in the dictionary. The retain callback will be used
    /// within this function, for example, to retain all of the new
    /// values from the values C array. A copy of the contents of the
    /// callbacks structure is made, so that a pointer to a structure
    /// on the stack can be passed in, or can be reused for multiple
    /// dictionary creations. If the version field of this callbacks
    /// structure is not one of the defined ones for CFDictionary, the
    /// behavior is undefined. The retain field may be NULL, in which
    /// case the CFDictionary will do nothing to add a retain to values
    /// as they are put into the dictionary. The release field may be
    /// NULL, in which case the CFDictionary will do nothing to remove
    /// the dictionary's retain (if any) on the values when the
    /// dictionary is destroyed or a key-value pair is removed. If the
    /// copyDescription field is NULL, the dictionary will create a
    /// simple description for a value. If the equal field is NULL, the
    /// dictionary will use pointer equality to test for equality of
    /// values. This callbacks parameter itself may be NULL, which is
    /// treated as if a valid structure of version 0 with all fields
    /// NULL had been passed in. Otherwise,
    /// if any of the fields are not valid pointers to functions
    /// of the correct type, or this parameter is not a valid
    /// pointer to a CFDictionaryValueCallBacks callbacks structure,
    /// the behavior is undefined. If any of the values put into the
    /// dictionary is not one understood by one of the callback functions
    /// the behavior when that callback function is used is undefined.
    ///
    /// Returns: A reference to the new mutable CFDictionary.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        key_call_backs: *const CFDictionaryKeyCallBacks,
        value_call_backs: *const CFDictionaryValueCallBacks,
    ) -> CFMutableDictionaryRef;
}

extern "C-unwind" {
    /// Creates a new mutable dictionary with the key-value pairs from
    /// the given dictionary.
    ///
    /// Parameter `allocator`: The CFAllocator which should be used to allocate
    /// memory for the dictionary and its storage for values. This
    /// parameter may be NULL in which case the current default
    /// CFAllocator is used. If this reference is not a valid
    /// CFAllocator, the behavior is undefined.
    ///
    /// Parameter `capacity`: A hint about the number of values that will be held
    /// by the CFDictionary. Pass 0 for no hint. The implementation may
    /// ignore this hint, or may use it to optimize various
    /// operations. A dictionary's actual capacity is only limited by
    /// address space and available memory constraints).
    /// This parameter must be greater than or equal
    /// to the count of the dictionary which is to be copied, or the
    /// behavior is undefined. If this parameter is negative, the
    /// behavior is undefined.
    ///
    /// Parameter `theDict`: The dictionary which is to be copied. The keys and values
    /// from the dictionary are copied as pointers into the new
    /// dictionary (that is, the values themselves are copied, not
    /// that which the values point to, if anything). However, the
    /// keys and values are also retained by the new dictionary using
    /// the retain function of the original dictionary.
    /// The count of the new dictionary will be the same as the
    /// given dictionary. The new dictionary uses the same callbacks
    /// as the dictionary to be copied. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Returns: A reference to the new mutable CFDictionary.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        the_dict: CFDictionaryRef,
    ) -> CFMutableDictionaryRef;
}

extern "C-unwind" {
    /// Returns the number of values currently in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be queried. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Returns: The number of values in the dictionary.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryGetCount(the_dict: CFDictionaryRef) -> CFIndex;
}

extern "C-unwind" {
    /// Counts the number of times the given key occurs in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be searched. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `key`: The key for which to find matches in the dictionary. The
    /// hash() and equal() key callbacks provided when the dictionary
    /// was created are used to compare. If the hash() key callback
    /// was NULL, the key is treated as a pointer and converted to
    /// an integer. If the equal() key callback was NULL, pointer
    /// equality (in C, ==) is used. If key, or any of the keys in
    /// the dictionary, are not understood by the equal() callback,
    /// the behavior is undefined.
    ///
    /// Returns: Returns 1 if a matching key is used by the dictionary,
    /// 0 otherwise.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryGetCountOfKey(the_dict: CFDictionaryRef, key: *const c_void) -> CFIndex;
}

extern "C-unwind" {
    /// Counts the number of times the given value occurs in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be searched. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `value`: The value for which to find matches in the dictionary. The
    /// equal() callback provided when the dictionary was created is
    /// used to compare. If the equal() value callback was NULL, pointer
    /// equality (in C, ==) is used. If value, or any of the values in
    /// the dictionary, are not understood by the equal() callback,
    /// the behavior is undefined.
    ///
    /// Returns: The number of times the given value occurs in the dictionary.
    #[cfg(feature = "CFBase")]
    pub fn CFDictionaryGetCountOfValue(the_dict: CFDictionaryRef, value: *const c_void) -> CFIndex;
}

extern "C-unwind" {
    /// Reports whether or not the key is in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be searched. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `key`: The key for which to find matches in the dictionary. The
    /// hash() and equal() key callbacks provided when the dictionary
    /// was created are used to compare. If the hash() key callback
    /// was NULL, the key is treated as a pointer and converted to
    /// an integer. If the equal() key callback was NULL, pointer
    /// equality (in C, ==) is used. If key, or any of the keys in
    /// the dictionary, are not understood by the equal() callback,
    /// the behavior is undefined.
    ///
    /// Returns: true, if the key is in the dictionary, otherwise false.
    pub fn CFDictionaryContainsKey(the_dict: CFDictionaryRef, key: *const c_void) -> Boolean;
}

extern "C-unwind" {
    /// Reports whether or not the value is in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be searched. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `value`: The value for which to find matches in the dictionary. The
    /// equal() callback provided when the dictionary was created is
    /// used to compare. If the equal() callback was NULL, pointer
    /// equality (in C, ==) is used. If value, or any of the values
    /// in the dictionary, are not understood by the equal() callback,
    /// the behavior is undefined.
    ///
    /// Returns: true, if the value is in the dictionary, otherwise false.
    pub fn CFDictionaryContainsValue(the_dict: CFDictionaryRef, value: *const c_void) -> Boolean;
}

extern "C-unwind" {
    /// Retrieves the value associated with the given key.
    ///
    /// Parameter `theDict`: The dictionary to be queried. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `key`: The key for which to find a match in the dictionary. The
    /// hash() and equal() key callbacks provided when the dictionary
    /// was created are used to compare. If the hash() key callback
    /// was NULL, the key is treated as a pointer and converted to
    /// an integer. If the equal() key callback was NULL, pointer
    /// equality (in C, ==) is used. If key, or any of the keys in
    /// the dictionary, are not understood by the equal() callback,
    /// the behavior is undefined.
    ///
    /// Returns: The value with the given key in the dictionary, or NULL if
    /// no key-value pair with a matching key exists. Since NULL
    /// can be a valid value in some dictionaries, the function
    /// CFDictionaryGetValueIfPresent() must be used to distinguish
    /// NULL-no-found from NULL-is-the-value.
    pub fn CFDictionaryGetValue(the_dict: CFDictionaryRef, key: *const c_void) -> *const c_void;
}

extern "C-unwind" {
    /// Retrieves the value associated with the given key.
    ///
    /// Parameter `theDict`: The dictionary to be queried. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `key`: The key for which to find a match in the dictionary. The
    /// hash() and equal() key callbacks provided when the dictionary
    /// was created are used to compare. If the hash() key callback
    /// was NULL, the key is treated as a pointer and converted to
    /// an integer. If the equal() key callback was NULL, pointer
    /// equality (in C, ==) is used. If key, or any of the keys in
    /// the dictionary, are not understood by the equal() callback,
    /// the behavior is undefined.
    ///
    /// Parameter `value`: A pointer to memory which should be filled with the
    /// pointer-sized value if a matching key is found. If no key
    /// match is found, the contents of the storage pointed to by
    /// this parameter are undefined. This parameter may be NULL,
    /// in which case the value from the dictionary is not returned
    /// (but the return value of this function still indicates
    /// whether or not the key-value pair was present).
    ///
    /// Returns: true, if a matching key was found, false otherwise.
    pub fn CFDictionaryGetValueIfPresent(
        the_dict: CFDictionaryRef,
        key: *const c_void,
        value: *mut *const c_void,
    ) -> Boolean;
}

extern "C-unwind" {
    /// Fills the two buffers with the keys and values from the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be queried. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `keys`: A C array of pointer-sized values to be filled with keys
    /// from the dictionary. The keys and values C arrays are parallel
    /// to each other (that is, the items at the same indices form a
    /// key-value pair from the dictionary). This parameter may be NULL
    /// if the keys are not desired. If this parameter is not a valid
    /// pointer to a C array of at least CFDictionaryGetCount() pointers,
    /// or NULL, the behavior is undefined.
    ///
    /// Parameter `values`: A C array of pointer-sized values to be filled with values
    /// from the dictionary. The keys and values C arrays are parallel
    /// to each other (that is, the items at the same indices form a
    /// key-value pair from the dictionary). This parameter may be NULL
    /// if the values are not desired. If this parameter is not a valid
    /// pointer to a C array of at least CFDictionaryGetCount() pointers,
    /// or NULL, the behavior is undefined.
    pub fn CFDictionaryGetKeysAndValues(
        the_dict: CFDictionaryRef,
        keys: *mut *const c_void,
        values: *mut *const c_void,
    );
}

extern "C-unwind" {
    /// Calls a function once for each value in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to be queried. If this parameter is
    /// not a valid CFDictionary, the behavior is undefined.
    ///
    /// Parameter `applier`: The callback function to call once for each value in
    /// the dictionary. If this parameter is not a
    /// pointer to a function of the correct prototype, the behavior
    /// is undefined. If there are keys or values which the
    /// applier function does not expect or cannot properly apply
    /// to, the behavior is undefined.
    ///
    /// Parameter `context`: A pointer-sized user-defined value, which is passed
    /// as the third parameter to the applier function, but is
    /// otherwise unused by this function. If the context is not
    /// what is expected by the applier function, the behavior is
    /// undefined.
    pub fn CFDictionaryApplyFunction(
        the_dict: CFDictionaryRef,
        applier: CFDictionaryApplierFunction,
        context: *mut c_void,
    );
}

extern "C-unwind" {
    /// Adds the key-value pair to the dictionary if no such key already exists.
    ///
    /// Parameter `theDict`: The dictionary to which the value is to be added. If this
    /// parameter is not a valid mutable CFDictionary, the behavior is
    /// undefined.
    ///
    /// Parameter `key`: The key of the value to add to the dictionary. The key is
    /// retained by the dictionary using the retain callback provided
    /// when the dictionary was created. If the key is not of the sort
    /// expected by the retain callback, the behavior is undefined. If
    /// a key which matches this key is already present in the dictionary,
    /// this function does nothing ("add if absent").
    ///
    /// Parameter `value`: The value to add to the dictionary. The value is retained
    /// by the dictionary using the retain callback provided when the
    /// dictionary was created. If the value is not of the sort expected
    /// by the retain callback, the behavior is undefined.
    pub fn CFDictionaryAddValue(
        the_dict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
}

extern "C-unwind" {
    /// Sets the value of the key in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to which the value is to be set. If this
    /// parameter is not a valid mutable CFDictionary, the behavior is
    /// undefined.
    ///
    /// Parameter `key`: The key of the value to set into the dictionary. If a key
    /// which matches this key is already present in the dictionary, only
    /// the value is changed ("add if absent, replace if present"). If
    /// no key matches the given key, the key-value pair is added to the
    /// dictionary. If added, the key is retained by the dictionary,
    /// using the retain callback provided
    /// when the dictionary was created. If the key is not of the sort
    /// expected by the key retain callback, the behavior is undefined.
    ///
    /// Parameter `value`: The value to add to or replace into the dictionary. The value
    /// is retained by the dictionary using the retain callback provided
    /// when the dictionary was created, and the previous value if any is
    /// released. If the value is not of the sort expected by the
    /// retain or release callbacks, the behavior is undefined.
    pub fn CFDictionarySetValue(
        the_dict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
}

extern "C-unwind" {
    /// Replaces the value of the key in the dictionary.
    ///
    /// Parameter `theDict`: The dictionary to which the value is to be replaced. If this
    /// parameter is not a valid mutable CFDictionary, the behavior is
    /// undefined.
    ///
    /// Parameter `key`: The key of the value to replace in the dictionary. If a key
    /// which matches this key is present in the dictionary, the value
    /// is changed to the given value, otherwise this function does
    /// nothing ("replace if present").
    ///
    /// Parameter `value`: The value to replace into the dictionary. The value
    /// is retained by the dictionary using the retain callback provided
    /// when the dictionary was created, and the previous value is
    /// released. If the value is not of the sort expected by the
    /// retain or release callbacks, the behavior is undefined.
    pub fn CFDictionaryReplaceValue(
        the_dict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
}

extern "C-unwind" {
    /// Removes the value of the key from the dictionary.
    ///
    /// Parameter `theDict`: The dictionary from which the value is to be removed. If this
    /// parameter is not a valid mutable CFDictionary, the behavior is
    /// undefined.
    ///
    /// Parameter `key`: The key of the value to remove from the dictionary. If a key
    /// which matches this key is present in the dictionary, the key-value
    /// pair is removed from the dictionary, otherwise this function does
    /// nothing ("remove if present").
    pub fn CFDictionaryRemoveValue(the_dict: CFMutableDictionaryRef, key: *const c_void);
}

extern "C-unwind" {
    /// Removes all the values from the dictionary, making it empty.
    ///
    /// Parameter `theDict`: The dictionary from which all of the values are to be
    /// removed. If this parameter is not a valid mutable
    /// CFDictionary, the behavior is undefined.
    pub fn CFDictionaryRemoveAllValues(the_dict: CFMutableDictionaryRef);
}
