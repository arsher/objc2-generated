//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnconfidence?language=objc)
pub type VNConfidence = c_float;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnaspectratio?language=objc)
pub type VNAspectRatio = c_float;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndegrees?language=objc)
pub type VNDegrees = c_float;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnimagecropandscaleoption?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNImageCropAndScaleOption(pub NSUInteger);
impl VNImageCropAndScaleOption {
    #[doc(alias = "VNImageCropAndScaleOptionCenterCrop")]
    pub const CenterCrop: Self = Self(0);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFit")]
    pub const ScaleFit: Self = Self(1);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFill")]
    pub const ScaleFill: Self = Self(2);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFitRotate90CCW")]
    pub const ScaleFitRotate90CCW: Self = Self(0x100 + VNImageCropAndScaleOption::ScaleFit.0);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFillRotate90CCW")]
    pub const ScaleFillRotate90CCW: Self = Self(0x100 + VNImageCropAndScaleOption::ScaleFill.0);
}

unsafe impl Encode for VNImageCropAndScaleOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNImageCropAndScaleOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vncomputestage?language=objc)
// NS_TYPED_ENUM
pub type VNComputeStage = NSString;

extern "C" {
    /// The stage of a request where the main functionality is being performed.
    ///
    /// All requests will have this compute stage.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vncomputestagemain?language=objc)
    pub static VNComputeStageMain: Option<&'static VNComputeStage>;
}

extern "C" {
    /// A compute stage for additional analysis and/or conversion of the data produced by the `VNComputeStageMain`.
    ///
    /// This is an optional compute stage that some requests may expose.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vncomputestagepostprocessing?language=objc)
    pub static VNComputeStagePostProcessing: Option<&'static VNComputeStage>;
}

/// Barcode symbologies that are supported by the Vision framework.
///
///
/// The actual set of barcode symbologies that can actually be recognized by a specific version of the Vision framework should be determined by using the VNRequestNameSupportedBarcodeSymbologies request.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbology?language=objc)
// NS_TYPED_ENUM
pub type VNBarcodeSymbology = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyaztec?language=objc)
    pub static VNBarcodeSymbologyAztec: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode39?language=objc)
    pub static VNBarcodeSymbologyCode39: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode39checksum?language=objc)
    pub static VNBarcodeSymbologyCode39Checksum: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode39fullascii?language=objc)
    pub static VNBarcodeSymbologyCode39FullASCII: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode39fullasciichecksum?language=objc)
    pub static VNBarcodeSymbologyCode39FullASCIIChecksum: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode93?language=objc)
    pub static VNBarcodeSymbologyCode93: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode93i?language=objc)
    pub static VNBarcodeSymbologyCode93i: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycode128?language=objc)
    pub static VNBarcodeSymbologyCode128: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologydatamatrix?language=objc)
    pub static VNBarcodeSymbologyDataMatrix: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyean8?language=objc)
    pub static VNBarcodeSymbologyEAN8: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyean13?language=objc)
    pub static VNBarcodeSymbologyEAN13: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyi2of5?language=objc)
    pub static VNBarcodeSymbologyI2of5: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyi2of5checksum?language=objc)
    pub static VNBarcodeSymbologyI2of5Checksum: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyitf14?language=objc)
    pub static VNBarcodeSymbologyITF14: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologypdf417?language=objc)
    pub static VNBarcodeSymbologyPDF417: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyqr?language=objc)
    pub static VNBarcodeSymbologyQR: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologyupce?language=objc)
    pub static VNBarcodeSymbologyUPCE: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologycodabar?language=objc)
    pub static VNBarcodeSymbologyCodabar: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologygs1databar?language=objc)
    pub static VNBarcodeSymbologyGS1DataBar: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologygs1databarexpanded?language=objc)
    pub static VNBarcodeSymbologyGS1DataBarExpanded: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologygs1databarlimited?language=objc)
    pub static VNBarcodeSymbologyGS1DataBarLimited: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologymicropdf417?language=objc)
    pub static VNBarcodeSymbologyMicroPDF417: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologymicroqr?language=objc)
    pub static VNBarcodeSymbologyMicroQR: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodesymbologymsiplessey?language=objc)
    pub static VNBarcodeSymbologyMSIPlessey: Option<&'static VNBarcodeSymbology>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnelementtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNElementType(pub NSUInteger);
impl VNElementType {
    #[doc(alias = "VNElementTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "VNElementTypeFloat")]
    pub const Float: Self = Self(1);
    #[doc(alias = "VNElementTypeDouble")]
    pub const Double: Self = Self(2);
}

unsafe impl Encode for VNElementType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNElementType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnvideoprocessingoption?language=objc)
// NS_TYPED_ENUM
pub type VNVideoProcessingOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnvideoprocessingoptionframecadence?language=objc)
    pub static VNVideoProcessingOptionFrameCadence: Option<&'static VNVideoProcessingOption>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnvideoprocessingoptiontimeinterval?language=objc)
    pub static VNVideoProcessingOptionTimeInterval: Option<&'static VNVideoProcessingOption>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnchirality?language=objc)
// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VNChirality {
    #[doc(alias = "VNChiralityUnknown")]
    Unknown = 0,
    #[doc(alias = "VNChiralityLeft")]
    Left = -1,
    #[doc(alias = "VNChiralityRight")]
    Right = 1,
}

unsafe impl Encode for VNChirality {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNChirality {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnpointsclassification?language=objc)
// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VNPointsClassification {
    #[doc(alias = "VNPointsClassificationDisconnected")]
    Disconnected = 0,
    #[doc(alias = "VNPointsClassificationOpenPath")]
    OpenPath = 1,
    #[doc(alias = "VNPointsClassificationClosedPath")]
    ClosedPath = 2,
}

unsafe impl Encode for VNPointsClassification {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNPointsClassification {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnbarcodecompositetype?language=objc)
// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VNBarcodeCompositeType {
    #[doc(alias = "VNBarcodeCompositeTypeNone")]
    None = 0,
    #[doc(alias = "VNBarcodeCompositeTypeLinked")]
    Linked = 1,
    #[doc(alias = "VNBarcodeCompositeTypeGS1TypeA")]
    GS1TypeA = 2,
    #[doc(alias = "VNBarcodeCompositeTypeGS1TypeB")]
    GS1TypeB = 3,
    #[doc(alias = "VNBarcodeCompositeTypeGS1TypeC")]
    GS1TypeC = 4,
}

unsafe impl Encode for VNBarcodeCompositeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNBarcodeCompositeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrecognizedpointkey?language=objc)
// NS_TYPED_ENUM
pub type VNRecognizedPointKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnrecognizedpointgroupkey?language=objc)
// NS_TYPED_ENUM
pub type VNRecognizedPointGroupKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointname?language=objc)
// NS_TYPED_ENUM
pub type VNAnimalBodyPoseObservationJointName = VNRecognizedPointKey;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamelefteartop?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftEarTop:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerighteartop?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightEarTop:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftearmiddle?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftEarMiddle:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightearmiddle?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightEarMiddle:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftearbottom?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftEarBottom:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightearbottom?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightEarBottom:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamelefteye?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftEye:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerighteye?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightEye:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamenose?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameNose:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameneck?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameNeck:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftfrontelbow?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightfrontelbow?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightFrontElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftfrontknee?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightfrontknee?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightFrontKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftfrontpaw?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightfrontpaw?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightFrontPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftbackelbow?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftBackElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightbackelbow?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightBackElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftbackknee?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftBackKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightbackknee?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightBackKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnameleftbackpaw?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameLeftBackPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnamerightbackpaw?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameRightBackPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnametailtop?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameTailTop:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnametailmiddle?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameTailMiddle:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointnametailbottom?language=objc)
    pub static VNAnimalBodyPoseObservationJointNameTailBottom:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupname?language=objc)
// NS_TYPED_ENUM
pub type VNAnimalBodyPoseObservationJointsGroupName = VNRecognizedPointGroupKey;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupnamehead?language=objc)
    pub static VNAnimalBodyPoseObservationJointsGroupNameHead:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupnametrunk?language=objc)
    pub static VNAnimalBodyPoseObservationJointsGroupNameTrunk:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupnameforelegs?language=objc)
    pub static VNAnimalBodyPoseObservationJointsGroupNameForelegs:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupnamehindlegs?language=objc)
    pub static VNAnimalBodyPoseObservationJointsGroupNameHindlegs:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupnametail?language=objc)
    pub static VNAnimalBodyPoseObservationJointsGroupNameTail:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnanimalbodyposeobservationjointsgroupnameall?language=objc)
    pub static VNAnimalBodyPoseObservationJointsGroupNameAll:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

/// Human Body 3D Pose Joints that are suppported by Vision framework
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointname?language=objc)
// NS_TYPED_ENUM
pub type VNHumanBodyPose3DObservationJointName = VNRecognizedPointKey;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnameroot?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRoot:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamerighthip?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRightHip:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamerightknee?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRightKnee:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamerightankle?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRightAnkle:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamelefthip?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameLeftHip:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnameleftknee?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameLeftKnee:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnameleftankle?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameLeftAnkle:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamespine?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameSpine:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamecentershoulder?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameCenterShoulder:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamecenterhead?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameCenterHead:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnametophead?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameTopHead:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnameleftshoulder?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameLeftShoulder:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnameleftelbow?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameLeftElbow:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnameleftwrist?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameLeftWrist:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamerightshoulder?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRightShoulder:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamerightelbow?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRightElbow:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointnamerightwrist?language=objc)
    pub static VNHumanBodyPose3DObservationJointNameRightWrist:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupname?language=objc)
// NS_TYPED_ENUM
pub type VNHumanBodyPose3DObservationJointsGroupName = VNRecognizedPointGroupKey;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnamehead?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameHead:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnametorso?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameTorso:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnameleftarm?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameLeftArm:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnamerightarm?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameRightArm:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnameleftleg?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameLeftLeg:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnamerightleg?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameRightLeg:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/vision/vnhumanbodypose3dobservationjointsgroupnameall?language=objc)
    pub static VNHumanBodyPose3DObservationJointsGroupNameAll:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}
