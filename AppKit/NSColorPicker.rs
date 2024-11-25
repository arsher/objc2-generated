//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorpicker?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorPicker;
);

#[cfg(feature = "NSColorPicking")]
unsafe impl NSColorPickingDefault for NSColorPicker {}

unsafe impl NSObjectProtocol for NSColorPicker {}

extern_methods!(
    unsafe impl NSColorPicker {
        #[cfg(all(
            feature = "NSColorPanel",
            feature = "NSPanel",
            feature = "NSResponder",
            feature = "NSWindow"
        ))]
        #[method_id(@__retain_semantics Init initWithPickerMask:colorPanel:)]
        pub unsafe fn initWithPickerMask_colorPanel(
            this: Allocated<Self>,
            mask: NSUInteger,
            owning_color_panel: &NSColorPanel,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "NSColorPanel",
            feature = "NSPanel",
            feature = "NSResponder",
            feature = "NSWindow"
        ))]
        #[method_id(@__retain_semantics Other colorPanel)]
        pub unsafe fn colorPanel(&self) -> Retained<NSColorPanel>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other provideNewButtonImage)]
        pub unsafe fn provideNewButtonImage(&self) -> Retained<NSImage>;

        #[cfg(all(
            feature = "NSActionCell",
            feature = "NSButtonCell",
            feature = "NSCell",
            feature = "NSImage"
        ))]
        #[method(insertNewButtonImage:in:)]
        pub unsafe fn insertNewButtonImage_in(
            &self,
            new_button_image: &NSImage,
            button_cell: &NSButtonCell,
        );

        #[method(viewSizeChanged:)]
        pub unsafe fn viewSizeChanged(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSColorList")]
        #[method(attachColorList:)]
        pub unsafe fn attachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "NSColorList")]
        #[method(detachColorList:)]
        pub unsafe fn detachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "NSColorPanel")]
        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[method_id(@__retain_semantics Other buttonToolTip)]
        pub unsafe fn buttonToolTip(&self) -> Retained<NSString>;

        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSColorPicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
