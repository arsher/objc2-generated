//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSPDFPanelOptions {
        NSPDFPanelShowsPaperSize = 1 << 2,
        NSPDFPanelShowsOrientation = 1 << 3,
        NSPDFPanelRequestsParentDirectory = 1 << 24,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPDFPanel")]
    pub struct NSPDFPanel;

    #[cfg(feature = "AppKit_NSPDFPanel")]
    unsafe impl ClassType for NSPDFPanel {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPDFPanel")]
unsafe impl NSObjectProtocol for NSPDFPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPDFPanel")]
    unsafe impl NSPDFPanel {
        #[method_id(@__retain_semantics Other panel)]
        pub unsafe fn panel() -> Id<NSPDFPanel, Shared>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other accessoryController)]
        pub unsafe fn accessoryController(&self) -> Option<Id<NSViewController, Shared>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setAccessoryController:)]
        pub unsafe fn setAccessoryController(
            &self,
            accessory_controller: Option<&NSViewController>,
        );

        #[method(options)]
        pub unsafe fn options(&self) -> NSPDFPanelOptions;

        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPDFPanelOptions);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultFileName)]
        pub unsafe fn defaultFileName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDefaultFileName:)]
        pub unsafe fn setDefaultFileName(&self, default_file_name: &NSString);

        #[cfg(all(feature = "AppKit_NSPDFInfo", feature = "AppKit_NSWindow"))]
        #[method(beginSheetWithPDFInfo:modalForWindow:completionHandler:)]
        pub unsafe fn beginSheetWithPDFInfo_modalForWindow_completionHandler(
            &self,
            pdf_info: &NSPDFInfo,
            doc_window: Option<&NSWindow>,
            completion_handler: &Block<(NSInteger,), ()>,
        );
    }
);
