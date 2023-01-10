//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPaperOrientation {
        NSPaperOrientationPortrait = 0,
        NSPaperOrientationLandscape = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPrintingPaginationMode {
        NSPrintingPaginationModeAutomatic = 0,
        NSPrintingPaginationModeFit = 1,
        NSPrintingPaginationModeClip = 2,
    }
);

typed_extensible_enum!(
    pub type NSPrintInfoAttributeKey = Foundation::NSString;
);

extern_static!(NSPrintPaperName: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintPaperSize: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintOrientation: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintScalingFactor: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintLeftMargin: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintRightMargin: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintTopMargin: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintBottomMargin: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintHorizontallyCentered: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintVerticallyCentered: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintHorizontalPagination: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintVerticalPagination: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintPrinter: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintCopies: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintAllPages: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintFirstPage: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintLastPage: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintMustCollate: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintReversePageOrder: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintJobDisposition: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintPagesAcross: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintPagesDown: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintTime: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintDetailedErrorReporting: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintFaxNumber: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintPrinterName: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintSelectionOnly: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintJobSavingURL: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintJobSavingFileNameExtensionHidden: &'static AppKit::NSPrintInfoAttributeKey);

extern_static!(NSPrintHeaderAndFooter: &'static AppKit::NSPrintInfoAttributeKey);

typed_enum!(
    pub type NSPrintJobDispositionValue = Foundation::NSString;
);

extern_static!(NSPrintSpoolJob: &'static AppKit::NSPrintJobDispositionValue);

extern_static!(NSPrintPreviewJob: &'static AppKit::NSPrintJobDispositionValue);

extern_static!(NSPrintSaveJob: &'static AppKit::NSPrintJobDispositionValue);

extern_static!(NSPrintCancelJob: &'static AppKit::NSPrintJobDispositionValue);

pub type NSPrintInfoSettingKey = Foundation::NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPrintInfo;

    unsafe impl ClassType for NSPrintInfo {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPrintInfo")]
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics Other sharedPrintInfo)]
        pub unsafe fn sharedPrintInfo() -> Id<AppKit::NSPrintInfo, Shared>;

        #[method(setSharedPrintInfo:)]
        pub unsafe fn setSharedPrintInfo(sharedPrintInfo: &AppKit::NSPrintInfo);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Option<Allocated<Self>>,
            attributes: &Foundation::NSDictionary<AppKit::NSPrintInfoAttributeKey, Object>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary(
            &self,
        ) -> Id<Foundation::NSMutableDictionary<AppKit::NSPrintInfoAttributeKey, Object>, Owned>;

        #[method_id(@__retain_semantics Other paperName)]
        pub unsafe fn paperName(&self) -> Option<Id<AppKit::NSPrinterPaperName, Shared>>;

        #[method(setPaperName:)]
        pub unsafe fn setPaperName(&self, paperName: Option<&AppKit::NSPrinterPaperName>);

        #[method(paperSize)]
        pub unsafe fn paperSize(&self) -> NSSize;

        #[method(setPaperSize:)]
        pub unsafe fn setPaperSize(&self, paperSize: NSSize);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSPaperOrientation;

        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);

        #[method(scalingFactor)]
        pub unsafe fn scalingFactor(&self) -> CGFloat;

        #[method(setScalingFactor:)]
        pub unsafe fn setScalingFactor(&self, scalingFactor: CGFloat);

        #[method(leftMargin)]
        pub unsafe fn leftMargin(&self) -> CGFloat;

        #[method(setLeftMargin:)]
        pub unsafe fn setLeftMargin(&self, leftMargin: CGFloat);

        #[method(rightMargin)]
        pub unsafe fn rightMargin(&self) -> CGFloat;

        #[method(setRightMargin:)]
        pub unsafe fn setRightMargin(&self, rightMargin: CGFloat);

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(setTopMargin:)]
        pub unsafe fn setTopMargin(&self, topMargin: CGFloat);

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[method(setBottomMargin:)]
        pub unsafe fn setBottomMargin(&self, bottomMargin: CGFloat);

        #[method(isHorizontallyCentered)]
        pub unsafe fn isHorizontallyCentered(&self) -> bool;

        #[method(setHorizontallyCentered:)]
        pub unsafe fn setHorizontallyCentered(&self, horizontallyCentered: bool);

        #[method(isVerticallyCentered)]
        pub unsafe fn isVerticallyCentered(&self) -> bool;

        #[method(setVerticallyCentered:)]
        pub unsafe fn setVerticallyCentered(&self, verticallyCentered: bool);

        #[method(horizontalPagination)]
        pub unsafe fn horizontalPagination(&self) -> NSPrintingPaginationMode;

        #[method(setHorizontalPagination:)]
        pub unsafe fn setHorizontalPagination(
            &self,
            horizontalPagination: NSPrintingPaginationMode,
        );

        #[method(verticalPagination)]
        pub unsafe fn verticalPagination(&self) -> NSPrintingPaginationMode;

        #[method(setVerticalPagination:)]
        pub unsafe fn setVerticalPagination(&self, verticalPagination: NSPrintingPaginationMode);

        #[method_id(@__retain_semantics Other jobDisposition)]
        pub unsafe fn jobDisposition(&self) -> Id<AppKit::NSPrintJobDispositionValue, Shared>;

        #[method(setJobDisposition:)]
        pub unsafe fn setJobDisposition(&self, jobDisposition: &AppKit::NSPrintJobDispositionValue);

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other printer)]
        pub unsafe fn printer(&self) -> Id<AppKit::NSPrinter, Shared>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method(setPrinter:)]
        pub unsafe fn setPrinter(&self, printer: &AppKit::NSPrinter);

        #[method(setUpPrintOperationDefaultValues)]
        pub unsafe fn setUpPrintOperationDefaultValues(&self);

        #[method(imageablePageBounds)]
        pub unsafe fn imageablePageBounds(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedPaperName)]
        pub unsafe fn localizedPaperName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other defaultPrinter)]
        pub unsafe fn defaultPrinter() -> Option<Id<AppKit::NSPrinter, Shared>>;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other printSettings)]
        pub unsafe fn printSettings(
            &self,
        ) -> Id<Foundation::NSMutableDictionary<AppKit::NSPrintInfoSettingKey, Object>, Owned>;

        #[method(PMPrintSession)]
        pub unsafe fn PMPrintSession(&self) -> NonNull<c_void>;

        #[method(PMPageFormat)]
        pub unsafe fn PMPageFormat(&self) -> NonNull<c_void>;

        #[method(PMPrintSettings)]
        pub unsafe fn PMPrintSettings(&self) -> NonNull<c_void>;

        #[method(updateFromPMPageFormat)]
        pub unsafe fn updateFromPMPageFormat(&self);

        #[method(updateFromPMPrintSettings)]
        pub unsafe fn updateFromPMPrintSettings(&self);

        #[method(isSelectionOnly)]
        pub unsafe fn isSelectionOnly(&self) -> bool;

        #[method(setSelectionOnly:)]
        pub unsafe fn setSelectionOnly(&self, selectionOnly: bool);

        #[cfg(feature = "AppKit_NSPDFInfo")]
        #[method(takeSettingsFromPDFInfo:)]
        pub unsafe fn takeSettingsFromPDFInfo(&self, inPDFInfo: &AppKit::NSPDFInfo);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSPrintInfo")]
    unsafe impl NSPrintInfo {
        #[cfg(feature = "AppKit_NSPrinter")]
        #[method(setDefaultPrinter:)]
        pub unsafe fn setDefaultPrinter(printer: Option<&AppKit::NSPrinter>);

        #[method(sizeForPaperName:)]
        pub unsafe fn sizeForPaperName(name: Option<&AppKit::NSPrinterPaperName>) -> NSSize;
    }
);

extern_static!(NSPrintFormName: &'static Foundation::NSString);

extern_static!(NSPrintJobFeatures: &'static Foundation::NSString);

extern_static!(NSPrintManualFeed: &'static Foundation::NSString);

extern_static!(NSPrintPagesPerSheet: &'static Foundation::NSString);

extern_static!(NSPrintPaperFeed: &'static Foundation::NSString);

extern_static!(NSPrintSavePath: &'static Foundation::NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPrintingOrientation {
        NSPortraitOrientation = 0,
        NSLandscapeOrientation = 1,
    }
);

extern_static!(NSAutoPagination: NSPrintingPaginationMode = NSPrintingPaginationModeAutomatic);

extern_static!(NSFitPagination: NSPrintingPaginationMode = NSPrintingPaginationModeFit);

extern_static!(NSClipPagination: NSPrintingPaginationMode = NSPrintingPaginationModeClip);
