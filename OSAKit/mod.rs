// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::unportable_markdown)]
#![allow(rustdoc::invalid_html_tags)]

#[link(name = "OSAKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "OSALanguage")]
#[path = "OSALanguage.rs"]
mod __OSALanguage;
#[cfg(feature = "OSALanguageInstance")]
#[path = "OSALanguageInstance.rs"]
mod __OSALanguageInstance;
#[cfg(feature = "OSAScript")]
#[path = "OSAScript.rs"]
mod __OSAScript;
#[cfg(feature = "OSAScriptController")]
#[path = "OSAScriptController.rs"]
mod __OSAScriptController;
#[cfg(feature = "OSAScriptView")]
#[path = "OSAScriptView.rs"]
mod __OSAScriptView;

#[cfg(feature = "OSALanguage")]
pub use self::__OSALanguage::OSALanguage;
#[cfg(feature = "OSALanguage")]
pub use self::__OSALanguage::OSALanguageFeatures;
#[cfg(feature = "OSALanguageInstance")]
pub use self::__OSALanguageInstance::OSALanguageInstance;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScript;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorAppAddressKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorAppName;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorAppNameKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorBriefMessage;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorBriefMessageKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorExpectedTypeKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorMessage;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorMessageKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorNumber;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorNumberKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorOffendingObjectKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorPartialResultKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorRange;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAScriptErrorRangeKey;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAStorageApplicationBundleType;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAStorageApplicationType;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAStorageOptions;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAStorageScriptBundleType;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAStorageScriptType;
#[cfg(feature = "OSAScript")]
pub use self::__OSAScript::OSAStorageTextType;
#[cfg(feature = "OSAScriptController")]
pub use self::__OSAScriptController::OSAScriptController;
#[cfg(feature = "OSAScriptController")]
pub use self::__OSAScriptController::OSAScriptState;
#[cfg(feature = "OSAScriptView")]
pub use self::__OSAScriptView::OSAScriptView;
