//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An identifier to make a virtual machine unique.
    ///
    /// The generic machine identifier is used by guests to uniquely identify the virtual hardware.
    ///
    /// If the virtual machine is serialized to disk, the identifier can be preserved in a binary representation through VZGenericMachineIdentifier.dataRepresentation.
    /// The identifier can then be recreated with -[VZGenericMachineIdentifier initWithDataRepresentation:] from the binary representation.
    ///
    /// The contents of two identifiers can be compared with -[VZGenericMachineIdentifier isEqual:].
    ///
    /// See also: VZGenericPlatformConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzgenericmachineidentifier?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZGenericMachineIdentifier;
);

unsafe impl NSCopying for VZGenericMachineIdentifier {}

unsafe impl CopyingHelper for VZGenericMachineIdentifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZGenericMachineIdentifier {}

extern_methods!(
    unsafe impl VZGenericMachineIdentifier {
        /// Create a new unique machine identifier.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Get the machine identifier described by the specified data representation.
        ///
        /// Parameter `dataRepresentation`: The opaque data representation of the machine identifier to be obtained.
        ///
        /// Returns: A unique identifier identical to the one that generated the dataRepresentation, or nil if the data is invalid.
        ///
        /// See: VZGenericMachineIdentifier.dataRepresentation
        #[method_id(@__retain_semantics Init initWithDataRepresentation:)]
        pub unsafe fn initWithDataRepresentation(
            this: Allocated<Self>,
            data_representation: &NSData,
        ) -> Option<Retained<Self>>;

        /// Opaque data representation of the machine identifier.
        ///
        /// This can be used to recreate the same machine identifier with -[VZGenericMachineIdentifier initWithDataRepresentation:].
        ///
        /// See: -[VZGenericMachineIdentifier initWithDataRepresentation:]
        #[method_id(@__retain_semantics Other dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VZGenericMachineIdentifier {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
