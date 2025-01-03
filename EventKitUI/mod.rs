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

#[link(name = "EventKitUI", kind = "framework")]
extern "C" {}

#[cfg(feature = "EKCalendarChooser")]
#[path = "EKCalendarChooser.rs"]
mod __EKCalendarChooser;
#[cfg(feature = "EKEventEditViewController")]
#[path = "EKEventEditViewController.rs"]
mod __EKEventEditViewController;
#[cfg(feature = "EKEventViewController")]
#[path = "EKEventViewController.rs"]
mod __EKEventViewController;
#[cfg(feature = "EventKitUIBundle")]
#[path = "EventKitUIBundle.rs"]
mod __EventKitUIBundle;
#[cfg(feature = "EventKitUIDefines")]
#[path = "EventKitUIDefines.rs"]
mod __EventKitUIDefines;

#[cfg(all(feature = "EKCalendarChooser", feature = "objc2-ui-kit"))]
pub use self::__EKCalendarChooser::EKCalendarChooser;
#[cfg(feature = "EKCalendarChooser")]
pub use self::__EKCalendarChooser::EKCalendarChooserDelegate;
#[cfg(feature = "EKCalendarChooser")]
pub use self::__EKCalendarChooser::EKCalendarChooserDisplayStyle;
#[cfg(feature = "EKCalendarChooser")]
pub use self::__EKCalendarChooser::EKCalendarChooserSelectionStyle;
#[cfg(feature = "EKEventEditViewController")]
pub use self::__EKEventEditViewController::EKEventEditViewAction;
#[cfg(all(feature = "EKEventEditViewController", feature = "objc2-ui-kit"))]
pub use self::__EKEventEditViewController::EKEventEditViewController;
#[cfg(feature = "EKEventEditViewController")]
pub use self::__EKEventEditViewController::EKEventEditViewDelegate;
#[cfg(feature = "EKEventViewController")]
pub use self::__EKEventViewController::EKEventViewAction;
#[cfg(all(feature = "EKEventViewController", feature = "objc2-ui-kit"))]
pub use self::__EKEventViewController::EKEventViewController;
#[cfg(feature = "EKEventViewController")]
pub use self::__EKEventViewController::EKEventViewDelegate;
#[cfg(feature = "EventKitUIBundle")]
pub use self::__EventKitUIBundle::EventKitUIBundle;
