//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorpickingdefault?language=objc)
    pub unsafe trait NSColorPickingDefault: MainThreadOnly {
        #[cfg(all(
            feature = "NSColorPanel",
            feature = "NSPanel",
            feature = "NSResponder",
            feature = "NSWindow"
        ))]
        #[method_id(@__retain_semantics Init initWithPickerMask:colorPanel:)]
        unsafe fn initWithPickerMask_colorPanel(
            this: Allocated<Self>,
            mask: NSUInteger,
            owning_color_panel: &NSColorPanel,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other provideNewButtonImage)]
        unsafe fn provideNewButtonImage(&self) -> Retained<NSImage>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSButtonCell",
            feature = "NSCell",
            feature = "NSImage"
        ))]
        #[method(insertNewButtonImage:in:)]
        unsafe fn insertNewButtonImage_in(
            &self,
            new_button_image: &NSImage,
            button_cell: &NSButtonCell,
        );

        #[method(viewSizeChanged:)]
        unsafe fn viewSizeChanged(&self, sender: Option<&AnyObject>);

        #[method(alphaControlAddedOrRemoved:)]
        unsafe fn alphaControlAddedOrRemoved(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSColorList")]
        #[method(attachColorList:)]
        unsafe fn attachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "NSColorList")]
        #[method(detachColorList:)]
        unsafe fn detachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "NSColorPanel")]
        #[method(setMode:)]
        unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[method_id(@__retain_semantics Other buttonToolTip)]
        unsafe fn buttonToolTip(&self) -> Retained<NSString>;

        #[method(minContentSize)]
        unsafe fn minContentSize(&self) -> NSSize;
    }

    unsafe impl ProtocolType for dyn NSColorPickingDefault {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorpickingcustom?language=objc)
    pub unsafe trait NSColorPickingCustom: NSColorPickingDefault + MainThreadOnly {
        #[cfg(feature = "NSColorPanel")]
        #[method(supportsMode:)]
        unsafe fn supportsMode(&self, mode: NSColorPanelMode) -> bool;

        #[cfg(feature = "NSColorPanel")]
        #[method(currentMode)]
        unsafe fn currentMode(&self) -> NSColorPanelMode;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other provideNewView:)]
        unsafe fn provideNewView(&self, initial_request: bool) -> Retained<NSView>;

        #[cfg(feature = "NSColor")]
        #[method(setColor:)]
        unsafe fn setColor(&self, new_color: &NSColor);
    }

    unsafe impl ProtocolType for dyn NSColorPickingCustom {}
);
