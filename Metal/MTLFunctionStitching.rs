//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStitchedLibraryOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTLStitchedLibraryOptions: NSUInteger {
        const MTLStitchedLibraryOptionNone = 0;
        const MTLStitchedLibraryOptionFailOnBinaryArchiveMiss = 1<<0;
        const MTLStitchedLibraryOptionStoreLibraryInMetalScript = 1<<1;
    }
}

unsafe impl Encode for MTLStitchedLibraryOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLStitchedLibraryOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MTLFunctionStitchingAttribute: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn MTLFunctionStitchingAttribute {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingAttributeAlwaysInline;

    unsafe impl ClassType for MTLFunctionStitchingAttributeAlwaysInline {
        type Super = NSObject;
    }
);

unsafe impl MTLFunctionStitchingAttribute for MTLFunctionStitchingAttributeAlwaysInline {}

unsafe impl NSObjectProtocol for MTLFunctionStitchingAttributeAlwaysInline {}

extern_methods!(
    unsafe impl MTLFunctionStitchingAttributeAlwaysInline {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionStitchingAttributeAlwaysInline {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLFunctionStitchingNode: NSCopying + NSObjectProtocol {}

    unsafe impl ProtocolType for dyn MTLFunctionStitchingNode {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingInputNode;

    unsafe impl ClassType for MTLFunctionStitchingInputNode {
        type Super = NSObject;
    }
);

unsafe impl MTLFunctionStitchingNode for MTLFunctionStitchingInputNode {}

unsafe impl NSCopying for MTLFunctionStitchingInputNode {}

unsafe impl CopyingHelper for MTLFunctionStitchingInputNode {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFunctionStitchingInputNode {}

extern_methods!(
    unsafe impl MTLFunctionStitchingInputNode {
        #[method(argumentIndex)]
        pub unsafe fn argumentIndex(&self) -> NSUInteger;

        #[method(setArgumentIndex:)]
        pub unsafe fn setArgumentIndex(&self, argument_index: NSUInteger);

        #[method_id(@__retain_semantics Init initWithArgumentIndex:)]
        pub unsafe fn initWithArgumentIndex(
            this: Allocated<Self>,
            argument: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionStitchingInputNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingFunctionNode;

    unsafe impl ClassType for MTLFunctionStitchingFunctionNode {
        type Super = NSObject;
    }
);

unsafe impl MTLFunctionStitchingNode for MTLFunctionStitchingFunctionNode {}

unsafe impl NSCopying for MTLFunctionStitchingFunctionNode {}

unsafe impl CopyingHelper for MTLFunctionStitchingFunctionNode {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFunctionStitchingFunctionNode {}

extern_methods!(
    unsafe impl MTLFunctionStitchingFunctionNode {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MTLFunctionStitchingNode>>>;

        #[method(setArguments:)]
        pub unsafe fn setArguments(
            &self,
            arguments: &NSArray<ProtocolObject<dyn MTLFunctionStitchingNode>>,
        );

        #[method_id(@__retain_semantics Other controlDependencies)]
        pub unsafe fn controlDependencies(
            &self,
        ) -> Retained<NSArray<MTLFunctionStitchingFunctionNode>>;

        #[method(setControlDependencies:)]
        pub unsafe fn setControlDependencies(
            &self,
            control_dependencies: &NSArray<MTLFunctionStitchingFunctionNode>,
        );

        #[method_id(@__retain_semantics Init initWithName:arguments:controlDependencies:)]
        pub unsafe fn initWithName_arguments_controlDependencies(
            this: Allocated<Self>,
            name: &NSString,
            arguments: &NSArray<ProtocolObject<dyn MTLFunctionStitchingNode>>,
            control_dependencies: &NSArray<MTLFunctionStitchingFunctionNode>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionStitchingFunctionNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingGraph;

    unsafe impl ClassType for MTLFunctionStitchingGraph {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLFunctionStitchingGraph {}

unsafe impl CopyingHelper for MTLFunctionStitchingGraph {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLFunctionStitchingGraph {}

extern_methods!(
    unsafe impl MTLFunctionStitchingGraph {
        #[method_id(@__retain_semantics Other functionName)]
        pub unsafe fn functionName(&self) -> Retained<NSString>;

        #[method(setFunctionName:)]
        pub unsafe fn setFunctionName(&self, function_name: &NSString);

        #[method_id(@__retain_semantics Other nodes)]
        pub unsafe fn nodes(&self) -> Retained<NSArray<MTLFunctionStitchingFunctionNode>>;

        #[method(setNodes:)]
        pub unsafe fn setNodes(&self, nodes: &NSArray<MTLFunctionStitchingFunctionNode>);

        #[method_id(@__retain_semantics Other outputNode)]
        pub unsafe fn outputNode(&self) -> Option<Retained<MTLFunctionStitchingFunctionNode>>;

        #[method(setOutputNode:)]
        pub unsafe fn setOutputNode(&self, output_node: Option<&MTLFunctionStitchingFunctionNode>);

        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MTLFunctionStitchingAttribute>>>;

        #[method(setAttributes:)]
        pub unsafe fn setAttributes(
            &self,
            attributes: &NSArray<ProtocolObject<dyn MTLFunctionStitchingAttribute>>,
        );

        #[method_id(@__retain_semantics Init initWithFunctionName:nodes:outputNode:attributes:)]
        pub unsafe fn initWithFunctionName_nodes_outputNode_attributes(
            this: Allocated<Self>,
            function_name: &NSString,
            nodes: &NSArray<MTLFunctionStitchingFunctionNode>,
            output_node: Option<&MTLFunctionStitchingFunctionNode>,
            attributes: &NSArray<ProtocolObject<dyn MTLFunctionStitchingAttribute>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLFunctionStitchingGraph {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStitchedLibraryDescriptor;

    unsafe impl ClassType for MTLStitchedLibraryDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLStitchedLibraryDescriptor {}

unsafe impl CopyingHelper for MTLStitchedLibraryDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLStitchedLibraryDescriptor {}

extern_methods!(
    unsafe impl MTLStitchedLibraryDescriptor {
        #[method_id(@__retain_semantics Other functionGraphs)]
        pub unsafe fn functionGraphs(&self) -> Retained<NSArray<MTLFunctionStitchingGraph>>;

        #[method(setFunctionGraphs:)]
        pub unsafe fn setFunctionGraphs(
            &self,
            function_graphs: &NSArray<MTLFunctionStitchingGraph>,
        );

        #[cfg(feature = "MTLLibrary")]
        #[method_id(@__retain_semantics Other functions)]
        pub unsafe fn functions(&self) -> Retained<NSArray<ProtocolObject<dyn MTLFunction>>>;

        #[cfg(feature = "MTLLibrary")]
        #[method(setFunctions:)]
        pub unsafe fn setFunctions(&self, functions: &NSArray<ProtocolObject<dyn MTLFunction>>);

        #[cfg(feature = "MTLBinaryArchive")]
        #[method_id(@__retain_semantics Other binaryArchives)]
        pub unsafe fn binaryArchives(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>;

        #[cfg(feature = "MTLBinaryArchive")]
        #[method(setBinaryArchives:)]
        pub unsafe fn setBinaryArchives(
            &self,
            binary_archives: &NSArray<ProtocolObject<dyn MTLBinaryArchive>>,
        );

        #[method(options)]
        pub unsafe fn options(&self) -> MTLStitchedLibraryOptions;

        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: MTLStitchedLibraryOptions);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLStitchedLibraryDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
