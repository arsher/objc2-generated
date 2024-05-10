//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SKProductStorePromotionVisibility {
    #[doc(alias = "SKProductStorePromotionVisibilityDefault")]
    Default = 0,
    #[doc(alias = "SKProductStorePromotionVisibilityShow")]
    Show = 1,
    #[doc(alias = "SKProductStorePromotionVisibilityHide")]
    Hide = 2,
}

unsafe impl Encode for SKProductStorePromotionVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKProductStorePromotionVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKProductStorePromotionController;

    unsafe impl ClassType for SKProductStorePromotionController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKProductStorePromotionController {}

extern_methods!(
    unsafe impl SKProductStorePromotionController {
        #[method_id(@__retain_semantics Other defaultController)]
        pub unsafe fn defaultController() -> Id<Self>;

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[method(fetchStorePromotionVisibilityForProduct:completionHandler:)]
        pub unsafe fn fetchStorePromotionVisibilityForProduct_completionHandler(
            &self,
            product: &SKProduct,
            completion_handler: Option<
                &block2::Block<dyn Fn(SKProductStorePromotionVisibility, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[method(updateStorePromotionVisibility:forProduct:completionHandler:)]
        pub unsafe fn updateStorePromotionVisibility_forProduct_completionHandler(
            &self,
            promotion_visibility: SKProductStorePromotionVisibility,
            product: &SKProduct,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[method(fetchStorePromotionOrderWithCompletionHandler:)]
        pub unsafe fn fetchStorePromotionOrderWithCompletionHandler(
            &self,
            completion_handler: Option<
                &block2::Block<dyn Fn(NonNull<NSArray<SKProduct>>, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "SKProduct", feature = "block2"))]
        #[method(updateStorePromotionOrder:completionHandler:)]
        pub unsafe fn updateStorePromotionOrder_completionHandler(
            &self,
            promotion_order: &NSArray<SKProduct>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKProductStorePromotionController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
