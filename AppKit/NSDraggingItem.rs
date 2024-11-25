//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdraggingimagecomponentkey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSDraggingImageComponentKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdraggingimagecomponenticonkey?language=objc)
    pub static NSDraggingImageComponentIconKey: &'static NSDraggingImageComponentKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdraggingimagecomponentlabelkey?language=objc)
    pub static NSDraggingImageComponentLabelKey: &'static NSDraggingImageComponentKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdraggingimagecomponent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDraggingImageComponent;
);

unsafe impl NSObjectProtocol for NSDraggingImageComponent {}

extern_methods!(
    unsafe impl NSDraggingImageComponent {
        #[method_id(@__retain_semantics Other draggingImageComponentWithKey:)]
        pub unsafe fn draggingImageComponentWithKey(
            key: &NSDraggingImageComponentKey,
        ) -> Retained<NSDraggingImageComponent>;

        #[method_id(@__retain_semantics Init initWithKey:)]
        pub unsafe fn initWithKey(
            this: Allocated<Self>,
            key: &NSDraggingImageComponentKey,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Retained<NSDraggingImageComponentKey>;

        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: &NSDraggingImageComponentKey);

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Retained<AnyObject>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&AnyObject>);

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDraggingImageComponent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdraggingitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDraggingItem;
);

unsafe impl NSObjectProtocol for NSDraggingItem {}

extern_methods!(
    unsafe impl NSDraggingItem {
        #[cfg(feature = "NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboardWriter:)]
        pub unsafe fn initWithPasteboardWriter(
            this: Allocated<Self>,
            pasteboard_writer: &ProtocolObject<dyn NSPasteboardWriting>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other item)]
        pub unsafe fn item(&self) -> Retained<AnyObject>;

        #[method(draggingFrame)]
        pub unsafe fn draggingFrame(&self) -> NSRect;

        #[method(setDraggingFrame:)]
        pub unsafe fn setDraggingFrame(&self, dragging_frame: NSRect);

        #[cfg(feature = "block2")]
        #[method(imageComponentsProvider)]
        pub unsafe fn imageComponentsProvider(
            &self,
        ) -> *mut block2::Block<dyn Fn() -> NonNull<NSArray<NSDraggingImageComponent>>>;

        #[cfg(feature = "block2")]
        #[method(setImageComponentsProvider:)]
        pub unsafe fn setImageComponentsProvider(
            &self,
            image_components_provider: Option<
                &block2::Block<dyn Fn() -> NonNull<NSArray<NSDraggingImageComponent>>>,
            >,
        );

        #[method(setDraggingFrame:contents:)]
        pub unsafe fn setDraggingFrame_contents(&self, frame: NSRect, contents: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other imageComponents)]
        pub unsafe fn imageComponents(&self)
            -> Option<Retained<NSArray<NSDraggingImageComponent>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDraggingItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
