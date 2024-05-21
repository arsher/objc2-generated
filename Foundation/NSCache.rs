//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCache<KeyType: ?Sized = AnyObject, ObjectType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut KeyType>,
        _inner1: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<KeyType: ?Sized + Message, ObjectType: ?Sized + Message> ClassType
        for NSCache<KeyType, ObjectType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSObjectProtocol for NSCache<KeyType, ObjectType> {}

extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSCache<KeyType, ObjectType> {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn NSCacheDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSCacheDelegate>>);

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, key: &KeyType) -> Option<Retained<ObjectType>>;

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, obj: &ObjectType, key: &KeyType);

        #[method(setObject:forKey:cost:)]
        pub unsafe fn setObject_forKey_cost(&self, obj: &ObjectType, key: &KeyType, g: NSUInteger);

        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, key: &KeyType);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method(totalCostLimit)]
        pub unsafe fn totalCostLimit(&self) -> NSUInteger;

        #[method(setTotalCostLimit:)]
        pub unsafe fn setTotalCostLimit(&self, total_cost_limit: NSUInteger);

        #[method(countLimit)]
        pub unsafe fn countLimit(&self) -> NSUInteger;

        #[method(setCountLimit:)]
        pub unsafe fn setCountLimit(&self, count_limit: NSUInteger);

        #[method(evictsObjectsWithDiscardedContent)]
        pub unsafe fn evictsObjectsWithDiscardedContent(&self) -> bool;

        #[method(setEvictsObjectsWithDiscardedContent:)]
        pub unsafe fn setEvictsObjectsWithDiscardedContent(
            &self,
            evicts_objects_with_discarded_content: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<KeyType: Message, ObjectType: Message> NSCache<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSCacheDelegate: NSObjectProtocol {
        #[optional]
        #[method(cache:willEvictObject:)]
        unsafe fn cache_willEvictObject(&self, cache: &NSCache, obj: &AnyObject);
    }

    unsafe impl ProtocolType for dyn NSCacheDelegate {}
);
