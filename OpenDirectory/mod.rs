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

#[link(name = "OpenDirectory", kind = "framework")]
extern "C" {}

#[cfg(feature = "CFOpenDirectory")]
#[path = "CFOpenDirectory/mod.rs"]
mod __CFOpenDirectory;
#[cfg(feature = "ODAttributeMap")]
#[path = "ODAttributeMap.rs"]
mod __ODAttributeMap;
#[cfg(feature = "ODConfiguration")]
#[path = "ODConfiguration.rs"]
mod __ODConfiguration;
#[cfg(feature = "ODMappings")]
#[path = "ODMappings.rs"]
mod __ODMappings;
#[cfg(feature = "ODModuleEntry")]
#[path = "ODModuleEntry.rs"]
mod __ODModuleEntry;
#[cfg(feature = "ODNode")]
#[path = "ODNode.rs"]
mod __ODNode;
#[cfg(feature = "ODQuery")]
#[path = "ODQuery.rs"]
mod __ODQuery;
#[cfg(feature = "ODRecord")]
#[path = "ODRecord.rs"]
mod __ODRecord;
#[cfg(feature = "ODRecordMap")]
#[path = "ODRecordMap.rs"]
mod __ODRecordMap;
#[cfg(feature = "ODSession")]
#[path = "ODSession.rs"]
mod __ODSession;

#[cfg(feature = "CFOpenDirectory")]
pub use self::__CFOpenDirectory::*;
#[cfg(feature = "ODAttributeMap")]
pub use self::__ODAttributeMap::ODAttributeMap;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODConfiguration;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketEncryptionAllow;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketEncryptionDisabled;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketEncryptionRequired;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketEncryptionSSL;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketSigningAllow;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketSigningDisabled;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODPacketSigningRequired;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODTrustTypeAnonymous;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODTrustTypeJoined;
#[cfg(feature = "ODConfiguration")]
pub use self::__ODConfiguration::ODTrustTypeUsingCredentials;
#[cfg(feature = "ODMappings")]
pub use self::__ODMappings::ODMappings;
#[cfg(feature = "ODModuleEntry")]
pub use self::__ODModuleEntry::ODModuleEntry;
#[cfg(feature = "ODNode")]
pub use self::__ODNode::ODNode;
#[cfg(feature = "ODQuery")]
pub use self::__ODQuery::ODQuery;
#[cfg(feature = "ODQuery")]
pub use self::__ODQuery::ODQueryDelegate;
#[cfg(feature = "ODRecord")]
pub use self::__ODRecord::ODRecord;
#[cfg(feature = "ODRecordMap")]
pub use self::__ODRecordMap::ODRecordMap;
#[cfg(feature = "ODSession")]
pub use self::__ODSession::ODSession;
#[cfg(feature = "ODSession")]
pub use self::__ODSession::ODSessionProxyAddress;
#[cfg(feature = "ODSession")]
pub use self::__ODSession::ODSessionProxyPassword;
#[cfg(feature = "ODSession")]
pub use self::__ODSession::ODSessionProxyPort;
#[cfg(feature = "ODSession")]
pub use self::__ODSession::ODSessionProxyUsername;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odframeworkerrordomain?language=objc)
    pub static ODFrameworkErrorDomain: Option<&'static NSString>;
}
