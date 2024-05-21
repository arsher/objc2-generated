//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    pub struct VZLinuxRosettaUnixSocketCachingOptions;

    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    unsafe impl ClassType for VZLinuxRosettaUnixSocketCachingOptions {
        #[inherits(NSObject)]
        type Super = VZLinuxRosettaCachingOptions;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZLinuxRosettaCachingOptions")]
unsafe impl NSObjectProtocol for VZLinuxRosettaUnixSocketCachingOptions {}

extern_methods!(
    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    unsafe impl VZLinuxRosettaUnixSocketCachingOptions {
        #[method_id(@__retain_semantics Init initWithPath:error:_)]
        pub unsafe fn initWithPath_error(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Retained<NSString>;

        #[method(maximumPathLength)]
        pub unsafe fn maximumPathLength() -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZLinuxRosettaCachingOptions`
    #[cfg(feature = "VZLinuxRosettaCachingOptions")]
    unsafe impl VZLinuxRosettaUnixSocketCachingOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
