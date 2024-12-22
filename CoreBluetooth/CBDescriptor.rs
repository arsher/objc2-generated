//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Represents a characteristic's descriptor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbdescriptor?language=objc)
    #[unsafe(super(CBAttribute, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CBAttribute")]
    pub struct CBDescriptor;
);

#[cfg(feature = "CBAttribute")]
unsafe impl NSObjectProtocol for CBDescriptor {}

extern_methods!(
    #[cfg(feature = "CBAttribute")]
    unsafe impl CBDescriptor {
        #[cfg(feature = "CBCharacteristic")]
        /// A back-pointer to the characteristic this descriptor belongs to.
        #[method_id(@__retain_semantics Other characteristic)]
        pub unsafe fn characteristic(&self) -> Option<Retained<CBCharacteristic>>;

        /// The value of the descriptor. The corresponding value types for the various descriptors are detailed in
        ///
        /// ```text
        ///  CBUUID.h
        /// ```
        ///
        /// .
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CBAttribute`
    #[cfg(feature = "CBAttribute")]
    unsafe impl CBDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CBAttribute")]
    unsafe impl CBDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Used to create a local characteristic descriptor, which can be added to the local database via
    /// <code>
    /// CBPeripheralManager
    /// </code>
    /// .
    /// Once a descriptor is published, it is cached and can no longer be changed.
    /// Descriptor types are detailed in
    ///
    /// ```text
    ///  CBUUID.h
    /// ```
    ///
    /// , but only the
    /// <code>
    /// Characteristic User Description
    /// </code>
    /// and
    /// <code>
    /// Characteristic Presentation
    /// Format
    /// </code>
    /// descriptors are currently supported. The
    /// <code>
    /// Characteristic Extended Properties
    /// </code>
    /// and
    /// <code>
    /// Client Characteristic
    /// Configuration
    /// </code>
    /// descriptors will be created automatically upon publication of the parent service, depending on the properties of the characteristic itself.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbmutabledescriptor?language=objc)
    #[unsafe(super(CBDescriptor, CBAttribute, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CBAttribute")]
    pub struct CBMutableDescriptor;
);

#[cfg(feature = "CBAttribute")]
unsafe impl NSObjectProtocol for CBMutableDescriptor {}

extern_methods!(
    #[cfg(feature = "CBAttribute")]
    unsafe impl CBMutableDescriptor {
        #[cfg(feature = "CBUUID")]
        /// Parameter `UUID`: The Bluetooth UUID of the descriptor.
        ///
        /// Parameter `value`: The value of the descriptor.
        ///
        ///
        /// Returns a decriptor, initialized with a service type and value. The
        /// <i>
        /// value
        /// </i>
        /// is required and cannot be updated dynamically
        /// once the parent service has been published.
        #[method_id(@__retain_semantics Init initWithType:value:)]
        pub unsafe fn initWithType_value(
            this: Allocated<Self>,
            uuid: &CBUUID,
            value: Option<&AnyObject>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CBAttribute`
    #[cfg(feature = "CBAttribute")]
    unsafe impl CBMutableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CBAttribute")]
    unsafe impl CBMutableDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
