//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCYOLOLossDescriptor;

    unsafe impl ClassType for MLCYOLOLossDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MLCYOLOLossDescriptor {}

unsafe impl NSObjectProtocol for MLCYOLOLossDescriptor {}

extern_methods!(
    unsafe impl MLCYOLOLossDescriptor {
        #[deprecated]
        #[method(anchorBoxCount)]
        pub unsafe fn anchorBoxCount(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other anchorBoxes)]
        pub unsafe fn anchorBoxes(&self) -> Id<NSData>;

        #[deprecated]
        #[method(shouldRescore)]
        pub unsafe fn shouldRescore(&self) -> bool;

        #[deprecated]
        #[method(setShouldRescore:)]
        pub unsafe fn setShouldRescore(&self, should_rescore: bool);

        #[deprecated]
        #[method(scaleSpatialPositionLoss)]
        pub unsafe fn scaleSpatialPositionLoss(&self) -> c_float;

        #[deprecated]
        #[method(setScaleSpatialPositionLoss:)]
        pub unsafe fn setScaleSpatialPositionLoss(&self, scale_spatial_position_loss: c_float);

        #[deprecated]
        #[method(scaleSpatialSizeLoss)]
        pub unsafe fn scaleSpatialSizeLoss(&self) -> c_float;

        #[deprecated]
        #[method(setScaleSpatialSizeLoss:)]
        pub unsafe fn setScaleSpatialSizeLoss(&self, scale_spatial_size_loss: c_float);

        #[deprecated]
        #[method(scaleNoObjectConfidenceLoss)]
        pub unsafe fn scaleNoObjectConfidenceLoss(&self) -> c_float;

        #[deprecated]
        #[method(setScaleNoObjectConfidenceLoss:)]
        pub unsafe fn setScaleNoObjectConfidenceLoss(
            &self,
            scale_no_object_confidence_loss: c_float,
        );

        #[deprecated]
        #[method(scaleObjectConfidenceLoss)]
        pub unsafe fn scaleObjectConfidenceLoss(&self) -> c_float;

        #[deprecated]
        #[method(setScaleObjectConfidenceLoss:)]
        pub unsafe fn setScaleObjectConfidenceLoss(&self, scale_object_confidence_loss: c_float);

        #[deprecated]
        #[method(scaleClassLoss)]
        pub unsafe fn scaleClassLoss(&self) -> c_float;

        #[deprecated]
        #[method(setScaleClassLoss:)]
        pub unsafe fn setScaleClassLoss(&self, scale_class_loss: c_float);

        #[deprecated]
        #[method(minimumIOUForObjectPresence)]
        pub unsafe fn minimumIOUForObjectPresence(&self) -> c_float;

        #[deprecated]
        #[method(setMinimumIOUForObjectPresence:)]
        pub unsafe fn setMinimumIOUForObjectPresence(
            &self,
            minimum_iou_for_object_presence: c_float,
        );

        #[deprecated]
        #[method(maximumIOUForObjectAbsence)]
        pub unsafe fn maximumIOUForObjectAbsence(&self) -> c_float;

        #[deprecated]
        #[method(setMaximumIOUForObjectAbsence:)]
        pub unsafe fn setMaximumIOUForObjectAbsence(&self, maximum_iou_for_object_absence: c_float);

        #[deprecated]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other descriptorWithAnchorBoxes:anchorBoxCount:)]
        pub unsafe fn descriptorWithAnchorBoxes_anchorBoxCount(
            anchor_boxes: &NSData,
            anchor_box_count: NSUInteger,
        ) -> Id<Self>;
    }
);