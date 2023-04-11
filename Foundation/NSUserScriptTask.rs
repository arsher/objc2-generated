//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSUserScriptTaskCompletionHandler = *mut Block<(*mut NSError,), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserScriptTask")]
    pub struct NSUserScriptTask;

    #[cfg(feature = "Foundation_NSUserScriptTask")]
    unsafe impl ClassType for NSUserScriptTask {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserScriptTask")]
unsafe impl NSObjectProtocol for NSUserScriptTask {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserScriptTask")]
    unsafe impl NSUserScriptTask {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other scriptURL)]
        pub unsafe fn scriptURL(&self) -> Id<NSURL>;

        #[method(executeWithCompletionHandler:)]
        pub unsafe fn executeWithCompletionHandler(
            &self,
            handler: NSUserScriptTaskCompletionHandler,
        );
    }
);

pub type NSUserUnixTaskCompletionHandler = *mut Block<(*mut NSError,), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    pub struct NSUserUnixTask;

    #[cfg(feature = "Foundation_NSUserUnixTask")]
    unsafe impl ClassType for NSUserUnixTask {
        #[inherits(NSObject)]
        type Super = NSUserScriptTask;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserUnixTask")]
unsafe impl NSObjectProtocol for NSUserUnixTask {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    unsafe impl NSUserUnixTask {
        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method_id(@__retain_semantics Other standardInput)]
        pub unsafe fn standardInput(&self) -> Option<Id<NSFileHandle>>;

        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method(setStandardInput:)]
        pub unsafe fn setStandardInput(&self, standard_input: Option<&NSFileHandle>);

        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method_id(@__retain_semantics Other standardOutput)]
        pub unsafe fn standardOutput(&self) -> Option<Id<NSFileHandle>>;

        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method(setStandardOutput:)]
        pub unsafe fn setStandardOutput(&self, standard_output: Option<&NSFileHandle>);

        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method_id(@__retain_semantics Other standardError)]
        pub unsafe fn standardError(&self) -> Option<Id<NSFileHandle>>;

        #[cfg(feature = "Foundation_NSFileHandle")]
        #[method(setStandardError:)]
        pub unsafe fn setStandardError(&self, standard_error: Option<&NSFileHandle>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(executeWithArguments:completionHandler:)]
        pub unsafe fn executeWithArguments_completionHandler(
            &self,
            arguments: Option<&NSArray<NSString>>,
            handler: NSUserUnixTaskCompletionHandler,
        );
    }
);

pub type NSUserAppleScriptTaskCompletionHandler =
    *mut Block<(*mut NSAppleEventDescriptor, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    pub struct NSUserAppleScriptTask;

    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    unsafe impl ClassType for NSUserAppleScriptTask {
        #[inherits(NSObject)]
        type Super = NSUserScriptTask;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserAppleScriptTask")]
unsafe impl NSObjectProtocol for NSUserAppleScriptTask {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    unsafe impl NSUserAppleScriptTask {
        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method(executeWithAppleEvent:completionHandler:)]
        pub unsafe fn executeWithAppleEvent_completionHandler(
            &self,
            event: Option<&NSAppleEventDescriptor>,
            handler: NSUserAppleScriptTaskCompletionHandler,
        );
    }
);

pub type NSUserAutomatorTaskCompletionHandler = *mut Block<(*mut Object, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    pub struct NSUserAutomatorTask;

    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    unsafe impl ClassType for NSUserAutomatorTask {
        #[inherits(NSObject)]
        type Super = NSUserScriptTask;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserAutomatorTask")]
unsafe impl NSObjectProtocol for NSUserAutomatorTask {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    unsafe impl NSUserAutomatorTask {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other variables)]
        pub unsafe fn variables(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setVariables:)]
        pub unsafe fn setVariables(&self, variables: Option<&NSDictionary<NSString, Object>>);

        #[method(executeWithInput:completionHandler:)]
        pub unsafe fn executeWithInput_completionHandler(
            &self,
            input: Option<&ProtocolObject<dyn NSSecureCoding>>,
            handler: NSUserAutomatorTaskCompletionHandler,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserScriptTask`
    #[cfg(feature = "Foundation_NSUserUnixTask")]
    unsafe impl NSUserUnixTask {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserScriptTask`
    #[cfg(feature = "Foundation_NSUserAppleScriptTask")]
    unsafe impl NSUserAppleScriptTask {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUserScriptTask`
    #[cfg(feature = "Foundation_NSUserAutomatorTask")]
    unsafe impl NSUserAutomatorTask {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);
