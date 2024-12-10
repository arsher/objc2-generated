//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/careplicatorlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAReplicatorLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CAReplicatorLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CAReplicatorLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CAReplicatorLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CAReplicatorLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CAReplicatorLayer {
        #[method(instanceCount)]
        pub unsafe fn instanceCount(&self) -> NSInteger;

        #[method(setInstanceCount:)]
        pub unsafe fn setInstanceCount(&self, instance_count: NSInteger);

        #[method(preservesDepth)]
        pub unsafe fn preservesDepth(&self) -> bool;

        #[method(setPreservesDepth:)]
        pub unsafe fn setPreservesDepth(&self, preserves_depth: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(instanceDelay)]
        pub unsafe fn instanceDelay(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setInstanceDelay:)]
        pub unsafe fn setInstanceDelay(&self, instance_delay: CFTimeInterval);

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        #[method(instanceTransform)]
        pub unsafe fn instanceTransform(&self) -> CATransform3D;

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        #[method(setInstanceTransform:)]
        pub unsafe fn setInstanceTransform(&self, instance_transform: CATransform3D);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(instanceColor)]
        pub unsafe fn instanceColor(&self) -> CGColorRef;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(setInstanceColor:)]
        pub unsafe fn setInstanceColor(&self, instance_color: CGColorRef);

        #[method(instanceRedOffset)]
        pub unsafe fn instanceRedOffset(&self) -> c_float;

        #[method(setInstanceRedOffset:)]
        pub unsafe fn setInstanceRedOffset(&self, instance_red_offset: c_float);

        #[method(instanceGreenOffset)]
        pub unsafe fn instanceGreenOffset(&self) -> c_float;

        #[method(setInstanceGreenOffset:)]
        pub unsafe fn setInstanceGreenOffset(&self, instance_green_offset: c_float);

        #[method(instanceBlueOffset)]
        pub unsafe fn instanceBlueOffset(&self) -> c_float;

        #[method(setInstanceBlueOffset:)]
        pub unsafe fn setInstanceBlueOffset(&self, instance_blue_offset: c_float);

        #[method(instanceAlphaOffset)]
        pub unsafe fn instanceAlphaOffset(&self) -> c_float;

        #[method(setInstanceAlphaOffset:)]
        pub unsafe fn setInstanceAlphaOffset(&self, instance_alpha_offset: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CAReplicatorLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CALayer")]
    unsafe impl CAReplicatorLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
