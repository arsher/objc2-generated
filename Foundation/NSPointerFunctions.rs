//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPointerFunctionsOptions {
        NSPointerFunctionsStrongMemory = 0 << 0,
        #[deprecated = "GC no longer supported"]
        NSPointerFunctionsZeroingWeakMemory = 1 << 0,
        NSPointerFunctionsOpaqueMemory = 2 << 0,
        NSPointerFunctionsMallocMemory = 3 << 0,
        NSPointerFunctionsMachVirtualMemory = 4 << 0,
        NSPointerFunctionsWeakMemory = 5 << 0,
        NSPointerFunctionsObjectPersonality = 0 << 8,
        NSPointerFunctionsOpaquePersonality = 1 << 8,
        NSPointerFunctionsObjectPointerPersonality = 2 << 8,
        NSPointerFunctionsCStringPersonality = 3 << 8,
        NSPointerFunctionsStructPersonality = 4 << 8,
        NSPointerFunctionsIntegerPersonality = 5 << 8,
        NSPointerFunctionsCopyIn = 1 << 16,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPointerFunctions")]
    pub struct NSPointerFunctions;

    #[cfg(feature = "Foundation_NSPointerFunctions")]
    unsafe impl ClassType for NSPointerFunctions {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSPointerFunctions")]
unsafe impl NSObjectProtocol for NSPointerFunctions {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPointerFunctions")]
    unsafe impl NSPointerFunctions {
        #[method_id(@__retain_semantics Init initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Option<Allocated<Self>>,
            options: NSPointerFunctionsOptions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other pointerFunctionsWithOptions:)]
        pub unsafe fn pointerFunctionsWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSPointerFunctions, Shared>;

        #[method(hashFunction)]
        pub unsafe fn hashFunction(
            &self,
        ) -> Option<
            unsafe extern "C" fn(
                NonNull<c_void>,
                Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
            ) -> NSUInteger,
        >;

        #[method(setHashFunction:)]
        pub unsafe fn setHashFunction(
            &self,
            hash_function: Option<
                unsafe extern "C" fn(
                    NonNull<c_void>,
                    Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
                ) -> NSUInteger,
            >,
        );

        #[method(isEqualFunction)]
        pub unsafe fn isEqualFunction(
            &self,
        ) -> Option<
            unsafe extern "C" fn(
                NonNull<c_void>,
                NonNull<c_void>,
                Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
            ) -> Bool,
        >;

        #[method(setIsEqualFunction:)]
        pub unsafe fn setIsEqualFunction(
            &self,
            is_equal_function: Option<
                unsafe extern "C" fn(
                    NonNull<c_void>,
                    NonNull<c_void>,
                    Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
                ) -> Bool,
            >,
        );

        #[method(sizeFunction)]
        pub unsafe fn sizeFunction(
            &self,
        ) -> Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>;

        #[method(setSizeFunction:)]
        pub unsafe fn setSizeFunction(
            &self,
            size_function: Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(descriptionFunction)]
        pub unsafe fn descriptionFunction(
            &self,
        ) -> Option<unsafe extern "C" fn(NonNull<c_void>) -> *mut NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDescriptionFunction:)]
        pub unsafe fn setDescriptionFunction(
            &self,
            description_function: Option<unsafe extern "C" fn(NonNull<c_void>) -> *mut NSString>,
        );

        #[method(relinquishFunction)]
        pub unsafe fn relinquishFunction(
            &self,
        ) -> Option<
            unsafe extern "C" fn(
                NonNull<c_void>,
                Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
            ),
        >;

        #[method(setRelinquishFunction:)]
        pub unsafe fn setRelinquishFunction(
            &self,
            relinquish_function: Option<
                unsafe extern "C" fn(
                    NonNull<c_void>,
                    Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
                ),
            >,
        );

        #[method(acquireFunction)]
        pub unsafe fn acquireFunction(
            &self,
        ) -> Option<
            unsafe extern "C" fn(
                NonNull<c_void>,
                Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
                Bool,
            ) -> NonNull<c_void>,
        >;

        #[method(setAcquireFunction:)]
        pub unsafe fn setAcquireFunction(
            &self,
            acquire_function: Option<
                unsafe extern "C" fn(
                    NonNull<c_void>,
                    Option<unsafe extern "C" fn(NonNull<c_void>) -> NSUInteger>,
                    Bool,
                ) -> NonNull<c_void>,
            >,
        );

        #[deprecated = "Garbage collection no longer supported"]
        #[method(usesStrongWriteBarrier)]
        pub unsafe fn usesStrongWriteBarrier(&self) -> bool;

        #[deprecated = "Garbage collection no longer supported"]
        #[method(setUsesStrongWriteBarrier:)]
        pub unsafe fn setUsesStrongWriteBarrier(&self, uses_strong_write_barrier: bool);

        #[deprecated = "Garbage collection no longer supported"]
        #[method(usesWeakReadAndWriteBarriers)]
        pub unsafe fn usesWeakReadAndWriteBarriers(&self) -> bool;

        #[deprecated = "Garbage collection no longer supported"]
        #[method(setUsesWeakReadAndWriteBarriers:)]
        pub unsafe fn setUsesWeakReadAndWriteBarriers(
            &self,
            uses_weak_read_and_write_barriers: bool,
        );
    }
);
