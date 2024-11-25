//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchinsertrequestresulttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBatchInsertRequestResultType(pub NSUInteger);
impl NSBatchInsertRequestResultType {
    #[doc(alias = "NSBatchInsertRequestResultTypeStatusOnly")]
    pub const StatusOnly: Self = Self(0x0);
    #[doc(alias = "NSBatchInsertRequestResultTypeObjectIDs")]
    pub const ObjectIDs: Self = Self(0x1);
    #[doc(alias = "NSBatchInsertRequestResultTypeCount")]
    pub const Count: Self = Self(0x2);
}

unsafe impl Encode for NSBatchInsertRequestResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBatchInsertRequestResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchupdaterequestresulttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBatchUpdateRequestResultType(pub NSUInteger);
impl NSBatchUpdateRequestResultType {
    pub const NSStatusOnlyResultType: Self = Self(0x0);
    pub const NSUpdatedObjectIDsResultType: Self = Self(0x1);
    pub const NSUpdatedObjectsCountResultType: Self = Self(0x2);
}

unsafe impl Encode for NSBatchUpdateRequestResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBatchUpdateRequestResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchdeleterequestresulttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBatchDeleteRequestResultType(pub NSUInteger);
impl NSBatchDeleteRequestResultType {
    pub const NSBatchDeleteResultTypeStatusOnly: Self = Self(0x0);
    pub const NSBatchDeleteResultTypeObjectIDs: Self = Self(0x1);
    pub const NSBatchDeleteResultTypeCount: Self = Self(0x2);
}

unsafe impl Encode for NSBatchDeleteRequestResultType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBatchDeleteRequestResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistenthistoryresulttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentHistoryResultType(pub NSInteger);
impl NSPersistentHistoryResultType {
    #[doc(alias = "NSPersistentHistoryResultTypeStatusOnly")]
    pub const StatusOnly: Self = Self(0x0);
    #[doc(alias = "NSPersistentHistoryResultTypeObjectIDs")]
    pub const ObjectIDs: Self = Self(0x1);
    #[doc(alias = "NSPersistentHistoryResultTypeCount")]
    pub const Count: Self = Self(0x2);
    #[doc(alias = "NSPersistentHistoryResultTypeTransactionsOnly")]
    pub const TransactionsOnly: Self = Self(0x3);
    #[doc(alias = "NSPersistentHistoryResultTypeChangesOnly")]
    pub const ChangesOnly: Self = Self(0x4);
    #[doc(alias = "NSPersistentHistoryResultTypeTransactionsAndChanges")]
    pub const TransactionsAndChanges: Self = Self(0x5);
}

unsafe impl Encode for NSPersistentHistoryResultType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentHistoryResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreresult?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreResult;
);

unsafe impl NSObjectProtocol for NSPersistentStoreResult {}

extern_methods!(
    unsafe impl NSPersistentStoreResult {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentStoreResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentstoreasynchronousresult?language=objc)
    #[unsafe(super(NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreAsynchronousResult;
);

unsafe impl NSObjectProtocol for NSPersistentStoreAsynchronousResult {}

extern_methods!(
    unsafe impl NSPersistentStoreAsynchronousResult {
        #[cfg(feature = "NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Retained<NSManagedObjectContext>;

        #[method_id(@__retain_semantics Other operationError)]
        pub unsafe fn operationError(&self) -> Option<Retained<NSError>>;

        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Option<Retained<NSProgress>>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentStoreAsynchronousResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsasynchronousfetchresult?language=objc)
    #[unsafe(super(NSPersistentStoreAsynchronousResult, NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAsynchronousFetchResult<ResultType: ?Sized = AnyObject>;
);

unsafe impl<ResultType: ?Sized> NSObjectProtocol for NSAsynchronousFetchResult<ResultType> {}

extern_methods!(
    unsafe impl<ResultType: Message> NSAsynchronousFetchResult<ResultType> {
        #[cfg(all(feature = "NSFetchRequest", feature = "NSPersistentStoreRequest"))]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Retained<NSAsynchronousFetchRequest<ResultType>>;

        #[method_id(@__retain_semantics Other finalResult)]
        pub unsafe fn finalResult(&self) -> Option<Retained<NSArray<ResultType>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ResultType: Message> NSAsynchronousFetchResult<ResultType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchinsertresult?language=objc)
    #[unsafe(super(NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchInsertResult;
);

unsafe impl NSObjectProtocol for NSBatchInsertResult {}

extern_methods!(
    unsafe impl NSBatchInsertResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Retained<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBatchInsertResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchupdateresult?language=objc)
    #[unsafe(super(NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchUpdateResult;
);

unsafe impl NSObjectProtocol for NSBatchUpdateResult {}

extern_methods!(
    unsafe impl NSBatchUpdateResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Retained<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchUpdateRequestResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBatchUpdateResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nsbatchdeleteresult?language=objc)
    #[unsafe(super(NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchDeleteResult;
);

unsafe impl NSObjectProtocol for NSBatchDeleteResult {}

extern_methods!(
    unsafe impl NSBatchDeleteResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Retained<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSBatchDeleteResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistenthistoryresult?language=objc)
    #[unsafe(super(NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentHistoryResult;
);

unsafe impl NSObjectProtocol for NSPersistentHistoryResult {}

extern_methods!(
    unsafe impl NSPersistentHistoryResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Retained<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentHistoryResultType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPersistentHistoryResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentcloudkitcontainereventresulttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPersistentCloudKitContainerEventResultType(pub NSInteger);
impl NSPersistentCloudKitContainerEventResultType {
    #[doc(alias = "NSPersistentCloudKitContainerEventResultTypeEvents")]
    pub const Events: Self = Self(0);
    #[doc(alias = "NSPersistentCloudKitContainerEventResultTypeCountEvents")]
    pub const CountEvents: Self = Self(1);
}

unsafe impl Encode for NSPersistentCloudKitContainerEventResultType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPersistentCloudKitContainerEventResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coredata/nspersistentcloudkitcontainereventresult?language=objc)
    #[unsafe(super(NSPersistentStoreResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentCloudKitContainerEventResult;
);

unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEventResult {}

extern_methods!(
    unsafe impl NSPersistentCloudKitContainerEventResult {
        #[method_id(@__retain_semantics Other result)]
        pub unsafe fn result(&self) -> Option<Retained<AnyObject>>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSPersistentCloudKitContainerEventResultType;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
