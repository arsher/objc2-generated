//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avkit/avcaptureviewcontrolsstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptureViewControlsStyle(pub NSInteger);
impl AVCaptureViewControlsStyle {
    #[doc(alias = "AVCaptureViewControlsStyleInline")]
    pub const Inline: Self = Self(0);
    #[doc(alias = "AVCaptureViewControlsStyleFloating")]
    pub const Floating: Self = Self(1);
    #[doc(alias = "AVCaptureViewControlsStyleInlineDeviceSelection")]
    pub const InlineDeviceSelection: Self = Self(2);
    #[doc(alias = "AVCaptureViewControlsStyleDefault")]
    pub const Default: Self = Self(AVCaptureViewControlsStyle::Inline.0);
}

unsafe impl Encode for AVCaptureViewControlsStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptureViewControlsStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avkit/avcaptureview?language=objc)
    #[unsafe(super(NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct AVCaptureView;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for AVCaptureView {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for AVCaptureView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl AVCaptureView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVCaptureViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVCaptureViewDelegate>>,
        );

        #[method(controlsStyle)]
        pub unsafe fn controlsStyle(&self) -> AVCaptureViewControlsStyle;

        #[method(setControlsStyle:)]
        pub unsafe fn setControlsStyle(&self, controls_style: AVCaptureViewControlsStyle);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl AVCaptureView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl AVCaptureView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl AVCaptureView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avkit/avcaptureviewdelegate?language=objc)
    pub unsafe trait AVCaptureViewDelegate: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn AVCaptureViewDelegate {}
);
