//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photosui/phprojectextensioncontroller?language=objc)
    pub unsafe trait PHProjectExtensionController: NSObjectProtocol {
        #[cfg(feature = "PHProjectTypeDescription")]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other supportedProjectTypes)]
        unsafe fn supportedProjectTypes(&self) -> Retained<NSArray<PHProjectTypeDescription>>;

        #[cfg(all(
            feature = "PHProjectTypeDescriptionDataSource",
            feature = "PhotosUITypes"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other typeDescriptionDataSourceForCategory:invalidator:)]
        unsafe fn typeDescriptionDataSourceForCategory_invalidator(
            &self,
            category: &PHProjectCategory,
            invalidator: &ProtocolObject<dyn PHProjectTypeDescriptionInvalidator>,
        ) -> Retained<ProtocolObject<dyn PHProjectTypeDescriptionDataSource>>;

        #[cfg(all(
            feature = "PHProjectExtensionContext",
            feature = "PHProjectInfo",
            feature = "block2"
        ))]
        #[method(beginProjectWithExtensionContext:projectInfo:completion:)]
        unsafe fn beginProjectWithExtensionContext_projectInfo_completion(
            &self,
            extension_context: &PHProjectExtensionContext,
            project_info: &PHProjectInfo,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "PHProjectExtensionContext", feature = "block2"))]
        #[method(resumeProjectWithExtensionContext:completion:)]
        unsafe fn resumeProjectWithExtensionContext_completion(
            &self,
            extension_context: &PHProjectExtensionContext,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(finishProjectWithCompletionHandler:)]
        unsafe fn finishProjectWithCompletionHandler(&self, completion: &block2::Block<dyn Fn()>);
    }

    unsafe impl ProtocolType for dyn PHProjectExtensionController {}
);
