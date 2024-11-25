//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrordomain?language=objc)
    pub static WebKitErrorDomain: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrormimetypekey?language=objc)
    pub static WebKitErrorMIMETypeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorpluginnamekey?language=objc)
    pub static WebKitErrorPlugInNameKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorpluginpageurlstringkey?language=objc)
    pub static WebKitErrorPlugInPageURLStringKey: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorcannotshowmimetype?language=objc)
#[deprecated]
pub const WebKitErrorCannotShowMIMEType: c_uint = 100;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorcannotshowurl?language=objc)
#[deprecated]
pub const WebKitErrorCannotShowURL: c_uint = 101;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorframeloadinterruptedbypolicychange?language=objc)
#[deprecated]
pub const WebKitErrorFrameLoadInterruptedByPolicyChange: c_uint = 102;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorcannotfindplugin?language=objc)
#[deprecated]
pub const WebKitErrorCannotFindPlugIn: c_uint = 200;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorcannotloadplugin?language=objc)
#[deprecated]
pub const WebKitErrorCannotLoadPlugIn: c_uint = 201;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorjavaunavailable?language=objc)
#[deprecated]
pub const WebKitErrorJavaUnavailable: c_uint = 202;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/webkiterrorblockedpluginversion?language=objc)
#[deprecated]
pub const WebKitErrorBlockedPlugInVersion: c_uint = 203;
