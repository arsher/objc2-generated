//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidynamicitembehavior?language=objc)
    #[unsafe(super(UIDynamicBehavior, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UIDynamicItemBehavior;
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UIDynamicItemBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIDynamicItemBehavior {
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<ProtocolObject<dyn UIDynamicItem>>,
        ) -> Retained<Self>;

        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &ProtocolObject<dyn UIDynamicItem>);

        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Retained<NSArray<ProtocolObject<dyn UIDynamicItem>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(elasticity)]
        pub unsafe fn elasticity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setElasticity:)]
        pub unsafe fn setElasticity(&self, elasticity: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(friction)]
        pub unsafe fn friction(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setFriction:)]
        pub unsafe fn setFriction(&self, friction: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(density)]
        pub unsafe fn density(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setDensity:)]
        pub unsafe fn setDensity(&self, density: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(resistance)]
        pub unsafe fn resistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setResistance:)]
        pub unsafe fn setResistance(&self, resistance: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(angularResistance)]
        pub unsafe fn angularResistance(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAngularResistance:)]
        pub unsafe fn setAngularResistance(&self, angular_resistance: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(charge)]
        pub unsafe fn charge(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCharge:)]
        pub unsafe fn setCharge(&self, charge: CGFloat);

        #[method(isAnchored)]
        pub unsafe fn isAnchored(&self) -> bool;

        #[method(setAnchored:)]
        pub unsafe fn setAnchored(&self, anchored: bool);

        #[method(allowsRotation)]
        pub unsafe fn allowsRotation(&self) -> bool;

        #[method(setAllowsRotation:)]
        pub unsafe fn setAllowsRotation(&self, allows_rotation: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(addLinearVelocity:forItem:)]
        pub unsafe fn addLinearVelocity_forItem(
            &self,
            velocity: CGPoint,
            item: &ProtocolObject<dyn UIDynamicItem>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(linearVelocityForItem:)]
        pub unsafe fn linearVelocityForItem(
            &self,
            item: &ProtocolObject<dyn UIDynamicItem>,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(addAngularVelocity:forItem:)]
        pub unsafe fn addAngularVelocity_forItem(
            &self,
            velocity: CGFloat,
            item: &ProtocolObject<dyn UIDynamicItem>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(angularVelocityForItem:)]
        pub unsafe fn angularVelocityForItem(
            &self,
            item: &ProtocolObject<dyn UIDynamicItem>,
        ) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UIDynamicItemBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
