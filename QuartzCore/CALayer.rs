//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayercontentsgravity?language=objc)
// NS_TYPED_ENUM
pub type CALayerContentsGravity = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayercontentsformat?language=objc)
// NS_TYPED_ENUM
pub type CALayerContentsFormat = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayercontentsfilter?language=objc)
// NS_TYPED_ENUM
pub type CALayerContentsFilter = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayercornercurve?language=objc)
// NS_TYPED_ENUM
pub type CALayerCornerCurve = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caautoresizingmask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAAutoresizingMask(pub c_uint);
bitflags::bitflags! {
    impl CAAutoresizingMask: c_uint {
        const kCALayerNotSizable = 0;
        const kCALayerMinXMargin = 1<<0;
        const kCALayerWidthSizable = 1<<1;
        const kCALayerMaxXMargin = 1<<2;
        const kCALayerMinYMargin = 1<<3;
        const kCALayerHeightSizable = 1<<4;
        const kCALayerMaxYMargin = 1<<5;
    }
}

unsafe impl Encode for CAAutoresizingMask {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for CAAutoresizingMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catonemapmode?language=objc)
// NS_TYPED_ENUM
pub type CAToneMapMode = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catonemapmodeautomatic?language=objc)
    pub static CAToneMapModeAutomatic: &'static CAToneMapMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catonemapmodenever?language=objc)
    pub static CAToneMapModeNever: &'static CAToneMapMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catonemapmodeifsupported?language=objc)
    pub static CAToneMapModeIfSupported: &'static CAToneMapMode;
}

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caedgeantialiasingmask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAEdgeAntialiasingMask(pub c_uint);
bitflags::bitflags! {
    impl CAEdgeAntialiasingMask: c_uint {
        const kCALayerLeftEdge = 1<<0;
        const kCALayerRightEdge = 1<<1;
        const kCALayerBottomEdge = 1<<2;
        const kCALayerTopEdge = 1<<3;
    }
}

unsafe impl Encode for CAEdgeAntialiasingMask {
    const ENCODING: Encoding = c_uint::ENCODING;
}

unsafe impl RefEncode for CAEdgeAntialiasingMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cacornermask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CACornerMask(pub NSUInteger);
bitflags::bitflags! {
    impl CACornerMask: NSUInteger {
        const kCALayerMinXMinYCorner = 1<<0;
        const kCALayerMaxXMinYCorner = 1<<1;
        const kCALayerMinXMaxYCorner = 1<<2;
        const kCALayerMaxXMaxYCorner = 1<<3;
    }
}

unsafe impl Encode for CACornerMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CACornerMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// The base layer class. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CALayer;
);

#[cfg(feature = "CAMediaTiming")]
unsafe impl CAMediaTiming for CALayer {}

unsafe impl NSCoding for CALayer {}

unsafe impl NSObjectProtocol for CALayer {}

unsafe impl NSSecureCoding for CALayer {}

extern_methods!(
    unsafe impl CALayer {
        /// Layer creation and initialization. *
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;

        #[method_id(@__retain_semantics Other presentationLayer)]
        pub unsafe fn presentationLayer(&self) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other modelLayer)]
        pub unsafe fn modelLayer(&self) -> Retained<Self>;

        /// Property methods. *
        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Retained<AnyObject>>;

        #[method(needsDisplayForKey:)]
        pub unsafe fn needsDisplayForKey(key: &NSString) -> bool;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        /// Geometry and layer hierarchy properties. *
        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`bounds`][Self::bounds].
        #[method(setBounds:)]
        pub fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(position)]
        pub fn position(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`position`][Self::position].
        #[method(setPosition:)]
        pub fn setPosition(&self, position: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(zPosition)]
        pub fn zPosition(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`zPosition`][Self::zPosition].
        #[method(setZPosition:)]
        pub fn setZPosition(&self, z_position: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(anchorPoint)]
        pub fn anchorPoint(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`anchorPoint`][Self::anchorPoint].
        #[method(setAnchorPoint:)]
        pub fn setAnchorPoint(&self, anchor_point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(anchorPointZ)]
        pub fn anchorPointZ(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`anchorPointZ`][Self::anchorPointZ].
        #[method(setAnchorPointZ:)]
        pub fn setAnchorPointZ(&self, anchor_point_z: CGFloat);

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        #[method(transform)]
        pub fn transform(&self) -> CATransform3D;

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        /// Setter for [`transform`][Self::transform].
        #[method(setTransform:)]
        pub fn setTransform(&self, transform: CATransform3D);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(affineTransform)]
        pub fn affineTransform(&self) -> CGAffineTransform;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAffineTransform:)]
        pub fn setAffineTransform(&self, m: CGAffineTransform);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(frame)]
        pub fn frame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`frame`][Self::frame].
        #[method(setFrame:)]
        pub fn setFrame(&self, frame: CGRect);

        #[method(isHidden)]
        pub fn isHidden(&self) -> bool;

        /// Setter for [`isHidden`][Self::isHidden].
        #[method(setHidden:)]
        pub fn setHidden(&self, hidden: bool);

        #[method(isDoubleSided)]
        pub fn isDoubleSided(&self) -> bool;

        /// Setter for [`isDoubleSided`][Self::isDoubleSided].
        #[method(setDoubleSided:)]
        pub fn setDoubleSided(&self, double_sided: bool);

        #[method(isGeometryFlipped)]
        pub fn isGeometryFlipped(&self) -> bool;

        /// Setter for [`isGeometryFlipped`][Self::isGeometryFlipped].
        #[method(setGeometryFlipped:)]
        pub fn setGeometryFlipped(&self, geometry_flipped: bool);

        #[method(contentsAreFlipped)]
        pub fn contentsAreFlipped(&self) -> bool;

        #[method_id(@__retain_semantics Other superlayer)]
        pub fn superlayer(&self) -> Option<Retained<CALayer>>;

        #[method(removeFromSuperlayer)]
        pub fn removeFromSuperlayer(&self);

        #[method_id(@__retain_semantics Other sublayers)]
        pub unsafe fn sublayers(&self) -> Option<Retained<NSArray<CALayer>>>;

        /// Setter for [`sublayers`][Self::sublayers].
        #[method(setSublayers:)]
        pub unsafe fn setSublayers(&self, sublayers: Option<&NSArray<CALayer>>);

        #[method(addSublayer:)]
        pub fn addSublayer(&self, layer: &CALayer);

        #[method(insertSublayer:atIndex:)]
        pub fn insertSublayer_atIndex(&self, layer: &CALayer, idx: c_uint);

        #[method(insertSublayer:below:)]
        pub fn insertSublayer_below(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(insertSublayer:above:)]
        pub fn insertSublayer_above(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(replaceSublayer:with:)]
        pub unsafe fn replaceSublayer_with(&self, old_layer: &CALayer, new_layer: &CALayer);

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        #[method(sublayerTransform)]
        pub fn sublayerTransform(&self) -> CATransform3D;

        #[cfg(all(feature = "CATransform3D", feature = "objc2-core-foundation"))]
        /// Setter for [`sublayerTransform`][Self::sublayerTransform].
        #[method(setSublayerTransform:)]
        pub fn setSublayerTransform(&self, sublayer_transform: CATransform3D);

        #[method_id(@__retain_semantics Other mask)]
        pub fn mask(&self) -> Option<Retained<CALayer>>;

        /// Setter for [`mask`][Self::mask].
        #[method(setMask:)]
        pub unsafe fn setMask(&self, mask: Option<&CALayer>);

        #[method(masksToBounds)]
        pub fn masksToBounds(&self) -> bool;

        /// Setter for [`masksToBounds`][Self::masksToBounds].
        #[method(setMasksToBounds:)]
        pub fn setMasksToBounds(&self, masks_to_bounds: bool);

        #[cfg(feature = "objc2-core-foundation")]
        /// Mapping between layer coordinate and time spaces. *
        #[method(convertPoint:fromLayer:)]
        pub fn convertPoint_fromLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertPoint:toLayer:)]
        pub fn convertPoint_toLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertRect:fromLayer:)]
        pub fn convertRect_fromLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertRect:toLayer:)]
        pub fn convertRect_toLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertTime:fromLayer:)]
        pub fn convertTime_fromLayer(
            &self,
            t: CFTimeInterval,
            l: Option<&CALayer>,
        ) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(convertTime:toLayer:)]
        pub fn convertTime_toLayer(&self, t: CFTimeInterval, l: Option<&CALayer>)
            -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Hit testing methods. *
        #[method_id(@__retain_semantics Other hitTest:)]
        pub fn hitTest(&self, p: CGPoint) -> Option<Retained<CALayer>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(containsPoint:)]
        pub fn containsPoint(&self, p: CGPoint) -> bool;

        /// Layer content properties and methods. *
        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`contents`][Self::contents].
        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&AnyObject>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentsRect)]
        pub fn contentsRect(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`contentsRect`][Self::contentsRect].
        #[method(setContentsRect:)]
        pub fn setContentsRect(&self, contents_rect: CGRect);

        #[method_id(@__retain_semantics Other contentsGravity)]
        pub fn contentsGravity(&self) -> Retained<CALayerContentsGravity>;

        /// Setter for [`contentsGravity`][Self::contentsGravity].
        #[method(setContentsGravity:)]
        pub fn setContentsGravity(&self, contents_gravity: &CALayerContentsGravity);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentsScale)]
        pub fn contentsScale(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`contentsScale`][Self::contentsScale].
        #[method(setContentsScale:)]
        pub fn setContentsScale(&self, contents_scale: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentsCenter)]
        pub fn contentsCenter(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`contentsCenter`][Self::contentsCenter].
        #[method(setContentsCenter:)]
        pub fn setContentsCenter(&self, contents_center: CGRect);

        #[method_id(@__retain_semantics Other contentsFormat)]
        pub fn contentsFormat(&self) -> Retained<CALayerContentsFormat>;

        /// Setter for [`contentsFormat`][Self::contentsFormat].
        #[method(setContentsFormat:)]
        pub fn setContentsFormat(&self, contents_format: &CALayerContentsFormat);

        #[method(wantsExtendedDynamicRangeContent)]
        pub unsafe fn wantsExtendedDynamicRangeContent(&self) -> bool;

        /// Setter for [`wantsExtendedDynamicRangeContent`][Self::wantsExtendedDynamicRangeContent].
        #[method(setWantsExtendedDynamicRangeContent:)]
        pub unsafe fn setWantsExtendedDynamicRangeContent(
            &self,
            wants_extended_dynamic_range_content: bool,
        );

        #[method_id(@__retain_semantics Other toneMapMode)]
        pub unsafe fn toneMapMode(&self) -> Retained<CAToneMapMode>;

        /// Setter for [`toneMapMode`][Self::toneMapMode].
        #[method(setToneMapMode:)]
        pub unsafe fn setToneMapMode(&self, tone_map_mode: &CAToneMapMode);

        #[method(wantsDynamicContentScaling)]
        pub unsafe fn wantsDynamicContentScaling(&self) -> bool;

        /// Setter for [`wantsDynamicContentScaling`][Self::wantsDynamicContentScaling].
        #[method(setWantsDynamicContentScaling:)]
        pub unsafe fn setWantsDynamicContentScaling(&self, wants_dynamic_content_scaling: bool);

        #[method_id(@__retain_semantics Other minificationFilter)]
        pub fn minificationFilter(&self) -> Retained<CALayerContentsFilter>;

        /// Setter for [`minificationFilter`][Self::minificationFilter].
        #[method(setMinificationFilter:)]
        pub fn setMinificationFilter(&self, minification_filter: &CALayerContentsFilter);

        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub fn magnificationFilter(&self) -> Retained<CALayerContentsFilter>;

        /// Setter for [`magnificationFilter`][Self::magnificationFilter].
        #[method(setMagnificationFilter:)]
        pub fn setMagnificationFilter(&self, magnification_filter: &CALayerContentsFilter);

        #[method(minificationFilterBias)]
        pub fn minificationFilterBias(&self) -> c_float;

        /// Setter for [`minificationFilterBias`][Self::minificationFilterBias].
        #[method(setMinificationFilterBias:)]
        pub fn setMinificationFilterBias(&self, minification_filter_bias: c_float);

        #[method(isOpaque)]
        pub fn isOpaque(&self) -> bool;

        /// Setter for [`isOpaque`][Self::isOpaque].
        #[method(setOpaque:)]
        pub fn setOpaque(&self, opaque: bool);

        #[method(display)]
        pub fn display(&self);

        #[method(setNeedsDisplay)]
        pub fn setNeedsDisplay(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setNeedsDisplayInRect:)]
        pub fn setNeedsDisplayInRect(&self, r: CGRect);

        #[method(needsDisplay)]
        pub fn needsDisplay(&self) -> bool;

        #[method(displayIfNeeded)]
        pub fn displayIfNeeded(&self);

        #[method(needsDisplayOnBoundsChange)]
        pub fn needsDisplayOnBoundsChange(&self) -> bool;

        /// Setter for [`needsDisplayOnBoundsChange`][Self::needsDisplayOnBoundsChange].
        #[method(setNeedsDisplayOnBoundsChange:)]
        pub fn setNeedsDisplayOnBoundsChange(&self, needs_display_on_bounds_change: bool);

        #[method(drawsAsynchronously)]
        pub fn drawsAsynchronously(&self) -> bool;

        /// Setter for [`drawsAsynchronously`][Self::drawsAsynchronously].
        #[method(setDrawsAsynchronously:)]
        pub fn setDrawsAsynchronously(&self, draws_asynchronously: bool);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(drawInContext:)]
        pub unsafe fn drawInContext(&self, ctx: CGContextRef);

        #[cfg(feature = "objc2-core-graphics")]
        /// Rendering properties and methods. *
        #[method(renderInContext:)]
        pub unsafe fn renderInContext(&self, ctx: CGContextRef);

        #[method(edgeAntialiasingMask)]
        pub fn edgeAntialiasingMask(&self) -> CAEdgeAntialiasingMask;

        /// Setter for [`edgeAntialiasingMask`][Self::edgeAntialiasingMask].
        #[method(setEdgeAntialiasingMask:)]
        pub fn setEdgeAntialiasingMask(&self, edge_antialiasing_mask: CAEdgeAntialiasingMask);

        #[method(allowsEdgeAntialiasing)]
        pub fn allowsEdgeAntialiasing(&self) -> bool;

        /// Setter for [`allowsEdgeAntialiasing`][Self::allowsEdgeAntialiasing].
        #[method(setAllowsEdgeAntialiasing:)]
        pub fn setAllowsEdgeAntialiasing(&self, allows_edge_antialiasing: bool);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> CGColorRef;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: CGColorRef);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cornerRadius)]
        pub fn cornerRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`cornerRadius`][Self::cornerRadius].
        #[method(setCornerRadius:)]
        pub fn setCornerRadius(&self, corner_radius: CGFloat);

        #[method(maskedCorners)]
        pub fn maskedCorners(&self) -> CACornerMask;

        /// Setter for [`maskedCorners`][Self::maskedCorners].
        #[method(setMaskedCorners:)]
        pub fn setMaskedCorners(&self, masked_corners: CACornerMask);

        #[method_id(@__retain_semantics Other cornerCurve)]
        pub fn cornerCurve(&self) -> Retained<CALayerCornerCurve>;

        /// Setter for [`cornerCurve`][Self::cornerCurve].
        #[method(setCornerCurve:)]
        pub fn setCornerCurve(&self, corner_curve: &CALayerCornerCurve);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cornerCurveExpansionFactor:)]
        pub fn cornerCurveExpansionFactor(curve: &CALayerCornerCurve) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(borderWidth)]
        pub fn borderWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`borderWidth`][Self::borderWidth].
        #[method(setBorderWidth:)]
        pub fn setBorderWidth(&self, border_width: CGFloat);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(borderColor)]
        pub unsafe fn borderColor(&self) -> CGColorRef;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`borderColor`][Self::borderColor].
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, border_color: CGColorRef);

        #[method(opacity)]
        pub fn opacity(&self) -> c_float;

        /// Setter for [`opacity`][Self::opacity].
        #[method(setOpacity:)]
        pub fn setOpacity(&self, opacity: c_float);

        #[method(allowsGroupOpacity)]
        pub fn allowsGroupOpacity(&self) -> bool;

        /// Setter for [`allowsGroupOpacity`][Self::allowsGroupOpacity].
        #[method(setAllowsGroupOpacity:)]
        pub fn setAllowsGroupOpacity(&self, allows_group_opacity: bool);

        #[method_id(@__retain_semantics Other compositingFilter)]
        pub unsafe fn compositingFilter(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`compositingFilter`][Self::compositingFilter].
        #[method(setCompositingFilter:)]
        pub unsafe fn setCompositingFilter(&self, compositing_filter: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other filters)]
        pub unsafe fn filters(&self) -> Option<Retained<NSArray>>;

        /// Setter for [`filters`][Self::filters].
        #[method(setFilters:)]
        pub unsafe fn setFilters(&self, filters: Option<&NSArray>);

        #[method_id(@__retain_semantics Other backgroundFilters)]
        pub unsafe fn backgroundFilters(&self) -> Option<Retained<NSArray>>;

        /// Setter for [`backgroundFilters`][Self::backgroundFilters].
        #[method(setBackgroundFilters:)]
        pub unsafe fn setBackgroundFilters(&self, background_filters: Option<&NSArray>);

        #[method(shouldRasterize)]
        pub fn shouldRasterize(&self) -> bool;

        /// Setter for [`shouldRasterize`][Self::shouldRasterize].
        #[method(setShouldRasterize:)]
        pub fn setShouldRasterize(&self, should_rasterize: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rasterizationScale)]
        pub fn rasterizationScale(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`rasterizationScale`][Self::rasterizationScale].
        #[method(setRasterizationScale:)]
        pub fn setRasterizationScale(&self, rasterization_scale: CGFloat);

        #[cfg(feature = "objc2-core-graphics")]
        /// Shadow properties. *
        #[method(shadowColor)]
        pub unsafe fn shadowColor(&self) -> CGColorRef;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`shadowColor`][Self::shadowColor].
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadow_color: CGColorRef);

        #[method(shadowOpacity)]
        pub fn shadowOpacity(&self) -> c_float;

        /// Setter for [`shadowOpacity`][Self::shadowOpacity].
        #[method(setShadowOpacity:)]
        pub fn setShadowOpacity(&self, shadow_opacity: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(shadowOffset)]
        pub fn shadowOffset(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowOffset`][Self::shadowOffset].
        #[method(setShadowOffset:)]
        pub fn setShadowOffset(&self, shadow_offset: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(shadowRadius)]
        pub fn shadowRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`shadowRadius`][Self::shadowRadius].
        #[method(setShadowRadius:)]
        pub fn setShadowRadius(&self, shadow_radius: CGFloat);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(shadowPath)]
        pub unsafe fn shadowPath(&self) -> CGPathRef;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`shadowPath`][Self::shadowPath].
        #[method(setShadowPath:)]
        pub unsafe fn setShadowPath(&self, shadow_path: CGPathRef);

        /// Layout methods. *
        #[method(autoresizingMask)]
        pub fn autoresizingMask(&self) -> CAAutoresizingMask;

        /// Setter for [`autoresizingMask`][Self::autoresizingMask].
        #[method(setAutoresizingMask:)]
        pub fn setAutoresizingMask(&self, autoresizing_mask: CAAutoresizingMask);

        #[method_id(@__retain_semantics Other layoutManager)]
        pub fn layoutManager(&self) -> Option<Retained<ProtocolObject<dyn CALayoutManager>>>;

        /// Setter for [`layoutManager`][Self::layoutManager].
        #[method(setLayoutManager:)]
        pub fn setLayoutManager(
            &self,
            layout_manager: Option<&ProtocolObject<dyn CALayoutManager>>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredFrameSize)]
        pub fn preferredFrameSize(&self) -> CGSize;

        #[method(setNeedsLayout)]
        pub fn setNeedsLayout(&self);

        #[method(needsLayout)]
        pub fn needsLayout(&self) -> bool;

        #[method(layoutIfNeeded)]
        pub fn layoutIfNeeded(&self);

        #[method(layoutSublayers)]
        pub fn layoutSublayers(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(resizeSublayersWithOldSize:)]
        pub fn resizeSublayersWithOldSize(&self, size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(resizeWithOldSuperlayerSize:)]
        pub fn resizeWithOldSuperlayerSize(&self, size: CGSize);

        /// Action methods. *
        #[method_id(@__retain_semantics Other defaultActionForKey:)]
        pub fn defaultActionForKey(
            event: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn CAAction>>>;

        #[method_id(@__retain_semantics Other actionForKey:)]
        pub fn actionForKey(
            &self,
            event: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn CAAction>>>;

        #[method_id(@__retain_semantics Other actions)]
        pub fn actions(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, ProtocolObject<dyn CAAction>>>>;

        /// Setter for [`actions`][Self::actions].
        #[method(setActions:)]
        pub fn setActions(
            &self,
            actions: Option<&NSDictionary<NSString, ProtocolObject<dyn CAAction>>>,
        );

        #[cfg(feature = "CAAnimation")]
        /// Animation methods. *
        #[method(addAnimation:forKey:)]
        pub fn addAnimation_forKey(&self, anim: &CAAnimation, key: Option<&NSString>);

        #[method(removeAllAnimations)]
        pub fn removeAllAnimations(&self);

        #[method(removeAnimationForKey:)]
        pub fn removeAnimationForKey(&self, key: &NSString);

        #[method_id(@__retain_semantics Other animationKeys)]
        pub fn animationKeys(&self) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "CAAnimation")]
        #[method_id(@__retain_semantics Other animationForKey:)]
        pub unsafe fn animationForKey(&self, key: &NSString) -> Option<Retained<CAAnimation>>;

        /// Miscellaneous properties. *
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other delegate)]
        pub fn delegate(&self) -> Option<Retained<ProtocolObject<dyn CALayerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn CALayerDelegate>>);

        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Retained<NSDictionary>>;

        /// Setter for [`style`][Self::style].
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: Option<&NSDictionary>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CALayer {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for CALayer {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    /// Layout manager protocol. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayoutmanager?language=objc)
    pub unsafe trait CALayoutManager: NSObjectProtocol {
        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method(preferredSizeOfLayer:)]
        unsafe fn preferredSizeOfLayer(&self, layer: &CALayer) -> CGSize;

        #[optional]
        #[method(invalidateLayoutOfLayer:)]
        unsafe fn invalidateLayoutOfLayer(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);
    }

    unsafe impl ProtocolType for dyn CALayoutManager {}
);

extern_protocol!(
    /// Action (event handler) protocol. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/caaction?language=objc)
    pub unsafe trait CAAction {
        #[method(runActionForKey:object:arguments:)]
        unsafe fn runActionForKey_object_arguments(
            &self,
            event: &NSString,
            an_object: &AnyObject,
            dict: Option<&NSDictionary>,
        );
    }

    unsafe impl ProtocolType for dyn CAAction {}
);

unsafe impl CAAction for NSNull {}

extern_protocol!(
    /// Delegate methods. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/calayerdelegate?language=objc)
    pub unsafe trait CALayerDelegate: NSObjectProtocol {
        #[optional]
        #[method(displayLayer:)]
        unsafe fn displayLayer(&self, layer: &CALayer);

        #[cfg(feature = "objc2-core-graphics")]
        #[optional]
        #[method(drawLayer:inContext:)]
        unsafe fn drawLayer_inContext(&self, layer: &CALayer, ctx: CGContextRef);

        #[optional]
        #[method(layerWillDraw:)]
        unsafe fn layerWillDraw(&self, layer: &CALayer);

        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);

        #[optional]
        #[method_id(@__retain_semantics Other actionForLayer:forKey:)]
        unsafe fn actionForLayer_forKey(
            &self,
            layer: &CALayer,
            event: &NSString,
        ) -> Option<Retained<ProtocolObject<dyn CAAction>>>;
    }

    unsafe impl ProtocolType for dyn CALayerDelegate {}
);

extern "C" {
    /// Layer `contentsGravity' values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitycenter?language=objc)
    pub static kCAGravityCenter: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitytop?language=objc)
    pub static kCAGravityTop: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitybottom?language=objc)
    pub static kCAGravityBottom: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravityleft?language=objc)
    pub static kCAGravityLeft: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravityright?language=objc)
    pub static kCAGravityRight: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitytopleft?language=objc)
    pub static kCAGravityTopLeft: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitytopright?language=objc)
    pub static kCAGravityTopRight: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitybottomleft?language=objc)
    pub static kCAGravityBottomLeft: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravitybottomright?language=objc)
    pub static kCAGravityBottomRight: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravityresize?language=objc)
    pub static kCAGravityResize: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravityresizeaspect?language=objc)
    pub static kCAGravityResizeAspect: &'static CALayerContentsGravity;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagravityresizeaspectfill?language=objc)
    pub static kCAGravityResizeAspectFill: &'static CALayerContentsGravity;
}

extern "C" {
    /// Layer `contentsFormat` values. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcacontentsformatrgba8uint?language=objc)
    pub static kCAContentsFormatRGBA8Uint: &'static CALayerContentsFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcacontentsformatrgba16float?language=objc)
    pub static kCAContentsFormatRGBA16Float: &'static CALayerContentsFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcacontentsformatgray8uint?language=objc)
    pub static kCAContentsFormatGray8Uint: &'static CALayerContentsFormat;
}

extern "C" {
    /// Contents filter names. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafilternearest?language=objc)
    pub static kCAFilterNearest: &'static CALayerContentsFilter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafilterlinear?language=objc)
    pub static kCAFilterLinear: &'static CALayerContentsFilter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcafiltertrilinear?language=objc)
    pub static kCAFilterTrilinear: &'static CALayerContentsFilter;
}

extern "C" {
    /// Corner curve names. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcacornercurvecircular?language=objc)
    pub static kCACornerCurveCircular: &'static CALayerCornerCurve;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcacornercurvecontinuous?language=objc)
    pub static kCACornerCurveContinuous: &'static CALayerCornerCurve;
}

extern "C" {
    /// Layer event names. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaonorderin?language=objc)
    pub static kCAOnOrderIn: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaonorderout?language=objc)
    pub static kCAOnOrderOut: &'static NSString;
}

extern "C" {
    /// The animation key used for transitions. *
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatransition?language=objc)
    pub static kCATransition: &'static NSString;
}
