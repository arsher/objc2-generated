//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSHTTPCookieAcceptPolicy {
        NSHTTPCookieAcceptPolicyAlways = 0,
        NSHTTPCookieAcceptPolicyNever = 1,
        NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSHTTPCookieStorage")]
    pub struct NSHTTPCookieStorage;

    #[cfg(feature = "Foundation_NSHTTPCookieStorage")]
    unsafe impl ClassType for NSHTTPCookieStorage {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSHTTPCookieStorage")]
unsafe impl NSObjectProtocol for NSHTTPCookieStorage {}

extern_methods!(
    #[cfg(feature = "Foundation_NSHTTPCookieStorage")]
    unsafe impl NSHTTPCookieStorage {
        #[method_id(@__retain_semantics Other sharedHTTPCookieStorage)]
        pub unsafe fn sharedHTTPCookieStorage() -> Id<NSHTTPCookieStorage>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sharedCookieStorageForGroupContainerIdentifier:)]
        pub unsafe fn sharedCookieStorageForGroupContainerIdentifier(
            identifier: &NSString,
        ) -> Id<NSHTTPCookieStorage>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSHTTPCookie"))]
        #[method_id(@__retain_semantics Other cookies)]
        pub unsafe fn cookies(&self) -> Option<Id<NSArray<NSHTTPCookie>>>;

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(setCookie:)]
        pub unsafe fn setCookie(&self, cookie: &NSHTTPCookie);

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(deleteCookie:)]
        pub unsafe fn deleteCookie(&self, cookie: &NSHTTPCookie);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(removeCookiesSinceDate:)]
        pub unsafe fn removeCookiesSinceDate(&self, date: &NSDate);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other cookiesForURL:)]
        pub unsafe fn cookiesForURL(&self, url: &NSURL) -> Option<Id<NSArray<NSHTTPCookie>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURL"
        ))]
        #[method(setCookies:forURL:mainDocumentURL:)]
        pub unsafe fn setCookies_forURL_mainDocumentURL(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            url: Option<&NSURL>,
            main_document_url: Option<&NSURL>,
        );

        #[method(cookieAcceptPolicy)]
        pub unsafe fn cookieAcceptPolicy(&self) -> NSHTTPCookieAcceptPolicy;

        #[method(setCookieAcceptPolicy:)]
        pub unsafe fn setCookieAcceptPolicy(&self, cookie_accept_policy: NSHTTPCookieAcceptPolicy);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortedCookiesUsingDescriptors:)]
        pub unsafe fn sortedCookiesUsingDescriptors(
            &self,
            sort_order: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<NSHTTPCookie>>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    #[cfg(feature = "Foundation_NSHTTPCookieStorage")]
    unsafe impl NSHTTPCookieStorage {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(storeCookies:forTask:)]
        pub unsafe fn storeCookies_forTask(
            &self,
            cookies: &NSArray<NSHTTPCookie>,
            task: &NSURLSessionTask,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(getCookiesForTask:completionHandler:)]
        pub unsafe fn getCookiesForTask_completionHandler(
            &self,
            task: &NSURLSessionTask,
            completion_handler: &Block<(*mut NSArray<NSHTTPCookie>,), ()>,
        );
    }
);

extern_static!(NSHTTPCookieManagerAcceptPolicyChangedNotification: &'static NSNotificationName);

extern_static!(NSHTTPCookieManagerCookiesChangedNotification: &'static NSNotificationName);
