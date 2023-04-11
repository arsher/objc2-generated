//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSEPSImageRep")]
    pub struct NSEPSImageRep;

    #[cfg(feature = "AppKit_NSEPSImageRep")]
    unsafe impl ClassType for NSEPSImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSEPSImageRep")]
unsafe impl NSCoding for NSEPSImageRep {}

#[cfg(feature = "AppKit_NSEPSImageRep")]
unsafe impl NSCopying for NSEPSImageRep {}

#[cfg(feature = "AppKit_NSEPSImageRep")]
unsafe impl NSObjectProtocol for NSEPSImageRep {}

extern_methods!(
    #[cfg(feature = "AppKit_NSEPSImageRep")]
    unsafe impl NSEPSImageRep {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(eps_data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            eps_data: &NSData,
        ) -> Option<Id<Self>>;

        #[deprecated]
        #[method(prepareGState)]
        pub unsafe fn prepareGState(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other EPSRepresentation)]
        pub unsafe fn EPSRepresentation(&self) -> Id<NSData>;

        #[method(boundingBox)]
        pub unsafe fn boundingBox(&self) -> NSRect;
    }
);
