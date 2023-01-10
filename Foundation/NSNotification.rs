//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

typed_extensible_enum!(
    pub type NSNotificationName = Foundation::NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotification;

    unsafe impl ClassType for NSNotification {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSNotification")]
    unsafe impl NSNotification {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<Foundation::NSNotificationName, Shared>;

        #[method_id(@__retain_semantics Other object)]
        pub unsafe fn object(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<Foundation::NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithName:object:userInfo:)]
        pub unsafe fn initWithName_object_userInfo(
            this: Option<Allocated<Self>>,
            name: &Foundation::NSNotificationName,
            object: Option<&Object>,
            userInfo: Option<&Foundation::NSDictionary>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSNotificationCreation
    #[cfg(feature = "Foundation_NSNotification")]
    unsafe impl NSNotification {
        #[method_id(@__retain_semantics Other notificationWithName:object:)]
        pub unsafe fn notificationWithName_object(
            aName: &Foundation::NSNotificationName,
            anObject: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other notificationWithName:object:userInfo:)]
        pub unsafe fn notificationWithName_object_userInfo(
            aName: &Foundation::NSNotificationName,
            anObject: Option<&Object>,
            aUserInfo: Option<&Foundation::NSDictionary>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotificationCenter;

    unsafe impl ClassType for NSNotificationCenter {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSNotificationCenter")]
    unsafe impl NSNotificationCenter {
        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<Foundation::NSNotificationCenter, Shared>;

        #[method(addObserver:selector:name:object:)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &Object,
            aSelector: Sel,
            aName: Option<&Foundation::NSNotificationName>,
            anObject: Option<&Object>,
        );

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(postNotification:)]
        pub unsafe fn postNotification(&self, notification: &Foundation::NSNotification);

        #[method(postNotificationName:object:)]
        pub unsafe fn postNotificationName_object(
            &self,
            aName: &Foundation::NSNotificationName,
            anObject: Option<&Object>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(postNotificationName:object:userInfo:)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            aName: &Foundation::NSNotificationName,
            anObject: Option<&Object>,
            aUserInfo: Option<&Foundation::NSDictionary>,
        );

        #[method(removeObserver:)]
        pub unsafe fn removeObserver(&self, observer: &Object);

        #[method(removeObserver:name:object:)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &Object,
            aName: Option<&Foundation::NSNotificationName>,
            anObject: Option<&Object>,
        );

        #[cfg(all(
            feature = "Foundation_NSNotification",
            feature = "Foundation_NSOperationQueue"
        ))]
        #[method_id(@__retain_semantics Other addObserverForName:object:queue:usingBlock:)]
        pub unsafe fn addObserverForName_object_queue_usingBlock(
            &self,
            name: Option<&Foundation::NSNotificationName>,
            obj: Option<&Object>,
            queue: Option<&Foundation::NSOperationQueue>,
            block: &Block<(NonNull<Foundation::NSNotification>,), ()>,
        ) -> Id<NSObject, Shared>;
    }
);
