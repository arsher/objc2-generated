//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caconstraintattribute?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAConstraintAttribute(pub c_int);
impl CAConstraintAttribute {
    pub const kCAConstraintMinX: Self = Self(0);
    pub const kCAConstraintMidX: Self = Self(1);
    pub const kCAConstraintMaxX: Self = Self(2);
    pub const kCAConstraintWidth: Self = Self(3);
    pub const kCAConstraintMinY: Self = Self(4);
    pub const kCAConstraintMidY: Self = Self(5);
    pub const kCAConstraintMaxY: Self = Self(6);
    pub const kCAConstraintHeight: Self = Self(7);
}

unsafe impl Encode for CAConstraintAttribute {
    const ENCODING: Encoding = c_int::ENCODING;
}

unsafe impl RefEncode for CAConstraintAttribute {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// CAConstraintLayoutManager
    #[cfg(feature = "CALayer")]
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other constraints)]
        pub unsafe fn constraints(&self) -> Option<Retained<NSArray<CAConstraint>>>;

        #[method(setConstraints:)]
        pub unsafe fn setConstraints(&self, constraints: Option<&NSArray<CAConstraint>>);

        #[method(addConstraint:)]
        pub unsafe fn addConstraint(&self, c: &CAConstraint);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caconstraintlayoutmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAConstraintLayoutManager;
);

#[cfg(feature = "CALayer")]
unsafe impl CALayoutManager for CAConstraintLayoutManager {}

unsafe impl NSObjectProtocol for CAConstraintLayoutManager {}

extern_methods!(
    unsafe impl CAConstraintLayoutManager {
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAConstraintLayoutManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caconstraint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAConstraint;
);

unsafe impl NSCoding for CAConstraint {}

unsafe impl NSObjectProtocol for CAConstraint {}

unsafe impl NSSecureCoding for CAConstraint {}

extern_methods!(
    unsafe impl CAConstraint {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other constraintWithAttribute:relativeTo:attribute:scale:offset:)]
        pub unsafe fn constraintWithAttribute_relativeTo_attribute_scale_offset(
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other constraintWithAttribute:relativeTo:attribute:offset:)]
        pub unsafe fn constraintWithAttribute_relativeTo_attribute_offset(
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
            c: CGFloat,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other constraintWithAttribute:relativeTo:attribute:)]
        pub unsafe fn constraintWithAttribute_relativeTo_attribute(
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithAttribute:relativeTo:attribute:scale:offset:)]
        pub unsafe fn initWithAttribute_relativeTo_attribute_scale_offset(
            this: Allocated<Self>,
            attr: CAConstraintAttribute,
            src_id: &NSString,
            src_attr: CAConstraintAttribute,
            m: CGFloat,
            c: CGFloat,
        ) -> Retained<Self>;

        #[method(attribute)]
        pub unsafe fn attribute(&self) -> CAConstraintAttribute;

        #[method_id(@__retain_semantics Other sourceName)]
        pub unsafe fn sourceName(&self) -> Retained<NSString>;

        #[method(sourceAttribute)]
        pub unsafe fn sourceAttribute(&self) -> CAConstraintAttribute;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scale)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(offset)]
        pub unsafe fn offset(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
