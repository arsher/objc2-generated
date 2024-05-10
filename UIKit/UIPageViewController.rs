//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPageViewControllerNavigationOrientation(pub NSInteger);
impl UIPageViewControllerNavigationOrientation {
    #[doc(alias = "UIPageViewControllerNavigationOrientationHorizontal")]
    pub const Horizontal: Self = Self(0);
    #[doc(alias = "UIPageViewControllerNavigationOrientationVertical")]
    pub const Vertical: Self = Self(1);
}

unsafe impl Encode for UIPageViewControllerNavigationOrientation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPageViewControllerNavigationOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPageViewControllerSpineLocation(pub NSInteger);
impl UIPageViewControllerSpineLocation {
    #[doc(alias = "UIPageViewControllerSpineLocationNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIPageViewControllerSpineLocationMin")]
    pub const Min: Self = Self(1);
    #[doc(alias = "UIPageViewControllerSpineLocationMid")]
    pub const Mid: Self = Self(2);
    #[doc(alias = "UIPageViewControllerSpineLocationMax")]
    pub const Max: Self = Self(3);
}

unsafe impl Encode for UIPageViewControllerSpineLocation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPageViewControllerSpineLocation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPageViewControllerNavigationDirection(pub NSInteger);
impl UIPageViewControllerNavigationDirection {
    #[doc(alias = "UIPageViewControllerNavigationDirectionForward")]
    pub const Forward: Self = Self(0);
    #[doc(alias = "UIPageViewControllerNavigationDirectionReverse")]
    pub const Reverse: Self = Self(1);
}

unsafe impl Encode for UIPageViewControllerNavigationDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPageViewControllerNavigationDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPageViewControllerTransitionStyle(pub NSInteger);
impl UIPageViewControllerTransitionStyle {
    #[doc(alias = "UIPageViewControllerTransitionStylePageCurl")]
    pub const PageCurl: Self = Self(0);
    #[doc(alias = "UIPageViewControllerTransitionStyleScroll")]
    pub const Scroll: Self = Self(1);
}

unsafe impl Encode for UIPageViewControllerTransitionStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPageViewControllerTransitionStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type UIPageViewControllerOptionsKey = NSString;

extern "C" {
    pub static UIPageViewControllerOptionSpineLocationKey: &'static UIPageViewControllerOptionsKey;
}

extern "C" {
    pub static UIPageViewControllerOptionInterPageSpacingKey:
        &'static UIPageViewControllerOptionsKey;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIPageViewController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UIPageViewController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIPageViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIPageViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIPageViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIPageViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIPageViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIPageViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIPageViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIPageViewController {
        #[method_id(@__retain_semantics Init initWithTransitionStyle:navigationOrientation:options:)]
        pub unsafe fn initWithTransitionStyle_navigationOrientation_options(
            this: Allocated<Self>,
            style: UIPageViewControllerTransitionStyle,
            navigation_orientation: UIPageViewControllerNavigationOrientation,
            options: Option<&NSDictionary<UIPageViewControllerOptionsKey, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UIPageViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPageViewControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UIPageViewControllerDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn UIPageViewControllerDataSource>>,
        );

        #[method(transitionStyle)]
        pub unsafe fn transitionStyle(&self) -> UIPageViewControllerTransitionStyle;

        #[method(navigationOrientation)]
        pub unsafe fn navigationOrientation(&self) -> UIPageViewControllerNavigationOrientation;

        #[method(spineLocation)]
        pub unsafe fn spineLocation(&self) -> UIPageViewControllerSpineLocation;

        #[method(isDoubleSided)]
        pub unsafe fn isDoubleSided(&self) -> bool;

        #[method(setDoubleSided:)]
        pub unsafe fn setDoubleSided(&self, double_sided: bool);

        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gestureRecognizers)]
        pub unsafe fn gestureRecognizers(&self) -> Id<NSArray<UIGestureRecognizer>>;

        #[method_id(@__retain_semantics Other viewControllers)]
        pub unsafe fn viewControllers(&self) -> Option<Id<NSArray<UIViewController>>>;

        #[cfg(feature = "block2")]
        #[method(setViewControllers:direction:animated:completion:)]
        pub unsafe fn setViewControllers_direction_animated_completion(
            &self,
            view_controllers: Option<&NSArray<UIViewController>>,
            direction: UIPageViewControllerNavigationDirection,
            animated: bool,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIPageViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIPageViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UIPageViewControllerDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(pageViewController:willTransitionToViewControllers:)]
        unsafe fn pageViewController_willTransitionToViewControllers(
            &self,
            page_view_controller: &UIPageViewController,
            pending_view_controllers: &NSArray<UIViewController>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(pageViewController:didFinishAnimating:previousViewControllers:transitionCompleted:)]
        unsafe fn pageViewController_didFinishAnimating_previousViewControllers_transitionCompleted(
            &self,
            page_view_controller: &UIPageViewController,
            finished: bool,
            previous_view_controllers: &NSArray<UIViewController>,
            completed: bool,
        );

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(pageViewController:spineLocationForInterfaceOrientation:)]
        unsafe fn pageViewController_spineLocationForInterfaceOrientation(
            &self,
            page_view_controller: &UIPageViewController,
            orientation: UIInterfaceOrientation,
        ) -> UIPageViewControllerSpineLocation;

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(pageViewControllerSupportedInterfaceOrientations:)]
        unsafe fn pageViewControllerSupportedInterfaceOrientations(
            &self,
            page_view_controller: &UIPageViewController,
        ) -> UIInterfaceOrientationMask;

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(pageViewControllerPreferredInterfaceOrientationForPresentation:)]
        unsafe fn pageViewControllerPreferredInterfaceOrientationForPresentation(
            &self,
            page_view_controller: &UIPageViewController,
        ) -> UIInterfaceOrientation;
    }

    unsafe impl ProtocolType for dyn UIPageViewControllerDelegate {}
);

extern_protocol!(
    pub unsafe trait UIPageViewControllerDataSource:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other pageViewController:viewControllerBeforeViewController:)]
        unsafe fn pageViewController_viewControllerBeforeViewController(
            &self,
            page_view_controller: &UIPageViewController,
            view_controller: &UIViewController,
        ) -> Option<Id<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other pageViewController:viewControllerAfterViewController:)]
        unsafe fn pageViewController_viewControllerAfterViewController(
            &self,
            page_view_controller: &UIPageViewController,
            view_controller: &UIViewController,
        ) -> Option<Id<UIViewController>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(presentationCountForPageViewController:)]
        unsafe fn presentationCountForPageViewController(
            &self,
            page_view_controller: &UIPageViewController,
        ) -> NSInteger;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(presentationIndexForPageViewController:)]
        unsafe fn presentationIndexForPageViewController(
            &self,
            page_view_controller: &UIPageViewController,
        ) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn UIPageViewControllerDataSource {}
);
