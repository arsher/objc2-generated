//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCGamepad", feature = "GCPhysicalInputProfile"))]
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub struct GCGamepadSnapshot;

    #[cfg(all(feature = "GCGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl ClassType for GCGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCGamepad;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "GCGamepad", feature = "GCPhysicalInputProfile"))]
unsafe impl NSObjectProtocol for GCGamepadSnapshot {}

extern_methods!(
    #[cfg(all(feature = "GCGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCGamepadSnapshot {
        #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
        #[method_id(@__retain_semantics Other snapshotData)]
        pub unsafe fn snapshotData(&self) -> Retained<NSData>;

        #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
        #[method(setSnapshotData:)]
        pub unsafe fn setSnapshotData(&self, snapshot_data: &NSData);

        #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
        #[method_id(@__retain_semantics Init initWithSnapshotData:)]
        pub unsafe fn initWithSnapshotData(this: Allocated<Self>, data: &NSData) -> Retained<Self>;

        #[cfg(feature = "GCController")]
        #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
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
    #[cfg(all(feature = "GCGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCGamepadSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCGamepadSnapShotDataV100 {
    pub version: u16,
    pub size: u16,
    pub dpadX: c_float,
    pub dpadY: c_float,
    pub buttonA: c_float,
    pub buttonB: c_float,
    pub buttonX: c_float,
    pub buttonY: c_float,
    pub leftShoulder: c_float,
    pub rightShoulder: c_float,
}

unsafe impl Encode for GCGamepadSnapShotDataV100 {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u16>::ENCODING,
            <u16>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCGamepadSnapShotDataV100 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub fn GCGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
}

extern "C" {
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub fn NSDataFromGCGamepadSnapShotDataV100(
        snapshot_data: *mut GCGamepadSnapShotDataV100,
    ) -> *mut NSData;
}
