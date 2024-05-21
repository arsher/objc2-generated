//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNPoint;

    unsafe impl ClassType for VNPoint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for VNPoint {}

unsafe impl NSCopying for VNPoint {}

unsafe impl NSObjectProtocol for VNPoint {}

unsafe impl NSSecureCoding for VNPoint {}

extern_methods!(
    unsafe impl VNPoint {
        #[method_id(@__retain_semantics Other zeroPoint)]
        pub unsafe fn zeroPoint() -> Retained<VNPoint>;

        #[method_id(@__retain_semantics Other pointByApplyingVector:toPoint:)]
        pub unsafe fn pointByApplyingVector_toPoint(
            vector: &VNVector,
            point: &VNPoint,
        ) -> Retained<VNPoint>;

        #[deprecated]
        #[method(distanceBetweenPoint:point:)]
        pub unsafe fn distanceBetweenPoint_point(point1: &VNPoint, point2: &VNPoint) -> c_double;

        #[method(distanceToPoint:)]
        pub unsafe fn distanceToPoint(&self, point: &VNPoint) -> c_double;

        #[method_id(@__retain_semantics Init initWithX:y:)]
        pub unsafe fn initWithX_y(
            this: Allocated<Self>,
            x: c_double,
            y: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocation:)]
        pub unsafe fn initWithLocation(this: Allocated<Self>, location: CGPoint) -> Retained<Self>;

        #[method(location)]
        pub unsafe fn location(&self) -> CGPoint;

        #[method(x)]
        pub unsafe fn x(&self) -> c_double;

        #[method(y)]
        pub unsafe fn y(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNPoint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNPoint3D;

    unsafe impl ClassType for VNPoint3D {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for VNPoint3D {}

unsafe impl NSCopying for VNPoint3D {}

unsafe impl NSObjectProtocol for VNPoint3D {}

unsafe impl NSSecureCoding for VNPoint3D {}

extern_methods!(
    unsafe impl VNPoint3D {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNPoint3D {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNVector;

    unsafe impl ClassType for VNVector {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for VNVector {}

unsafe impl NSCopying for VNVector {}

unsafe impl NSObjectProtocol for VNVector {}

unsafe impl NSSecureCoding for VNVector {}

extern_methods!(
    unsafe impl VNVector {
        #[method_id(@__retain_semantics Other zeroVector)]
        pub unsafe fn zeroVector() -> Retained<VNVector>;

        #[method_id(@__retain_semantics Other unitVectorForVector:)]
        pub unsafe fn unitVectorForVector(vector: &VNVector) -> Retained<VNVector>;

        #[method_id(@__retain_semantics Other vectorByMultiplyingVector:byScalar:)]
        pub unsafe fn vectorByMultiplyingVector_byScalar(
            vector: &VNVector,
            scalar: c_double,
        ) -> Retained<VNVector>;

        #[method_id(@__retain_semantics Other vectorByAddingVector:toVector:)]
        pub unsafe fn vectorByAddingVector_toVector(
            v1: &VNVector,
            v2: &VNVector,
        ) -> Retained<VNVector>;

        #[method_id(@__retain_semantics Other vectorBySubtractingVector:fromVector:)]
        pub unsafe fn vectorBySubtractingVector_fromVector(
            v1: &VNVector,
            v2: &VNVector,
        ) -> Retained<VNVector>;

        #[method(dotProductOfVector:vector:)]
        pub unsafe fn dotProductOfVector_vector(v1: &VNVector, v2: &VNVector) -> c_double;

        #[method_id(@__retain_semantics Init initWithXComponent:yComponent:)]
        pub unsafe fn initWithXComponent_yComponent(
            this: Allocated<Self>,
            x: c_double,
            y: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithR:theta:)]
        pub unsafe fn initWithR_theta(
            this: Allocated<Self>,
            r: c_double,
            theta: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithVectorHead:tail:)]
        pub unsafe fn initWithVectorHead_tail(
            this: Allocated<Self>,
            head: &VNPoint,
            tail: &VNPoint,
        ) -> Retained<Self>;

        #[method(x)]
        pub unsafe fn x(&self) -> c_double;

        #[method(y)]
        pub unsafe fn y(&self) -> c_double;

        #[method(r)]
        pub unsafe fn r(&self) -> c_double;

        #[method(theta)]
        pub unsafe fn theta(&self) -> c_double;

        #[method(length)]
        pub unsafe fn length(&self) -> c_double;

        #[method(squaredLength)]
        pub unsafe fn squaredLength(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNVector {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNCircle;

    unsafe impl ClassType for VNCircle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for VNCircle {}

unsafe impl NSCopying for VNCircle {}

unsafe impl NSObjectProtocol for VNCircle {}

unsafe impl NSSecureCoding for VNCircle {}

extern_methods!(
    unsafe impl VNCircle {
        #[method_id(@__retain_semantics Other zeroCircle)]
        pub unsafe fn zeroCircle() -> Retained<VNCircle>;

        #[method_id(@__retain_semantics Init initWithCenter:radius:)]
        pub unsafe fn initWithCenter_radius(
            this: Allocated<Self>,
            center: &VNPoint,
            radius: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCenter:diameter:)]
        pub unsafe fn initWithCenter_diameter(
            this: Allocated<Self>,
            center: &VNPoint,
            diameter: c_double,
        ) -> Retained<Self>;

        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: &VNPoint) -> bool;

        #[method(containsPoint:inCircumferentialRingOfWidth:)]
        pub unsafe fn containsPoint_inCircumferentialRingOfWidth(
            &self,
            point: &VNPoint,
            ring_width: c_double,
        ) -> bool;

        #[method_id(@__retain_semantics Other center)]
        pub unsafe fn center(&self) -> Retained<VNPoint>;

        #[method(radius)]
        pub unsafe fn radius(&self) -> c_double;

        #[method(diameter)]
        pub unsafe fn diameter(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VNCircle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VNContour;

    unsafe impl ClassType for VNContour {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VNContour {}

unsafe impl NSObjectProtocol for VNContour {}

#[cfg(feature = "VNRequestRevisionProviding")]
unsafe impl VNRequestRevisionProviding for VNContour {}

extern_methods!(
    unsafe impl VNContour {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other indexPath)]
        pub unsafe fn indexPath(&self) -> Retained<NSIndexPath>;

        #[method(childContourCount)]
        pub unsafe fn childContourCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other childContours)]
        pub unsafe fn childContours(&self) -> Retained<NSArray<VNContour>>;

        #[method_id(@__retain_semantics Other childContourAtIndex:error:_)]
        pub unsafe fn childContourAtIndex_error(
            &self,
            child_contour_index: NSUInteger,
        ) -> Result<Retained<VNContour>, Retained<NSError>>;

        #[method(pointCount)]
        pub unsafe fn pointCount(&self) -> NSInteger;

        #[method(aspectRatio)]
        pub unsafe fn aspectRatio(&self) -> c_float;

        #[method_id(@__retain_semantics Other polygonApproximationWithEpsilon:error:_)]
        pub unsafe fn polygonApproximationWithEpsilon_error(
            &self,
            epsilon: c_float,
        ) -> Result<Retained<VNContour>, Retained<NSError>>;
    }
);
