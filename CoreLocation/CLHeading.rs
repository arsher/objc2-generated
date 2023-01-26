//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLHeadingComponentValue = c_double;

extern_static!(kCLHeadingFilterNone: CLLocationDegrees);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLHeading")]
    pub struct CLHeading;

    #[cfg(feature = "CoreLocation_CLHeading")]
    unsafe impl ClassType for CLHeading {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLHeading")]
unsafe impl NSCoding for CLHeading {}

#[cfg(feature = "CoreLocation_CLHeading")]
unsafe impl NSObjectProtocol for CLHeading {}

#[cfg(feature = "CoreLocation_CLHeading")]
unsafe impl NSSecureCoding for CLHeading {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLHeading")]
    unsafe impl CLHeading {
        #[method(magneticHeading)]
        pub unsafe fn magneticHeading(&self) -> CLLocationDirection;

        #[method(trueHeading)]
        pub unsafe fn trueHeading(&self) -> CLLocationDirection;

        #[method(headingAccuracy)]
        pub unsafe fn headingAccuracy(&self) -> CLLocationDirection;

        #[method(x)]
        pub unsafe fn x(&self) -> CLHeadingComponentValue;

        #[method(y)]
        pub unsafe fn y(&self) -> CLHeadingComponentValue;

        #[method(z)]
        pub unsafe fn z(&self) -> CLHeadingComponentValue;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate, Shared>;
    }
);
