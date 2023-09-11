//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLPatchType {
        MTLPatchTypeNone = 0,
        MTLPatchTypeTriangle = 1,
        MTLPatchTypeQuad = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexAttribute")]
    pub struct MTLVertexAttribute;

    #[cfg(feature = "Metal_MTLVertexAttribute")]
    unsafe impl ClassType for MTLVertexAttribute {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLVertexAttribute")]
unsafe impl NSObjectProtocol for MTLVertexAttribute {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexAttribute")]
    unsafe impl MTLVertexAttribute {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString>;

        #[method(attributeIndex)]
        pub fn attributeIndex(&self) -> NSUInteger;

        #[method(attributeType)]
        pub fn attributeType(&self) -> MTLDataType;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(isPatchData)]
        pub fn isPatchData(&self) -> bool;

        #[method(isPatchControlPointData)]
        pub fn isPatchControlPointData(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLVertexAttribute")]
    unsafe impl MTLVertexAttribute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAttribute")]
    pub struct MTLAttribute;

    #[cfg(feature = "Metal_MTLAttribute")]
    unsafe impl ClassType for MTLAttribute {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLAttribute")]
unsafe impl NSObjectProtocol for MTLAttribute {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAttribute")]
    unsafe impl MTLAttribute {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString>;

        #[method(attributeIndex)]
        pub fn attributeIndex(&self) -> NSUInteger;

        #[method(attributeType)]
        pub fn attributeType(&self) -> MTLDataType;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(isPatchData)]
        pub fn isPatchData(&self) -> bool;

        #[method(isPatchControlPointData)]
        pub fn isPatchControlPointData(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLAttribute")]
    unsafe impl MTLAttribute {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLFunctionType {
        MTLFunctionTypeVertex = 1,
        MTLFunctionTypeFragment = 2,
        MTLFunctionTypeKernel = 3,
        MTLFunctionTypeVisible = 5,
        MTLFunctionTypeIntersection = 6,
        MTLFunctionTypeMesh = 7,
        MTLFunctionTypeObject = 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLFunctionConstant")]
    pub struct MTLFunctionConstant;

    #[cfg(feature = "Metal_MTLFunctionConstant")]
    unsafe impl ClassType for MTLFunctionConstant {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLFunctionConstant")]
unsafe impl NSObjectProtocol for MTLFunctionConstant {}

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionConstant")]
    unsafe impl MTLFunctionConstant {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLDataType;

        #[method(index)]
        pub fn index(&self) -> NSUInteger;

        #[method(required)]
        pub fn required(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLFunctionConstant")]
    unsafe impl MTLFunctionConstant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLFunction: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[method(functionType)]
        fn functionType(&self) -> MTLFunctionType;

        #[method(patchType)]
        fn patchType(&self) -> MTLPatchType;

        #[method(patchControlPointCount)]
        fn patchControlPointCount(&self) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLVertexAttribute"))]
        #[method_id(@__retain_semantics Other vertexAttributes)]
        fn vertexAttributes(&self) -> Option<Id<NSArray<MTLVertexAttribute>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLAttribute"))]
        #[method_id(@__retain_semantics Other stageInputAttributes)]
        fn stageInputAttributes(&self) -> Option<Id<NSArray<MTLAttribute>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        fn name(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionConstant"
        ))]
        #[method_id(@__retain_semantics Other functionConstantsDictionary)]
        fn functionConstantsDictionary(&self) -> Id<NSDictionary<NSString, MTLFunctionConstant>>;

        #[method_id(@__retain_semantics New newArgumentEncoderWithBufferIndex:)]
        unsafe fn newArgumentEncoderWithBufferIndex(
            &self,
            buffer_index: NSUInteger,
        ) -> Id<ProtocolObject<dyn MTLArgumentEncoder>>;

        #[method(options)]
        fn options(&self) -> MTLFunctionOptions;
    }

    unsafe impl ProtocolType for dyn MTLFunction {}
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLLanguageVersion {
        #[deprecated = "Use a newer language standard"]
        MTLLanguageVersion1_0 = 1 << 16,
        MTLLanguageVersion1_1 = (1 << 16) + 1,
        MTLLanguageVersion1_2 = (1 << 16) + 2,
        MTLLanguageVersion2_0 = 2 << 16,
        MTLLanguageVersion2_1 = (2 << 16) + 1,
        MTLLanguageVersion2_2 = (2 << 16) + 2,
        MTLLanguageVersion2_3 = (2 << 16) + 3,
        MTLLanguageVersion2_4 = (2 << 16) + 4,
        MTLLanguageVersion3_0 = (3 << 16) + 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLLibraryType {
        MTLLibraryTypeExecutable = 0,
        MTLLibraryTypeDynamic = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLLibraryOptimizationLevel {
        MTLLibraryOptimizationLevelDefault = 0,
        MTLLibraryOptimizationLevelSize = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCompileSymbolVisibility {
        MTLCompileSymbolVisibilityDefault = 0,
        MTLCompileSymbolVisibilityHidden = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLCompileOptions")]
    pub struct MTLCompileOptions;

    #[cfg(feature = "Metal_MTLCompileOptions")]
    unsafe impl ClassType for MTLCompileOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLCompileOptions")]
unsafe impl NSCopying for MTLCompileOptions {}

#[cfg(feature = "Metal_MTLCompileOptions")]
unsafe impl NSObjectProtocol for MTLCompileOptions {}

extern_methods!(
    #[cfg(feature = "Metal_MTLCompileOptions")]
    unsafe impl MTLCompileOptions {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other preprocessorMacros)]
        pub fn preprocessorMacros(&self) -> Option<Id<NSDictionary<NSString, NSObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setPreprocessorMacros:)]
        pub unsafe fn setPreprocessorMacros(
            &self,
            preprocessor_macros: Option<&NSDictionary<NSString, NSObject>>,
        );

        #[method(fastMathEnabled)]
        pub fn fastMathEnabled(&self) -> bool;

        #[method(setFastMathEnabled:)]
        pub fn setFastMathEnabled(&self, fast_math_enabled: bool);

        #[method(languageVersion)]
        pub fn languageVersion(&self) -> MTLLanguageVersion;

        #[method(setLanguageVersion:)]
        pub fn setLanguageVersion(&self, language_version: MTLLanguageVersion);

        #[method(libraryType)]
        pub fn libraryType(&self) -> MTLLibraryType;

        #[method(setLibraryType:)]
        pub fn setLibraryType(&self, library_type: MTLLibraryType);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other installName)]
        pub fn installName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInstallName:)]
        pub unsafe fn setInstallName(&self, install_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other libraries)]
        pub fn libraries(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setLibraries:)]
        pub fn setLibraries(
            &self,
            libraries: Option<&NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>,
        );

        #[method(preserveInvariance)]
        pub fn preserveInvariance(&self) -> bool;

        #[method(setPreserveInvariance:)]
        pub fn setPreserveInvariance(&self, preserve_invariance: bool);

        #[method(optimizationLevel)]
        pub unsafe fn optimizationLevel(&self) -> MTLLibraryOptimizationLevel;

        #[method(setOptimizationLevel:)]
        pub unsafe fn setOptimizationLevel(&self, optimization_level: MTLLibraryOptimizationLevel);

        #[method(compileSymbolVisibility)]
        pub unsafe fn compileSymbolVisibility(&self) -> MTLCompileSymbolVisibility;

        #[method(setCompileSymbolVisibility:)]
        pub unsafe fn setCompileSymbolVisibility(
            &self,
            compile_symbol_visibility: MTLCompileSymbolVisibility,
        );

        #[method(allowReferencingUndefinedSymbols)]
        pub unsafe fn allowReferencingUndefinedSymbols(&self) -> bool;

        #[method(setAllowReferencingUndefinedSymbols:)]
        pub unsafe fn setAllowReferencingUndefinedSymbols(
            &self,
            allow_referencing_undefined_symbols: bool,
        );

        #[method(maxTotalThreadsPerThreadgroup)]
        pub unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger;

        #[method(setMaxTotalThreadsPerThreadgroup:)]
        pub unsafe fn setMaxTotalThreadsPerThreadgroup(
            &self,
            max_total_threads_per_threadgroup: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLCompileOptions")]
    unsafe impl MTLCompileOptions {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Metal_MTLCompileOptions")]
impl DefaultId for MTLCompileOptions {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_static!(MTLLibraryErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLLibraryError {
        MTLLibraryErrorUnsupported = 1,
        MTLLibraryErrorInternal = 2,
        MTLLibraryErrorCompileFailure = 3,
        MTLLibraryErrorCompileWarning = 4,
        MTLLibraryErrorFunctionNotFound = 5,
        MTLLibraryErrorFileNotFound = 6,
    }
);

extern_protocol!(
    pub unsafe trait MTLLibrary: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics New newFunctionWithName:)]
        fn newFunctionWithName(
            &self,
            function_name: &NSString,
        ) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionConstantValues"
        ))]
        #[method_id(@__retain_semantics New newFunctionWithName:constantValues:error:_)]
        fn newFunctionWithName_constantValues_error(
            &self,
            name: &NSString,
            constant_values: &MTLFunctionConstantValues,
        ) -> Result<Id<ProtocolObject<dyn MTLFunction>>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionConstantValues"
        ))]
        #[method(newFunctionWithName:constantValues:completionHandler:)]
        unsafe fn newFunctionWithName_constantValues_completionHandler(
            &self,
            name: &NSString,
            constant_values: &MTLFunctionConstantValues,
            completion_handler: &Block<(*mut ProtocolObject<dyn MTLFunction>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLFunctionDescriptor"
        ))]
        #[method(newFunctionWithDescriptor:completionHandler:)]
        unsafe fn newFunctionWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLFunctionDescriptor,
            completion_handler: &Block<(*mut ProtocolObject<dyn MTLFunction>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLFunctionDescriptor"
        ))]
        #[method_id(@__retain_semantics New newFunctionWithDescriptor:error:_)]
        fn newFunctionWithDescriptor_error(
            &self,
            descriptor: &MTLFunctionDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLFunction>>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLIntersectionFunctionDescriptor"
        ))]
        #[method(newIntersectionFunctionWithDescriptor:completionHandler:)]
        unsafe fn newIntersectionFunctionWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLIntersectionFunctionDescriptor,
            completion_handler: &Block<(*mut ProtocolObject<dyn MTLFunction>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLIntersectionFunctionDescriptor"
        ))]
        #[method_id(@__retain_semantics New newIntersectionFunctionWithDescriptor:error:_)]
        fn newIntersectionFunctionWithDescriptor_error(
            &self,
            descriptor: &MTLIntersectionFunctionDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLFunction>>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other functionNames)]
        fn functionNames(&self) -> Id<NSArray<NSString>>;

        #[method(type)]
        unsafe fn r#type(&self) -> MTLLibraryType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other installName)]
        fn installName(&self) -> Option<Id<NSString>>;
    }

    unsafe impl ProtocolType for dyn MTLLibrary {}
);
