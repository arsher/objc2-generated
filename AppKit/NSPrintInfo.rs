//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

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
    pub type NSPrintInfoAttributeKey = NSString;
);

extern_static!(NSPrintPaperName: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintPaperSize: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintOrientation: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintScalingFactor: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintLeftMargin: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintRightMargin: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintTopMargin: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintBottomMargin: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintHorizontallyCentered: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintVerticallyCentered: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintHorizontalPagination: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintVerticalPagination: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintPrinter: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintCopies: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintAllPages: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintFirstPage: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintLastPage: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintMustCollate: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintReversePageOrder: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintJobDisposition: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintPagesAcross: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintPagesDown: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintTime: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintDetailedErrorReporting: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintFaxNumber: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintPrinterName: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintSelectionOnly: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintJobSavingURL: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintJobSavingFileNameExtensionHidden: &'static NSPrintInfoAttributeKey);

extern_static!(NSPrintHeaderAndFooter: &'static NSPrintInfoAttributeKey);

typed_enum!(
    pub type NSPrintJobDispositionValue = NSString;
);

extern_static!(NSPrintSpoolJob: &'static NSPrintJobDispositionValue);

extern_static!(NSPrintPreviewJob: &'static NSPrintJobDispositionValue);

extern_static!(NSPrintSaveJob: &'static NSPrintJobDispositionValue);

extern_static!(NSPrintCancelJob: &'static NSPrintJobDispositionValue);

pub type NSPrintInfoSettingKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPrintInfo")]
    pub struct NSPrintInfo;

    #[cfg(feature = "AppKit_NSPrintInfo")]
    unsafe impl ClassType for NSPrintInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSPrintInfo")]
unsafe impl NSCoding for NSPrintInfo {}

#[cfg(feature = "AppKit_NSPrintInfo")]
unsafe impl NSCopying for NSPrintInfo {}

#[cfg(feature = "AppKit_NSPrintInfo")]
unsafe impl NSObjectProtocol for NSPrintInfo {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPrintInfo")]
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics Other sharedPrintInfo)]
        pub unsafe fn sharedPrintInfo() -> Id<NSPrintInfo>;

        #[method(setSharedPrintInfo:)]
        pub unsafe fn setSharedPrintInfo(shared_print_info: &NSPrintInfo);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Option<Allocated<Self>>,
            attributes: &NSDictionary<NSPrintInfoAttributeKey, Object>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary(&self)
            -> Id<NSMutableDictionary<NSPrintInfoAttributeKey, Object>>;

        #[method_id(@__retain_semantics Other paperName)]
        pub unsafe fn paperName(&self) -> Option<Id<NSPrinterPaperName>>;

        #[method(setPaperName:)]
        pub unsafe fn setPaperName(&self, paper_name: Option<&NSPrinterPaperName>);

        #[method(paperSize)]
        pub unsafe fn paperSize(&self) -> NSSize;

        #[method(setPaperSize:)]
        pub unsafe fn setPaperSize(&self, paper_size: NSSize);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSPaperOrientation;

        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);

        #[method(scalingFactor)]
        pub unsafe fn scalingFactor(&self) -> CGFloat;

        #[method(setScalingFactor:)]
        pub unsafe fn setScalingFactor(&self, scaling_factor: CGFloat);

        #[method(leftMargin)]
        pub unsafe fn leftMargin(&self) -> CGFloat;

        #[method(setLeftMargin:)]
        pub unsafe fn setLeftMargin(&self, left_margin: CGFloat);

        #[method(rightMargin)]
        pub unsafe fn rightMargin(&self) -> CGFloat;

        #[method(setRightMargin:)]
        pub unsafe fn setRightMargin(&self, right_margin: CGFloat);

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(setTopMargin:)]
        pub unsafe fn setTopMargin(&self, top_margin: CGFloat);

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[method(setBottomMargin:)]
        pub unsafe fn setBottomMargin(&self, bottom_margin: CGFloat);

        #[method(isHorizontallyCentered)]
        pub unsafe fn isHorizontallyCentered(&self) -> bool;

        #[method(setHorizontallyCentered:)]
        pub unsafe fn setHorizontallyCentered(&self, horizontally_centered: bool);

        #[method(isVerticallyCentered)]
        pub unsafe fn isVerticallyCentered(&self) -> bool;

        #[method(setVerticallyCentered:)]
        pub unsafe fn setVerticallyCentered(&self, vertically_centered: bool);

        #[method(horizontalPagination)]
        pub unsafe fn horizontalPagination(&self) -> NSPrintingPaginationMode;

        #[method(setHorizontalPagination:)]
        pub unsafe fn setHorizontalPagination(
            &self,
            horizontal_pagination: NSPrintingPaginationMode,
        );

        #[method(verticalPagination)]
        pub unsafe fn verticalPagination(&self) -> NSPrintingPaginationMode;

        #[method(setVerticalPagination:)]
        pub unsafe fn setVerticalPagination(&self, vertical_pagination: NSPrintingPaginationMode);

        #[method_id(@__retain_semantics Other jobDisposition)]
        pub unsafe fn jobDisposition(&self) -> Id<NSPrintJobDispositionValue>;

        #[method(setJobDisposition:)]
        pub unsafe fn setJobDisposition(&self, job_disposition: &NSPrintJobDispositionValue);

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other printer)]
        pub unsafe fn printer(&self) -> Id<NSPrinter>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method(setPrinter:)]
        pub unsafe fn setPrinter(&self, printer: &NSPrinter);

        #[method(setUpPrintOperationDefaultValues)]
        pub unsafe fn setUpPrintOperationDefaultValues(&self);

        #[method(imageablePageBounds)]
        pub unsafe fn imageablePageBounds(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedPaperName)]
        pub unsafe fn localizedPaperName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "AppKit_NSPrinter")]
        #[method_id(@__retain_semantics Other defaultPrinter)]
        pub unsafe fn defaultPrinter() -> Option<Id<NSPrinter>>;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other printSettings)]
        pub unsafe fn printSettings(
            &self,
        ) -> Id<NSMutableDictionary<NSPrintInfoSettingKey, Object>>;

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
        pub unsafe fn setSelectionOnly(&self, selection_only: bool);

        #[cfg(feature = "AppKit_NSPDFInfo")]
        #[method(takeSettingsFromPDFInfo:)]
        pub unsafe fn takeSettingsFromPDFInfo(&self, in_pdf_info: &NSPDFInfo);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPrintInfo")]
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSPrintInfo")]
    unsafe impl NSPrintInfo {
        #[cfg(feature = "AppKit_NSPrinter")]
        #[deprecated = "NSPrintInfo's implementation has no effect"]
        #[method(setDefaultPrinter:)]
        pub unsafe fn setDefaultPrinter(printer: Option<&NSPrinter>);

        #[deprecated = "Use -[NSPrinter pageSizeForPaper:] instead"]
        #[method(sizeForPaperName:)]
        pub unsafe fn sizeForPaperName(name: Option<&NSPrinterPaperName>) -> NSSize;
    }
);

extern_static!(NSPrintFormName: &'static NSString);

extern_static!(NSPrintJobFeatures: &'static NSString);

extern_static!(NSPrintManualFeed: &'static NSString);

extern_static!(NSPrintPagesPerSheet: &'static NSString);

extern_static!(NSPrintPaperFeed: &'static NSString);

extern_static!(NSPrintSavePath: &'static NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum NSPrintingOrientation {
        #[deprecated]
        NSPortraitOrientation = 0,
        #[deprecated]
        NSLandscapeOrientation = 1,
    }
);

extern_static!(NSAutoPagination: NSPrintingPaginationMode = NSPrintingPaginationModeAutomatic);

extern_static!(NSFitPagination: NSPrintingPaginationMode = NSPrintingPaginationModeFit);

extern_static!(NSClipPagination: NSPrintingPaginationMode = NSPrintingPaginationModeClip);
