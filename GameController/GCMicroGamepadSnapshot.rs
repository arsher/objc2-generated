//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub struct GCMicroGamepadSnapshot;

    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl ClassType for GCMicroGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCMicroGamepad;
    }
);

#[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
unsafe impl NSObjectProtocol for GCMicroGamepadSnapshot {}

extern_methods!(
    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCMicroGamepadSnapshot {
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method_id(@__retain_semantics Other snapshotData)]
        pub unsafe fn snapshotData(&self) -> Retained<NSData>;

        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method(setSnapshotData:)]
        pub unsafe fn setSnapshotData(&self, snapshot_data: &NSData);

        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithSnapshotData:)]
        pub unsafe fn initWithSnapshotData(this: Allocated<Self>, data: &NSData) -> Retained<Self>;

        #[cfg(feature = "GCController")]
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithController:snapshotData:)]
        pub unsafe fn initWithController_snapshotData(
            this: Allocated<Self>,
            controller: &GCController,
            data: &NSData,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GCMicroGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCMicroGamepadSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

// NS_ENUM
#[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCMicroGamepadSnapshotDataVersion(pub NSInteger);
impl GCMicroGamepadSnapshotDataVersion {
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub const GCMicroGamepadSnapshotDataVersion1: Self = Self(0x0100);
}

unsafe impl Encode for GCMicroGamepadSnapshotDataVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCMicroGamepadSnapshotDataVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static GCCurrentMicroGamepadSnapshotDataVersion: GCMicroGamepadSnapshotDataVersion;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCMicroGamepadSnapshotData {
    pub version: u16,
    pub size: u16,
    pub dpadX: c_float,
    pub dpadY: c_float,
    pub buttonA: c_float,
    pub buttonX: c_float,
}

unsafe impl Encode for GCMicroGamepadSnapshotData {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u16>::ENCODING,
            <u16>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCMicroGamepadSnapshotData {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub fn GCMicroGamepadSnapshotDataFromNSData(
        snapshot_data: *mut GCMicroGamepadSnapshotData,
        data: Option<&NSData>,
    ) -> Bool;
}

extern "C-unwind" {
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub fn NSDataFromGCMicroGamepadSnapshotData(
        snapshot_data: *mut GCMicroGamepadSnapshotData,
    ) -> *mut NSData;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCMicroGamepadSnapShotDataV100 {
    pub version: u16,
    pub size: u16,
    pub dpadX: c_float,
    pub dpadY: c_float,
    pub buttonA: c_float,
    pub buttonX: c_float,
}

unsafe impl Encode for GCMicroGamepadSnapShotDataV100 {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u16>::ENCODING,
            <u16>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCMicroGamepadSnapShotDataV100 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub fn GCMicroGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCMicroGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
}

extern "C-unwind" {
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub fn NSDataFromGCMicroGamepadSnapShotDataV100(
        snapshot_data: *mut GCMicroGamepadSnapShotDataV100,
    ) -> *mut NSData;
}
