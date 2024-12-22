//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspaperorientation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPaperOrientation(pub NSInteger);
impl NSPaperOrientation {
    #[doc(alias = "NSPaperOrientationPortrait")]
    pub const Portrait: Self = Self(0);
    #[doc(alias = "NSPaperOrientationLandscape")]
    pub const Landscape: Self = Self(1);
}

unsafe impl Encode for NSPaperOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPaperOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintingpaginationmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintingPaginationMode(pub NSUInteger);
impl NSPrintingPaginationMode {
    #[doc(alias = "NSPrintingPaginationModeAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSPrintingPaginationModeFit")]
    pub const Fit: Self = Self(1);
    #[doc(alias = "NSPrintingPaginationModeClip")]
    pub const Clip: Self = Self(2);
}

unsafe impl Encode for NSPrintingPaginationMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintingPaginationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintinfoattributekey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSPrintInfoAttributeKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpapername?language=objc)
    pub static NSPrintPaperName: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpapersize?language=objc)
    pub static NSPrintPaperSize: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintorientation?language=objc)
    pub static NSPrintOrientation: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintscalingfactor?language=objc)
    pub static NSPrintScalingFactor: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintleftmargin?language=objc)
    pub static NSPrintLeftMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintrightmargin?language=objc)
    pub static NSPrintRightMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprinttopmargin?language=objc)
    pub static NSPrintTopMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintbottommargin?language=objc)
    pub static NSPrintBottomMargin: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprinthorizontallycentered?language=objc)
    pub static NSPrintHorizontallyCentered: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintverticallycentered?language=objc)
    pub static NSPrintVerticallyCentered: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprinthorizontalpagination?language=objc)
    pub static NSPrintHorizontalPagination: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintverticalpagination?language=objc)
    pub static NSPrintVerticalPagination: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintprinter?language=objc)
    pub static NSPrintPrinter: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintcopies?language=objc)
    pub static NSPrintCopies: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintallpages?language=objc)
    pub static NSPrintAllPages: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintfirstpage?language=objc)
    pub static NSPrintFirstPage: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintlastpage?language=objc)
    pub static NSPrintLastPage: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintmustcollate?language=objc)
    pub static NSPrintMustCollate: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintreversepageorder?language=objc)
    pub static NSPrintReversePageOrder: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintjobdisposition?language=objc)
    pub static NSPrintJobDisposition: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpagesacross?language=objc)
    pub static NSPrintPagesAcross: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpagesdown?language=objc)
    pub static NSPrintPagesDown: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprinttime?language=objc)
    pub static NSPrintTime: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintdetailederrorreporting?language=objc)
    pub static NSPrintDetailedErrorReporting: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintfaxnumber?language=objc)
    pub static NSPrintFaxNumber: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintprintername?language=objc)
    pub static NSPrintPrinterName: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintselectiononly?language=objc)
    pub static NSPrintSelectionOnly: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintjobsavingurl?language=objc)
    pub static NSPrintJobSavingURL: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintjobsavingfilenameextensionhidden?language=objc)
    pub static NSPrintJobSavingFileNameExtensionHidden: &'static NSPrintInfoAttributeKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintheaderandfooter?language=objc)
    pub static NSPrintHeaderAndFooter: &'static NSPrintInfoAttributeKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintjobdispositionvalue?language=objc)
// NS_TYPED_ENUM
pub type NSPrintJobDispositionValue = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintspooljob?language=objc)
    pub static NSPrintSpoolJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpreviewjob?language=objc)
    pub static NSPrintPreviewJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintsavejob?language=objc)
    pub static NSPrintSaveJob: &'static NSPrintJobDispositionValue;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintcanceljob?language=objc)
    pub static NSPrintCancelJob: &'static NSPrintJobDispositionValue;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintinfosettingkey?language=objc)
pub type NSPrintInfoSettingKey = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintinfo?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPrintInfo;
);

unsafe impl NSCoding for NSPrintInfo {}

unsafe impl NSCopying for NSPrintInfo {}

unsafe impl CopyingHelper for NSPrintInfo {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSPrintInfo {}

extern_methods!(
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics Other sharedPrintInfo)]
        pub unsafe fn sharedPrintInfo() -> Retained<NSPrintInfo>;

        /// Setter for [`sharedPrintInfo`][Self::sharedPrintInfo].
        #[method(setSharedPrintInfo:)]
        pub unsafe fn setSharedPrintInfo(shared_print_info: &NSPrintInfo);

        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Allocated<Self>,
            attributes: &NSDictionary<NSPrintInfoAttributeKey, AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary(
            &self,
        ) -> Retained<NSMutableDictionary<NSPrintInfoAttributeKey, AnyObject>>;

        #[cfg(feature = "NSPrinter")]
        #[method_id(@__retain_semantics Other paperName)]
        pub unsafe fn paperName(&self) -> Option<Retained<NSPrinterPaperName>>;

        #[cfg(feature = "NSPrinter")]
        /// Setter for [`paperName`][Self::paperName].
        #[method(setPaperName:)]
        pub unsafe fn setPaperName(&self, paper_name: Option<&NSPrinterPaperName>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(paperSize)]
        pub unsafe fn paperSize(&self) -> NSSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`paperSize`][Self::paperSize].
        #[method(setPaperSize:)]
        pub unsafe fn setPaperSize(&self, paper_size: NSSize);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSPaperOrientation;

        /// Setter for [`orientation`][Self::orientation].
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSPaperOrientation);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(scalingFactor)]
        pub unsafe fn scalingFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`scalingFactor`][Self::scalingFactor].
        #[method(setScalingFactor:)]
        pub unsafe fn setScalingFactor(&self, scaling_factor: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(leftMargin)]
        pub unsafe fn leftMargin(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`leftMargin`][Self::leftMargin].
        #[method(setLeftMargin:)]
        pub unsafe fn setLeftMargin(&self, left_margin: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rightMargin)]
        pub unsafe fn rightMargin(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`rightMargin`][Self::rightMargin].
        #[method(setRightMargin:)]
        pub unsafe fn setRightMargin(&self, right_margin: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`topMargin`][Self::topMargin].
        #[method(setTopMargin:)]
        pub unsafe fn setTopMargin(&self, top_margin: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`bottomMargin`][Self::bottomMargin].
        #[method(setBottomMargin:)]
        pub unsafe fn setBottomMargin(&self, bottom_margin: CGFloat);

        #[method(isHorizontallyCentered)]
        pub unsafe fn isHorizontallyCentered(&self) -> bool;

        /// Setter for [`isHorizontallyCentered`][Self::isHorizontallyCentered].
        #[method(setHorizontallyCentered:)]
        pub unsafe fn setHorizontallyCentered(&self, horizontally_centered: bool);

        #[method(isVerticallyCentered)]
        pub unsafe fn isVerticallyCentered(&self) -> bool;

        /// Setter for [`isVerticallyCentered`][Self::isVerticallyCentered].
        #[method(setVerticallyCentered:)]
        pub unsafe fn setVerticallyCentered(&self, vertically_centered: bool);

        #[method(horizontalPagination)]
        pub unsafe fn horizontalPagination(&self) -> NSPrintingPaginationMode;

        /// Setter for [`horizontalPagination`][Self::horizontalPagination].
        #[method(setHorizontalPagination:)]
        pub unsafe fn setHorizontalPagination(
            &self,
            horizontal_pagination: NSPrintingPaginationMode,
        );

        #[method(verticalPagination)]
        pub unsafe fn verticalPagination(&self) -> NSPrintingPaginationMode;

        /// Setter for [`verticalPagination`][Self::verticalPagination].
        #[method(setVerticalPagination:)]
        pub unsafe fn setVerticalPagination(&self, vertical_pagination: NSPrintingPaginationMode);

        #[method_id(@__retain_semantics Other jobDisposition)]
        pub unsafe fn jobDisposition(&self) -> Retained<NSPrintJobDispositionValue>;

        /// Setter for [`jobDisposition`][Self::jobDisposition].
        #[method(setJobDisposition:)]
        pub unsafe fn setJobDisposition(&self, job_disposition: &NSPrintJobDispositionValue);

        #[cfg(feature = "NSPrinter")]
        #[method_id(@__retain_semantics Other printer)]
        pub unsafe fn printer(&self) -> Retained<NSPrinter>;

        #[cfg(feature = "NSPrinter")]
        /// Setter for [`printer`][Self::printer].
        #[method(setPrinter:)]
        pub unsafe fn setPrinter(&self, printer: &NSPrinter);

        #[method(setUpPrintOperationDefaultValues)]
        pub unsafe fn setUpPrintOperationDefaultValues(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(imageablePageBounds)]
        pub unsafe fn imageablePageBounds(&self) -> NSRect;

        #[method_id(@__retain_semantics Other localizedPaperName)]
        pub unsafe fn localizedPaperName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSPrinter")]
        #[method_id(@__retain_semantics Other defaultPrinter)]
        pub unsafe fn defaultPrinter() -> Option<Retained<NSPrinter>>;

        #[method_id(@__retain_semantics Other printSettings)]
        pub unsafe fn printSettings(
            &self,
        ) -> Retained<NSMutableDictionary<NSPrintInfoSettingKey, AnyObject>>;

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

        /// Setter for [`isSelectionOnly`][Self::isSelectionOnly].
        #[method(setSelectionOnly:)]
        pub unsafe fn setSelectionOnly(&self, selection_only: bool);

        #[cfg(feature = "NSPDFInfo")]
        #[method(takeSettingsFromPDFInfo:)]
        pub unsafe fn takeSettingsFromPDFInfo(&self, in_pdf_info: &NSPDFInfo);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPrintInfo {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPrintInfo {
        #[cfg(feature = "NSPrinter")]
        #[deprecated = "NSPrintInfo's implementation has no effect"]
        #[method(setDefaultPrinter:)]
        pub unsafe fn setDefaultPrinter(printer: Option<&NSPrinter>);

        #[cfg(all(feature = "NSPrinter", feature = "objc2-core-foundation"))]
        #[deprecated = "Use -[NSPrinter pageSizeForPaper:] instead"]
        #[method(sizeForPaperName:)]
        pub unsafe fn sizeForPaperName(name: Option<&NSPrinterPaperName>) -> NSSize;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintformname?language=objc)
    pub static NSPrintFormName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintjobfeatures?language=objc)
    pub static NSPrintJobFeatures: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintmanualfeed?language=objc)
    pub static NSPrintManualFeed: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpagespersheet?language=objc)
    pub static NSPrintPagesPerSheet: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintpaperfeed?language=objc)
    pub static NSPrintPaperFeed: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintsavepath?language=objc)
    pub static NSPrintSavePath: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsprintingorientation?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintingOrientation(pub NSUInteger);
impl NSPrintingOrientation {
    #[deprecated]
    pub const NSPortraitOrientation: Self = Self(0);
    #[deprecated]
    pub const NSLandscapeOrientation: Self = Self(1);
}

unsafe impl Encode for NSPrintingOrientation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintingOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsautopagination?language=objc)
pub static NSAutoPagination: NSPrintingPaginationMode =
    NSPrintingPaginationMode(NSPrintingPaginationMode::Automatic.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfitpagination?language=objc)
pub static NSFitPagination: NSPrintingPaginationMode =
    NSPrintingPaginationMode(NSPrintingPaginationMode::Fit.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsclippagination?language=objc)
pub static NSClipPagination: NSPrintingPaginationMode =
    NSPrintingPaginationMode(NSPrintingPaginationMode::Clip.0);
