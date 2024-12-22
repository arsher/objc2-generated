//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayeremittershape?language=objc)
// NS_TYPED_ENUM
pub type CAEmitterLayerEmitterShape = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayeremittermode?language=objc)
// NS_TYPED_ENUM
pub type CAEmitterLayerEmitterMode = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayerrendermode?language=objc)
// NS_TYPED_ENUM
pub type CAEmitterLayerRenderMode = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caemitterlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAEmitterLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CAEmitterLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CAEmitterLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CAEmitterLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CAEmitterLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CAEmitterLayer {
        #[cfg(feature = "CAEmitterCell")]
        #[method_id(@__retain_semantics Other emitterCells)]
        pub unsafe fn emitterCells(&self) -> Option<Retained<NSArray<CAEmitterCell>>>;

        #[cfg(feature = "CAEmitterCell")]
        /// Setter for [`emitterCells`][Self::emitterCells].
        #[method(setEmitterCells:)]
        pub unsafe fn setEmitterCells(&self, emitter_cells: Option<&NSArray<CAEmitterCell>>);

        #[method(birthRate)]
        pub unsafe fn birthRate(&self) -> c_float;

        /// Setter for [`birthRate`][Self::birthRate].
        #[method(setBirthRate:)]
        pub unsafe fn setBirthRate(&self, birth_rate: c_float);

        #[method(lifetime)]
        pub unsafe fn lifetime(&self) -> c_float;

        /// Setter for [`lifetime`][Self::lifetime].
        #[method(setLifetime:)]
        pub unsafe fn setLifetime(&self, lifetime: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(emitterPosition)]
        pub unsafe fn emitterPosition(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterPosition`][Self::emitterPosition].
        #[method(setEmitterPosition:)]
        pub unsafe fn setEmitterPosition(&self, emitter_position: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(emitterZPosition)]
        pub unsafe fn emitterZPosition(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterZPosition`][Self::emitterZPosition].
        #[method(setEmitterZPosition:)]
        pub unsafe fn setEmitterZPosition(&self, emitter_z_position: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(emitterSize)]
        pub unsafe fn emitterSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterSize`][Self::emitterSize].
        #[method(setEmitterSize:)]
        pub unsafe fn setEmitterSize(&self, emitter_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(emitterDepth)]
        pub unsafe fn emitterDepth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`emitterDepth`][Self::emitterDepth].
        #[method(setEmitterDepth:)]
        pub unsafe fn setEmitterDepth(&self, emitter_depth: CGFloat);

        #[method_id(@__retain_semantics Other emitterShape)]
        pub unsafe fn emitterShape(&self) -> Retained<CAEmitterLayerEmitterShape>;

        /// Setter for [`emitterShape`][Self::emitterShape].
        #[method(setEmitterShape:)]
        pub unsafe fn setEmitterShape(&self, emitter_shape: &CAEmitterLayerEmitterShape);

        #[method_id(@__retain_semantics Other emitterMode)]
        pub unsafe fn emitterMode(&self) -> Retained<CAEmitterLayerEmitterMode>;

        /// Setter for [`emitterMode`][Self::emitterMode].
        #[method(setEmitterMode:)]
        pub unsafe fn setEmitterMode(&self, emitter_mode: &CAEmitterLayerEmitterMode);

        #[method_id(@__retain_semantics Other renderMode)]
        pub unsafe fn renderMode(&self) -> Retained<CAEmitterLayerRenderMode>;

        /// Setter for [`renderMode`][Self::renderMode].
        #[method(setRenderMode:)]
        pub unsafe fn setRenderMode(&self, render_mode: &CAEmitterLayerRenderMode);

        #[method(preservesDepth)]
        pub unsafe fn preservesDepth(&self) -> bool;

        /// Setter for [`preservesDepth`][Self::preservesDepth].
        #[method(setPreservesDepth:)]
        pub unsafe fn setPreservesDepth(&self, preserves_depth: bool);

        #[method(velocity)]
        pub unsafe fn velocity(&self) -> c_float;

        /// Setter for [`velocity`][Self::velocity].
        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: c_float);

        #[method(scale)]
        pub unsafe fn scale(&self) -> c_float;

        /// Setter for [`scale`][Self::scale].
        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: c_float);

        #[method(spin)]
        pub unsafe fn spin(&self) -> c_float;

        /// Setter for [`spin`][Self::spin].
        #[method(setSpin:)]
        pub unsafe fn setSpin(&self, spin: c_float);

        #[method(seed)]
        pub unsafe fn seed(&self) -> c_uint;

        /// Setter for [`seed`][Self::seed].
        #[method(setSeed:)]
        pub unsafe fn setSeed(&self, seed: c_uint);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CAEmitterLayer {
        /// Layer creation and initialization. *
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
    unsafe impl CAEmitterLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// `emitterShape' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerpoint?language=objc)
    pub static kCAEmitterLayerPoint: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerline?language=objc)
    pub static kCAEmitterLayerLine: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerrectangle?language=objc)
    pub static kCAEmitterLayerRectangle: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayercuboid?language=objc)
    pub static kCAEmitterLayerCuboid: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayercircle?language=objc)
    pub static kCAEmitterLayerCircle: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayersphere?language=objc)
    pub static kCAEmitterLayerSphere: &'static CAEmitterLayerEmitterShape;
}

extern "C" {
    /// `emitterMode' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerpoints?language=objc)
    pub static kCAEmitterLayerPoints: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeroutline?language=objc)
    pub static kCAEmitterLayerOutline: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayersurface?language=objc)
    pub static kCAEmitterLayerSurface: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayervolume?language=objc)
    pub static kCAEmitterLayerVolume: &'static CAEmitterLayerEmitterMode;
}

extern "C" {
    /// `renderMode' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerunordered?language=objc)
    pub static kCAEmitterLayerUnordered: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeroldestfirst?language=objc)
    pub static kCAEmitterLayerOldestFirst: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeroldestlast?language=objc)
    pub static kCAEmitterLayerOldestLast: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayerbacktofront?language=objc)
    pub static kCAEmitterLayerBackToFront: &'static CAEmitterLayerRenderMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaemitterlayeradditive?language=objc)
    pub static kCAEmitterLayerAdditive: &'static CAEmitterLayerRenderMode;
}
