//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfastenumeration?language=objc)
    pub unsafe trait NSFastEnumeration {
        #[method(countByEnumeratingWithState:objects:count:)]
        unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: NonNull<*mut AnyObject>,
            len: NSUInteger,
        ) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn NSFastEnumeration {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsenumerator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEnumerator<ObjectType: ?Sized = AnyObject>;
);

unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSEnumerator<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSEnumerator<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[method_id(@__retain_semantics Other nextObject)]
        pub fn nextObject(&self) -> Option<Retained<ObjectType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSExtendedEnumerator
    unsafe impl<ObjectType: Message> NSEnumerator<ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub fn allObjects(&self) -> Retained<NSArray<ObjectType>>;
    }
);
