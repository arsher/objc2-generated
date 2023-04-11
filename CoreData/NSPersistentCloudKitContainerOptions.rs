//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
    pub struct NSPersistentCloudKitContainerOptions;

    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
    unsafe impl ClassType for NSPersistentCloudKitContainerOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerOptions {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
    unsafe impl NSPersistentCloudKitContainerOptions {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContainerIdentifier:)]
        pub unsafe fn initWithContainerIdentifier(
            this: Option<Allocated<Self>>,
            container_identifier: &NSString,
        ) -> Id<Self>;
    }
);
