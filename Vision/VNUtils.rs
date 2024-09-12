//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static VNNormalizedIdentityRect: CGRect;
}

extern "C-unwind" {
    pub fn VNNormalizedRectIsIdentityRect(normalized_rect: CGRect) -> bool;
}

extern "C-unwind" {
    pub fn VNImagePointForNormalizedPoint(
        normalized_point: CGPoint,
        image_width: usize,
        image_height: usize,
    ) -> CGPoint;
}

extern "C-unwind" {
    pub fn VNImagePointForNormalizedPointUsingRegionOfInterest(
        normalized_point: CGPoint,
        image_width: usize,
        image_height: usize,
        roi: CGRect,
    ) -> CGPoint;
}

extern "C-unwind" {
    pub fn VNNormalizedPointForImagePoint(
        image_point: CGPoint,
        image_width: usize,
        image_height: usize,
    ) -> CGPoint;
}

extern "C-unwind" {
    pub fn VNNormalizedPointForImagePointUsingRegionOfInterest(
        image_point: CGPoint,
        image_width: usize,
        image_height: usize,
        roi: CGRect,
    ) -> CGPoint;
}

extern "C-unwind" {
    pub fn VNImageRectForNormalizedRect(
        normalized_rect: CGRect,
        image_width: usize,
        image_height: usize,
    ) -> CGRect;
}

extern "C-unwind" {
    pub fn VNImageRectForNormalizedRectUsingRegionOfInterest(
        normalized_rect: CGRect,
        image_width: usize,
        image_height: usize,
        roi: CGRect,
    ) -> CGRect;
}

extern "C-unwind" {
    pub fn VNNormalizedRectForImageRect(
        image_rect: CGRect,
        image_width: usize,
        image_height: usize,
    ) -> CGRect;
}

extern "C-unwind" {
    pub fn VNNormalizedRectForImageRectUsingRegionOfInterest(
        image_rect: CGRect,
        image_width: usize,
        image_height: usize,
        roi: CGRect,
    ) -> CGRect;
}

extern "C-unwind" {
    #[cfg(feature = "VNTypes")]
    pub fn VNElementTypeSize(element_type: VNElementType) -> NSUInteger;
}
