//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitCollection;

    unsafe impl ClassType for UITraitCollection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for UITraitCollection {}

unsafe impl Sync for UITraitCollection {}

unsafe impl NSCoding for UITraitCollection {}

unsafe impl NSCopying for UITraitCollection {}

unsafe impl NSObjectProtocol for UITraitCollection {}

unsafe impl NSSecureCoding for UITraitCollection {}

extern_methods!(
    unsafe impl UITraitCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[deprecated = "Compare values for specific traits in the trait collections instead"]
        #[method(containsTraitsInCollection:)]
        pub unsafe fn containsTraitsInCollection(
            &self,
            r#trait: Option<&UITraitCollection>,
        ) -> bool;

        #[deprecated = "Use +[UITraitCollection traitCollectionWithTraits:] and -[UITraitCollection traitCollectionByModifyingTraits:] to create and modify trait collections"]
        #[method_id(@__retain_semantics Other traitCollectionWithTraitsFromCollections:)]
        pub unsafe fn traitCollectionWithTraitsFromCollections(
            trait_collections: &NSArray<UITraitCollection>,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIDevice")]
        #[method_id(@__retain_semantics Other traitCollectionWithUserInterfaceIdiom:)]
        pub unsafe fn traitCollectionWithUserInterfaceIdiom(
            idiom: UIUserInterfaceIdiom,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIDevice")]
        #[method(userInterfaceIdiom)]
        pub unsafe fn userInterfaceIdiom(&self) -> UIUserInterfaceIdiom;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithUserInterfaceStyle:)]
        pub unsafe fn traitCollectionWithUserInterfaceStyle(
            user_interface_style: UIUserInterfaceStyle,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(userInterfaceStyle)]
        pub unsafe fn userInterfaceStyle(&self) -> UIUserInterfaceStyle;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithLayoutDirection:)]
        pub unsafe fn traitCollectionWithLayoutDirection(
            layout_direction: UITraitEnvironmentLayoutDirection,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(layoutDirection)]
        pub unsafe fn layoutDirection(&self) -> UITraitEnvironmentLayoutDirection;

        #[method_id(@__retain_semantics Other traitCollectionWithDisplayScale:)]
        pub unsafe fn traitCollectionWithDisplayScale(scale: CGFloat) -> Id<UITraitCollection>;

        #[method(displayScale)]
        pub unsafe fn displayScale(&self) -> CGFloat;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithHorizontalSizeClass:)]
        pub unsafe fn traitCollectionWithHorizontalSizeClass(
            horizontal_size_class: UIUserInterfaceSizeClass,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(horizontalSizeClass)]
        pub unsafe fn horizontalSizeClass(&self) -> UIUserInterfaceSizeClass;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithVerticalSizeClass:)]
        pub unsafe fn traitCollectionWithVerticalSizeClass(
            vertical_size_class: UIUserInterfaceSizeClass,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(verticalSizeClass)]
        pub unsafe fn verticalSizeClass(&self) -> UIUserInterfaceSizeClass;

        #[cfg(feature = "UITouch")]
        #[method_id(@__retain_semantics Other traitCollectionWithForceTouchCapability:)]
        pub unsafe fn traitCollectionWithForceTouchCapability(
            capability: UIForceTouchCapability,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITouch")]
        #[method(forceTouchCapability)]
        pub unsafe fn forceTouchCapability(&self) -> UIForceTouchCapability;

        #[cfg(feature = "UIContentSizeCategory")]
        #[method_id(@__retain_semantics Other traitCollectionWithPreferredContentSizeCategory:)]
        pub unsafe fn traitCollectionWithPreferredContentSizeCategory(
            preferred_content_size_category: &UIContentSizeCategory,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIContentSizeCategory")]
        #[method_id(@__retain_semantics Other preferredContentSizeCategory)]
        pub unsafe fn preferredContentSizeCategory(&self) -> Id<UIContentSizeCategory>;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithDisplayGamut:)]
        pub unsafe fn traitCollectionWithDisplayGamut(
            display_gamut: UIDisplayGamut,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(displayGamut)]
        pub unsafe fn displayGamut(&self) -> UIDisplayGamut;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithAccessibilityContrast:)]
        pub unsafe fn traitCollectionWithAccessibilityContrast(
            accessibility_contrast: UIAccessibilityContrast,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(accessibilityContrast)]
        pub unsafe fn accessibilityContrast(&self) -> UIAccessibilityContrast;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithUserInterfaceLevel:)]
        pub unsafe fn traitCollectionWithUserInterfaceLevel(
            user_interface_level: UIUserInterfaceLevel,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(userInterfaceLevel)]
        pub unsafe fn userInterfaceLevel(&self) -> UIUserInterfaceLevel;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithLegibilityWeight:)]
        pub unsafe fn traitCollectionWithLegibilityWeight(
            legibility_weight: UILegibilityWeight,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(legibilityWeight)]
        pub unsafe fn legibilityWeight(&self) -> UILegibilityWeight;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithActiveAppearance:)]
        pub unsafe fn traitCollectionWithActiveAppearance(
            user_interface_active_appearance: UIUserInterfaceActiveAppearance,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(activeAppearance)]
        pub unsafe fn activeAppearance(&self) -> UIUserInterfaceActiveAppearance;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithToolbarItemPresentationSize:)]
        pub unsafe fn traitCollectionWithToolbarItemPresentationSize(
            toolbar_item_presentation_size: UINSToolbarItemPresentationSize,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(toolbarItemPresentationSize)]
        pub unsafe fn toolbarItemPresentationSize(&self) -> UINSToolbarItemPresentationSize;

        #[cfg(feature = "UIInterface")]
        #[method_id(@__retain_semantics Other traitCollectionWithImageDynamicRange:)]
        pub unsafe fn traitCollectionWithImageDynamicRange(
            image_dynamic_range: UIImageDynamicRange,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UIInterface")]
        #[method(imageDynamicRange)]
        pub unsafe fn imageDynamicRange(&self) -> UIImageDynamicRange;

        #[method_id(@__retain_semantics Other traitCollectionWithTypesettingLanguage:)]
        pub unsafe fn traitCollectionWithTypesettingLanguage(
            language: &NSString,
        ) -> Id<UITraitCollection>;

        #[method_id(@__retain_semantics Other typesettingLanguage)]
        pub unsafe fn typesettingLanguage(&self) -> Id<NSString>;

        #[cfg(feature = "UISceneDefinitions")]
        #[method_id(@__retain_semantics Other traitCollectionWithSceneCaptureState:)]
        pub unsafe fn traitCollectionWithSceneCaptureState(
            scene_capture_state: UISceneCaptureState,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UISceneDefinitions")]
        #[method(sceneCaptureState)]
        pub unsafe fn sceneCaptureState(&self) -> UISceneCaptureState;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitCollection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UIMutableTraits: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "UITrait")]
        #[method(setCGFloatValue:forTrait:)]
        unsafe fn setCGFloatValue_forTrait(&self, value: CGFloat, r#trait: UICGFloatTrait);

        #[cfg(feature = "UITrait")]
        #[method(valueForCGFloatTrait:)]
        unsafe fn valueForCGFloatTrait(&self, r#trait: UICGFloatTrait) -> CGFloat;

        #[cfg(feature = "UITrait")]
        #[method(setNSIntegerValue:forTrait:)]
        unsafe fn setNSIntegerValue_forTrait(&self, value: NSInteger, r#trait: UINSIntegerTrait);

        #[cfg(feature = "UITrait")]
        #[method(valueForNSIntegerTrait:)]
        unsafe fn valueForNSIntegerTrait(&self, r#trait: UINSIntegerTrait) -> NSInteger;

        #[cfg(feature = "UITrait")]
        #[method(setObject:forTrait:)]
        unsafe fn setObject_forTrait(&self, object: Option<&NSObject>, r#trait: UIObjectTrait);

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other objectForTrait:)]
        unsafe fn objectForTrait(&self, r#trait: UIObjectTrait) -> Option<Id<NSObject>>;

        #[cfg(feature = "UIDevice")]
        #[method(userInterfaceIdiom)]
        unsafe fn userInterfaceIdiom(&self) -> UIUserInterfaceIdiom;

        #[cfg(feature = "UIDevice")]
        #[method(setUserInterfaceIdiom:)]
        unsafe fn setUserInterfaceIdiom(&self, user_interface_idiom: UIUserInterfaceIdiom);

        #[cfg(feature = "UIInterface")]
        #[method(userInterfaceStyle)]
        unsafe fn userInterfaceStyle(&self) -> UIUserInterfaceStyle;

        #[cfg(feature = "UIInterface")]
        #[method(setUserInterfaceStyle:)]
        unsafe fn setUserInterfaceStyle(&self, user_interface_style: UIUserInterfaceStyle);

        #[cfg(feature = "UIInterface")]
        #[method(layoutDirection)]
        unsafe fn layoutDirection(&self) -> UITraitEnvironmentLayoutDirection;

        #[cfg(feature = "UIInterface")]
        #[method(setLayoutDirection:)]
        unsafe fn setLayoutDirection(&self, layout_direction: UITraitEnvironmentLayoutDirection);

        #[method(displayScale)]
        unsafe fn displayScale(&self) -> CGFloat;

        #[method(setDisplayScale:)]
        unsafe fn setDisplayScale(&self, display_scale: CGFloat);

        #[cfg(feature = "UIInterface")]
        #[method(horizontalSizeClass)]
        unsafe fn horizontalSizeClass(&self) -> UIUserInterfaceSizeClass;

        #[cfg(feature = "UIInterface")]
        #[method(setHorizontalSizeClass:)]
        unsafe fn setHorizontalSizeClass(&self, horizontal_size_class: UIUserInterfaceSizeClass);

        #[cfg(feature = "UIInterface")]
        #[method(verticalSizeClass)]
        unsafe fn verticalSizeClass(&self) -> UIUserInterfaceSizeClass;

        #[cfg(feature = "UIInterface")]
        #[method(setVerticalSizeClass:)]
        unsafe fn setVerticalSizeClass(&self, vertical_size_class: UIUserInterfaceSizeClass);

        #[cfg(feature = "UITouch")]
        #[method(forceTouchCapability)]
        unsafe fn forceTouchCapability(&self) -> UIForceTouchCapability;

        #[cfg(feature = "UITouch")]
        #[method(setForceTouchCapability:)]
        unsafe fn setForceTouchCapability(&self, force_touch_capability: UIForceTouchCapability);

        #[cfg(feature = "UIContentSizeCategory")]
        #[method_id(@__retain_semantics Other preferredContentSizeCategory)]
        unsafe fn preferredContentSizeCategory(&self) -> Id<UIContentSizeCategory>;

        #[cfg(feature = "UIContentSizeCategory")]
        #[method(setPreferredContentSizeCategory:)]
        unsafe fn setPreferredContentSizeCategory(
            &self,
            preferred_content_size_category: &UIContentSizeCategory,
        );

        #[cfg(feature = "UIInterface")]
        #[method(displayGamut)]
        unsafe fn displayGamut(&self) -> UIDisplayGamut;

        #[cfg(feature = "UIInterface")]
        #[method(setDisplayGamut:)]
        unsafe fn setDisplayGamut(&self, display_gamut: UIDisplayGamut);

        #[cfg(feature = "UIInterface")]
        #[method(accessibilityContrast)]
        unsafe fn accessibilityContrast(&self) -> UIAccessibilityContrast;

        #[cfg(feature = "UIInterface")]
        #[method(setAccessibilityContrast:)]
        unsafe fn setAccessibilityContrast(&self, accessibility_contrast: UIAccessibilityContrast);

        #[cfg(feature = "UIInterface")]
        #[method(userInterfaceLevel)]
        unsafe fn userInterfaceLevel(&self) -> UIUserInterfaceLevel;

        #[cfg(feature = "UIInterface")]
        #[method(setUserInterfaceLevel:)]
        unsafe fn setUserInterfaceLevel(&self, user_interface_level: UIUserInterfaceLevel);

        #[cfg(feature = "UIInterface")]
        #[method(legibilityWeight)]
        unsafe fn legibilityWeight(&self) -> UILegibilityWeight;

        #[cfg(feature = "UIInterface")]
        #[method(setLegibilityWeight:)]
        unsafe fn setLegibilityWeight(&self, legibility_weight: UILegibilityWeight);

        #[cfg(feature = "UIInterface")]
        #[method(activeAppearance)]
        unsafe fn activeAppearance(&self) -> UIUserInterfaceActiveAppearance;

        #[cfg(feature = "UIInterface")]
        #[method(setActiveAppearance:)]
        unsafe fn setActiveAppearance(&self, active_appearance: UIUserInterfaceActiveAppearance);

        #[cfg(feature = "UIInterface")]
        #[method(toolbarItemPresentationSize)]
        unsafe fn toolbarItemPresentationSize(&self) -> UINSToolbarItemPresentationSize;

        #[cfg(feature = "UIInterface")]
        #[method(setToolbarItemPresentationSize:)]
        unsafe fn setToolbarItemPresentationSize(
            &self,
            toolbar_item_presentation_size: UINSToolbarItemPresentationSize,
        );

        #[cfg(feature = "UIInterface")]
        #[method(imageDynamicRange)]
        unsafe fn imageDynamicRange(&self) -> UIImageDynamicRange;

        #[cfg(feature = "UIInterface")]
        #[method(setImageDynamicRange:)]
        unsafe fn setImageDynamicRange(&self, image_dynamic_range: UIImageDynamicRange);

        #[cfg(feature = "UISceneDefinitions")]
        #[method(sceneCaptureState)]
        unsafe fn sceneCaptureState(&self) -> UISceneCaptureState;

        #[cfg(feature = "UISceneDefinitions")]
        #[method(setSceneCaptureState:)]
        unsafe fn setSceneCaptureState(&self, scene_capture_state: UISceneCaptureState);

        #[method_id(@__retain_semantics Other typesettingLanguage)]
        unsafe fn typesettingLanguage(&self) -> Id<NSString>;

        #[method(setTypesettingLanguage:)]
        unsafe fn setTypesettingLanguage(&self, typesetting_language: &NSString);
    }

    unsafe impl ProtocolType for dyn UIMutableTraits {}
);

#[cfg(feature = "block2")]
pub type UITraitMutations =
    *mut block2::Block<dyn Fn(NonNull<ProtocolObject<dyn UIMutableTraits>>)>;

extern_methods!(
    unsafe impl UITraitCollection {
        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other traitCollectionWithTraits:)]
        pub unsafe fn traitCollectionWithTraits(
            mutations: UITraitMutations,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other traitCollectionByModifyingTraits:)]
        pub unsafe fn traitCollectionByModifyingTraits(
            &self,
            mutations: UITraitMutations,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other traitCollectionWithCGFloatValue:forTrait:)]
        pub unsafe fn traitCollectionWithCGFloatValue_forTrait(
            value: CGFloat,
            r#trait: UICGFloatTrait,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other traitCollectionByReplacingCGFloatValue:forTrait:)]
        pub unsafe fn traitCollectionByReplacingCGFloatValue_forTrait(
            &self,
            value: CGFloat,
            r#trait: UICGFloatTrait,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method(valueForCGFloatTrait:)]
        pub unsafe fn valueForCGFloatTrait(&self, r#trait: UICGFloatTrait) -> CGFloat;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other traitCollectionWithNSIntegerValue:forTrait:)]
        pub unsafe fn traitCollectionWithNSIntegerValue_forTrait(
            value: NSInteger,
            r#trait: UINSIntegerTrait,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other traitCollectionByReplacingNSIntegerValue:forTrait:)]
        pub unsafe fn traitCollectionByReplacingNSIntegerValue_forTrait(
            &self,
            value: NSInteger,
            r#trait: UINSIntegerTrait,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method(valueForNSIntegerTrait:)]
        pub unsafe fn valueForNSIntegerTrait(&self, r#trait: UINSIntegerTrait) -> NSInteger;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other traitCollectionWithObject:forTrait:)]
        pub unsafe fn traitCollectionWithObject_forTrait(
            object: Option<&NSObject>,
            r#trait: UIObjectTrait,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other traitCollectionByReplacingObject:forTrait:)]
        pub unsafe fn traitCollectionByReplacingObject_forTrait(
            &self,
            object: Option<&NSObject>,
            r#trait: UIObjectTrait,
        ) -> Id<UITraitCollection>;

        #[cfg(feature = "UITrait")]
        #[method_id(@__retain_semantics Other objectForTrait:)]
        pub unsafe fn objectForTrait(&self, r#trait: UIObjectTrait) -> Option<Id<NSObject>>;
    }
);

extern_protocol!(
    pub unsafe trait UITraitEnvironment: NSObjectProtocol + IsMainThreadOnly {
        #[method_id(@__retain_semantics Other traitCollection)]
        unsafe fn traitCollection(&self) -> Id<UITraitCollection>;

        #[deprecated = "Use the trait change registration APIs declared in the UITraitChangeObservable protocol"]
        #[method(traitCollectionDidChange:)]
        unsafe fn traitCollectionDidChange(
            &self,
            previous_trait_collection: Option<&UITraitCollection>,
        );
    }

    unsafe impl ProtocolType for dyn UITraitEnvironment {}
);

extern_protocol!(
    pub unsafe trait UITraitChangeRegistration:
        NSCopying + NSObjectProtocol + IsMainThreadOnly
    {
    }

    unsafe impl ProtocolType for dyn UITraitChangeRegistration {}
);

#[cfg(feature = "block2")]
pub type UITraitChangeHandler = *mut block2::Block<
    dyn Fn(NonNull<ProtocolObject<dyn UITraitEnvironment>>, NonNull<UITraitCollection>),
>;

extern_protocol!(
    pub unsafe trait UITraitOverrides: UIMutableTraits + IsMainThreadOnly {
        #[cfg(feature = "UITrait")]
        #[method(containsTrait:)]
        unsafe fn containsTrait(&self, r#trait: UITrait) -> bool;

        #[cfg(feature = "UITrait")]
        #[method(removeTrait:)]
        unsafe fn removeTrait(&self, r#trait: UITrait);
    }

    unsafe impl ProtocolType for dyn UITraitOverrides {}
);

extern_protocol!(
    pub unsafe trait UITraitChangeObservable: IsMainThreadOnly {
        #[method(unregisterForTraitChanges:)]
        unsafe fn unregisterForTraitChanges(
            &self,
            registration: &ProtocolObject<dyn UITraitChangeRegistration>,
        );
    }

    unsafe impl ProtocolType for dyn UITraitChangeObservable {}
);

extern_methods!(
    /// CurrentTraitCollection
    unsafe impl UITraitCollection {
        #[method_id(@__retain_semantics Other currentTraitCollection)]
        pub unsafe fn currentTraitCollection() -> Id<UITraitCollection>;

        #[method(setCurrentTraitCollection:)]
        pub unsafe fn setCurrentTraitCollection(current_trait_collection: &UITraitCollection);

        #[cfg(feature = "block2")]
        #[method(performAsCurrentTraitCollection:)]
        pub unsafe fn performAsCurrentTraitCollection(
            &self,
            actions: &block2::Block<dyn Fn() + '_>,
        );
    }
);

extern_methods!(
    /// DynamicAppearance
    unsafe impl UITraitCollection {
        #[method(hasDifferentColorAppearanceComparedToTraitCollection:)]
        pub unsafe fn hasDifferentColorAppearanceComparedToTraitCollection(
            &self,
            trait_collection: Option<&UITraitCollection>,
        ) -> bool;
    }
);

extern_methods!(
    /// ImageConfiguration
    unsafe impl UITraitCollection {
        #[cfg(feature = "UIImageConfiguration")]
        #[method_id(@__retain_semantics Other imageConfiguration)]
        pub unsafe fn imageConfiguration(&self) -> Id<UIImageConfiguration>;
    }
);
