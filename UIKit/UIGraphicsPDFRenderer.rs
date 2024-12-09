//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfdrawingactions?language=objc)
#[cfg(all(feature = "UIGraphicsRenderer", feature = "block2"))]
pub type UIGraphicsPDFDrawingActions =
    *mut block2::Block<dyn Fn(NonNull<UIGraphicsPDFRendererContext>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfrendererformat?language=objc)
    #[unsafe(super(UIGraphicsRendererFormat, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRendererFormat;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSCopying for UIGraphicsPDFRendererFormat {}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl CopyingHelper for UIGraphicsPDFRendererFormat {
    type Result = Self;
}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRendererFormat {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[method_id(@__retain_semantics Other documentInfo)]
        pub unsafe fn documentInfo(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(setDocumentInfo:)]
        pub unsafe fn setDocumentInfo(&self, document_info: &NSDictionary<NSString, AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRendererFormat`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[deprecated]
        #[method_id(@__retain_semantics Other defaultFormat)]
        pub unsafe fn defaultFormat() -> Retained<Self>;

        #[method_id(@__retain_semantics Other preferredFormat)]
        pub unsafe fn preferredFormat() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfrenderercontext?language=objc)
    #[unsafe(super(UIGraphicsRendererContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRendererContext;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRendererContext {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererContext {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(pdfContextBounds)]
        pub unsafe fn pdfContextBounds(&self) -> CGRect;

        #[method(beginPage)]
        pub unsafe fn beginPage(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(beginPageWithBounds:pageInfo:)]
        pub unsafe fn beginPageWithBounds_pageInfo(
            &self,
            bounds: CGRect,
            page_info: &NSDictionary<NSString, AnyObject>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setURL:forRect:)]
        pub unsafe fn setURL_forRect(&self, url: &NSURL, rect: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(addDestinationWithName:atPoint:)]
        pub unsafe fn addDestinationWithName_atPoint(&self, name: &NSString, point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setDestinationWithName:forRect:)]
        pub unsafe fn setDestinationWithName_forRect(&self, name: &NSString, rect: CGRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfrenderer?language=objc)
    #[unsafe(super(UIGraphicsRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRenderer;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRenderer {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithBounds:format:)]
        pub unsafe fn initWithBounds_format(
            this: Allocated<Self>,
            bounds: CGRect,
            format: &UIGraphicsPDFRendererFormat,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(writePDFToURL:withActions:error:_)]
        pub unsafe fn writePDFToURL_withActions_error(
            &self,
            url: &NSURL,
            actions: UIGraphicsPDFDrawingActions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other PDFDataWithActions:)]
        pub unsafe fn PDFDataWithActions(
            &self,
            actions: UIGraphicsPDFDrawingActions,
        ) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRenderer`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithBounds:)]
        pub unsafe fn initWithBounds(this: Allocated<Self>, bounds: CGRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
