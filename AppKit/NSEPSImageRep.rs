//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsepsimagerep?language=objc)
    #[unsafe(super(NSImageRep, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    #[deprecated = "`NSEPSImageRep` instances cannot be created on macOS 14.0 and later"]
    pub struct NSEPSImageRep;
);

#[cfg(feature = "NSImageRep")]
unsafe impl NSCoding for NSEPSImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl NSCopying for NSEPSImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl CopyingHelper for NSEPSImageRep {
    type Result = Self;
}

#[cfg(feature = "NSImageRep")]
unsafe impl NSObjectProtocol for NSEPSImageRep {}

extern_methods!(
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSEPSImageRep {
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(eps_data: &NSData) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Allocated<Self>,
            eps_data: &NSData,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;

        #[method_id(@__retain_semantics Other EPSRepresentation)]
        pub unsafe fn EPSRepresentation(&self) -> Retained<NSData>;

        #[deprecated]
        #[method(prepareGState)]
        pub unsafe fn prepareGState(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSEPSImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSEPSImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
