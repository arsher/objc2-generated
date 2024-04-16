//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView",
    feature = "block2"
))]
pub type UICollectionViewCellRegistrationConfigurationHandler =
    *mut Block<dyn Fn(NonNull<UICollectionViewCell>, NonNull<NSIndexPath>, NonNull<AnyObject>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewCellRegistration;

    unsafe impl ClassType for UICollectionViewCellRegistration {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UICollectionViewCellRegistration {}

extern_methods!(
    unsafe impl UICollectionViewCellRegistration {
        #[cfg(all(
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other registrationWithCellClass:configurationHandler:)]
        pub unsafe fn registrationWithCellClass_configurationHandler(
            cell_class: &AnyClass,
            configuration_handler: UICollectionViewCellRegistrationConfigurationHandler,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "UICollectionViewCell",
            feature = "UINib",
            feature = "UIResponder",
            feature = "UIView",
            feature = "block2"
        ))]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other registrationWithCellNib:configurationHandler:)]
        pub unsafe fn registrationWithCellNib_configurationHandler(
            cell_nib: &UINib,
            configuration_handler: UICollectionViewCellRegistrationConfigurationHandler,
        ) -> Id<Self>;

        #[method(cellClass)]
        pub unsafe fn cellClass(&self) -> Option<&'static AnyClass>;

        #[cfg(feature = "UINib")]
        #[method_id(@__retain_semantics Other cellNib)]
        pub unsafe fn cellNib(&self) -> Option<Id<UINib>>;

        #[cfg(all(
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method(configurationHandler)]
        pub unsafe fn configurationHandler(
            &self,
        ) -> UICollectionViewCellRegistrationConfigurationHandler;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICollectionViewCellRegistration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

#[cfg(all(
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIView",
    feature = "block2"
))]
pub type UICollectionViewSupplementaryRegistrationConfigurationHandler =
    *mut Block<dyn Fn(NonNull<UICollectionReusableView>, NonNull<NSString>, NonNull<NSIndexPath>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewSupplementaryRegistration;

    unsafe impl ClassType for UICollectionViewSupplementaryRegistration {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UICollectionViewSupplementaryRegistration {}

extern_methods!(
    unsafe impl UICollectionViewSupplementaryRegistration {
        #[cfg(all(
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other registrationWithSupplementaryClass:elementKind:configurationHandler:)]
        pub unsafe fn registrationWithSupplementaryClass_elementKind_configurationHandler(
            supplementary_class: &AnyClass,
            element_kind: &NSString,
            configuration_handler: UICollectionViewSupplementaryRegistrationConfigurationHandler,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "UICollectionViewCell",
            feature = "UINib",
            feature = "UIResponder",
            feature = "UIView",
            feature = "block2"
        ))]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other registrationWithSupplementaryNib:elementKind:configurationHandler:)]
        pub unsafe fn registrationWithSupplementaryNib_elementKind_configurationHandler(
            supplementary_nib: &UINib,
            element_kind: &NSString,
            configuration_handler: UICollectionViewSupplementaryRegistrationConfigurationHandler,
        ) -> Id<Self>;

        #[method(supplementaryClass)]
        pub unsafe fn supplementaryClass(&self) -> Option<&'static AnyClass>;

        #[cfg(feature = "UINib")]
        #[method_id(@__retain_semantics Other supplementaryNib)]
        pub unsafe fn supplementaryNib(&self) -> Option<Id<UINib>>;

        #[method_id(@__retain_semantics Other elementKind)]
        pub unsafe fn elementKind(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method(configurationHandler)]
        pub unsafe fn configurationHandler(
            &self,
        ) -> UICollectionViewSupplementaryRegistrationConfigurationHandler;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICollectionViewSupplementaryRegistration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
