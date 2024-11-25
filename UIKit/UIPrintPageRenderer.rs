//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprintrenderingquality?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPrintRenderingQuality(pub NSInteger);
impl UIPrintRenderingQuality {
    #[doc(alias = "UIPrintRenderingQualityBest")]
    pub const Best: Self = Self(0);
    #[doc(alias = "UIPrintRenderingQualityResponsive")]
    pub const Responsive: Self = Self(1);
}

unsafe impl Encode for UIPrintRenderingQuality {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPrintRenderingQuality {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprintpagerenderer?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrintPageRenderer;
);

unsafe impl NSObjectProtocol for UIPrintPageRenderer {}

extern_methods!(
    unsafe impl UIPrintPageRenderer {
        #[method(headerHeight)]
        pub unsafe fn headerHeight(&self) -> CGFloat;

        #[method(setHeaderHeight:)]
        pub unsafe fn setHeaderHeight(&self, header_height: CGFloat);

        #[method(footerHeight)]
        pub unsafe fn footerHeight(&self) -> CGFloat;

        #[method(setFooterHeight:)]
        pub unsafe fn setFooterHeight(&self, footer_height: CGFloat);

        #[method(paperRect)]
        pub unsafe fn paperRect(&self) -> CGRect;

        #[method(printableRect)]
        pub unsafe fn printableRect(&self) -> CGRect;

        #[method(numberOfPages)]
        pub unsafe fn numberOfPages(&self) -> NSInteger;

        #[cfg(feature = "UIPrintFormatter")]
        #[method_id(@__retain_semantics Other printFormatters)]
        pub unsafe fn printFormatters(&self) -> Option<Retained<NSArray<UIPrintFormatter>>>;

        #[cfg(feature = "UIPrintFormatter")]
        #[method(setPrintFormatters:)]
        pub unsafe fn setPrintFormatters(
            &self,
            print_formatters: Option<&NSArray<UIPrintFormatter>>,
        );

        #[cfg(feature = "UIPrintFormatter")]
        #[method_id(@__retain_semantics Other printFormattersForPageAtIndex:)]
        pub unsafe fn printFormattersForPageAtIndex(
            &self,
            page_index: NSInteger,
        ) -> Option<Retained<NSArray<UIPrintFormatter>>>;

        #[cfg(feature = "UIPrintFormatter")]
        #[method(addPrintFormatter:startingAtPageAtIndex:)]
        pub unsafe fn addPrintFormatter_startingAtPageAtIndex(
            &self,
            formatter: &UIPrintFormatter,
            page_index: NSInteger,
        );

        #[method(currentRenderingQualityForRequestedRenderingQuality:)]
        pub unsafe fn currentRenderingQualityForRequestedRenderingQuality(
            &self,
            requested_rendering_quality: UIPrintRenderingQuality,
        ) -> UIPrintRenderingQuality;

        #[method(prepareForDrawingPages:)]
        pub unsafe fn prepareForDrawingPages(&self, range: NSRange);

        #[method(drawPageAtIndex:inRect:)]
        pub unsafe fn drawPageAtIndex_inRect(&self, page_index: NSInteger, printable_rect: CGRect);

        #[cfg(feature = "UIPrintFormatter")]
        #[method(drawPrintFormatter:forPageAtIndex:)]
        pub unsafe fn drawPrintFormatter_forPageAtIndex(
            &self,
            print_formatter: &UIPrintFormatter,
            page_index: NSInteger,
        );

        #[method(drawHeaderForPageAtIndex:inRect:)]
        pub unsafe fn drawHeaderForPageAtIndex_inRect(
            &self,
            page_index: NSInteger,
            header_rect: CGRect,
        );

        #[method(drawContentForPageAtIndex:inRect:)]
        pub unsafe fn drawContentForPageAtIndex_inRect(
            &self,
            page_index: NSInteger,
            content_rect: CGRect,
        );

        #[method(drawFooterForPageAtIndex:inRect:)]
        pub unsafe fn drawFooterForPageAtIndex_inRect(
            &self,
            page_index: NSInteger,
            footer_rect: CGRect,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPrintPageRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
