//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciplugin?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIPlugIn;
);

unsafe impl NSObjectProtocol for CIPlugIn {}

extern_methods!(
    unsafe impl CIPlugIn {
        #[deprecated]
        #[method(loadAllPlugIns)]
        pub unsafe fn loadAllPlugIns();

        #[method(loadNonExecutablePlugIns)]
        pub unsafe fn loadNonExecutablePlugIns();

        #[deprecated]
        #[method(loadPlugIn:allowNonExecutable:)]
        pub unsafe fn loadPlugIn_allowNonExecutable(
            url: Option<&NSURL>,
            allow_non_executable: bool,
        );

        #[deprecated]
        #[method(loadPlugIn:allowExecutableCode:)]
        pub unsafe fn loadPlugIn_allowExecutableCode(
            url: Option<&NSURL>,
            allow_executable_code: bool,
        );

        #[method(loadNonExecutablePlugIn:)]
        pub unsafe fn loadNonExecutablePlugIn(url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIPlugIn {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
