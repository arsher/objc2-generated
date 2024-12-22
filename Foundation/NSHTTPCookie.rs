//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiepropertykey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSHTTPCookiePropertyKey = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiestringpolicy?language=objc)
// NS_TYPED_ENUM
#[cfg(feature = "NSString")]
pub type NSHTTPCookieStringPolicy = NSString;

extern "C" {
    /// Key for cookie name
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiename?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieName: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie value
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookievalue?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieValue: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie origin URL
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookieoriginurl?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieOriginURL: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie version
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookieversion?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieVersion: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie domain
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiedomain?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieDomain: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie path
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiepath?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookiePath: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie secure flag
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiesecure?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSecure: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie expiration date
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookieexpires?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieExpires: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie comment text
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiecomment?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieComment: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie comment URL
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiecommenturl?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieCommentURL: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie discard (session-only) flag
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiediscard?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieDiscard: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie maximum age (an alternate way of specifying the expiration)
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiemaximumage?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieMaximumAge: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie ports
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookieport?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookiePort: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// An NSString object indicating that the cookie is set via JavaScript.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiesetbyjavascript?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSetByJavaScript: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// Key for cookie same site
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiesamesitepolicy?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSameSitePolicy: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    /// String constant "lax" to be used as a value for the property key NSHTTPCookieSameSite
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiesamesitelax?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSameSiteLax: &'static NSHTTPCookieStringPolicy;
}

extern "C" {
    /// String constant "strict" to be used as a value for the property key NSHTTPCookieSameSite
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookiesamesitestrict?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSameSiteStrict: &'static NSHTTPCookieStringPolicy;
}

extern_class!(
    /// NSHTTPCookie represents an http cookie.
    ///
    /// A NSHTTPCookie instance represents a single http cookie. It is
    /// an immutable object initialized from a dictionary that contains
    /// the various cookie attributes. It has accessors to get the various
    /// attributes of a cookie.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/foundation/nshttpcookie?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHTTPCookie;
);

unsafe impl Send for NSHTTPCookie {}

unsafe impl Sync for NSHTTPCookie {}

unsafe impl NSObjectProtocol for NSHTTPCookie {}

extern_methods!(
    unsafe impl NSHTTPCookie {
        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Initialize a NSHTTPCookie object with a dictionary of
        /// parameters
        ///
        /// Parameter `properties`: The dictionary of properties to be used to
        /// initialize this cookie.
        ///
        /// Supported dictionary keys and value types for the
        /// given dictionary are as follows.
        ///
        /// All properties can handle an NSString value, but some can also
        /// handle other types.
        ///
        /// <table border="1" cellspacing="2" cellpadding="4">
        /// <tr>
        /// <th>
        /// Property key constant
        /// </th>
        /// <th>
        /// Type of value
        /// </th>
        /// <th>
        /// Required
        /// </th>
        /// <th>
        /// Description
        /// </th>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieComment
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// Comment for the cookie. Only valid for version 1 cookies and
        /// later. Default is nil.
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieCommentURL
        /// </td>
        /// <td>
        /// NSURL or NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// Comment URL for the cookie. Only valid for version 1 cookies
        /// and later. Default is nil.
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieDomain
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// Special, a value for either NSHTTPCookieOriginURL or
        /// NSHTTPCookieDomain must be specified.
        /// </td>
        /// <td>
        /// Domain for the cookie. Inferred from the value for
        /// NSHTTPCookieOriginURL if not provided.
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieDiscard
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// A string stating whether the cookie should be discarded at
        /// the end of the session. String value must be either "TRUE" or
        /// "FALSE". Default is "FALSE", unless this is cookie is version
        /// 1 or greater and a value for NSHTTPCookieMaximumAge is not
        /// specified, in which case it is assumed "TRUE".
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieExpires
        /// </td>
        /// <td>
        /// NSDate or NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// Expiration date for the cookie. Used only for version 0
        /// cookies. Ignored for version 1 or greater.
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieMaximumAge
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// A string containing an integer value stating how long in
        /// seconds the cookie should be kept, at most. Only valid for
        /// version 1 cookies and later. Default is "0".
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieName
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// YES
        /// </td>
        /// <td>
        /// Name of the cookie
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieOriginURL
        /// </td>
        /// <td>
        /// NSURL or NSString
        /// </td>
        /// <td>
        /// Special, a value for either NSHTTPCookieOriginURL or
        /// NSHTTPCookieDomain must be specified.
        /// </td>
        /// <td>
        /// URL that set this cookie. Used as default for other fields
        /// as noted.
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookiePath
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// Path for the cookie. Inferred from the value for
        /// NSHTTPCookieOriginURL if not provided. Default is "/".
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookiePort
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// comma-separated integer values specifying the ports for the
        /// cookie. Only valid for version 1 cookies and later. Default is
        /// empty string ("").
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieSecure
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// A string stating whether the cookie should be transmitted
        /// only over secure channels. String value must be either "TRUE"
        /// or "FALSE". Default is "FALSE".
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieValue
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// YES
        /// </td>
        /// <td>
        /// Value of the cookie
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieVersion
        /// </td>
        /// <td>
        /// NSString
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// Specifies the version of the cookie. Must be either "0" or
        /// "1". Default is "0".
        /// </td>
        /// </tr>
        /// <tr>
        /// <td>
        /// NSHTTPCookieSetByJavaScript
        /// </td>
        /// <td>
        /// NSNumber
        /// </td>
        /// <td>
        /// NO
        /// </td>
        /// <td>
        /// `true`if the cookie is set via JavaScript.
        /// `false`if the cookie
        /// is not set via JavaScript
        /// </td>
        /// </tr>
        /// </table>
        /// <p>
        /// All other keys are ignored.
        ///
        /// Returns: An initialized NSHTTPCookie, or nil if the set of
        /// dictionary keys is invalid, for example because a required key is
        /// missing, or a recognized key maps to an illegal value.
        #[method_id(@__retain_semantics Init initWithProperties:)]
        pub unsafe fn initWithProperties(
            this: Allocated<Self>,
            properties: &NSDictionary<NSHTTPCookiePropertyKey, AnyObject>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Allocates and initializes an NSHTTPCookie with the given
        /// dictionary.
        ///
        /// See the NSHTTPCookie
        /// <tt>
        /// -initWithProperties:
        /// </tt>
        /// method for more information on the constraints imposed on the
        /// dictionary, and for descriptions of the supported keys and values.
        ///
        /// Parameter `properties`: The dictionary to use to initialize this cookie.
        ///
        /// Returns: A newly-created and autoreleased NSHTTPCookie instance, or
        /// nil if the set of dictionary keys is invalid, for example because
        /// a required key is missing, or a recognized key maps to an illegal
        /// value.
        #[method_id(@__retain_semantics Other cookieWithProperties:)]
        pub unsafe fn cookieWithProperties(
            properties: &NSDictionary<NSHTTPCookiePropertyKey, AnyObject>,
        ) -> Option<Retained<NSHTTPCookie>>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        /// Return a dictionary of header fields that can be used to add the
        /// specified cookies to the request.
        ///
        /// Parameter `cookies`: The cookies to turn into request headers.
        ///
        /// Returns: An NSDictionary where the keys are header field names, and the values
        /// are the corresponding header field values.
        #[method_id(@__retain_semantics Other requestHeaderFieldsWithCookies:)]
        pub unsafe fn requestHeaderFieldsWithCookies(
            cookies: &NSArray<NSHTTPCookie>,
        ) -> Retained<NSDictionary<NSString, NSString>>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSDictionary",
            feature = "NSString",
            feature = "NSURL"
        ))]
        /// Return an array of cookies parsed from the specified response header fields and URL.
        ///
        /// Parameter `headerFields`: The response header fields to check for cookies.
        ///
        /// Parameter `URL`: The URL that the cookies came from - relevant to how the cookies are interpreted.
        ///
        /// Returns: An NSArray of NSHTTPCookie objects
        ///
        /// This method will ignore irrelevant header fields so
        /// you can pass a dictionary containing data other than cookie data.
        #[method_id(@__retain_semantics Other cookiesWithResponseHeaderFields:forURL:)]
        pub unsafe fn cookiesWithResponseHeaderFields_forURL(
            header_fields: &NSDictionary<NSString, NSString>,
            url: &NSURL,
        ) -> Retained<NSArray<NSHTTPCookie>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        /// Returns a dictionary representation of the receiver.
        ///
        /// This method returns a dictionary representation of the
        /// NSHTTPCookie which can be saved and passed to
        /// <tt>
        /// -initWithProperties:
        /// </tt>
        /// or
        /// <tt>
        /// +cookieWithProperties:
        /// </tt>
        /// later to reconstitute an equivalent cookie.
        /// <p>
        /// See the NSHTTPCookie
        /// <tt>
        /// -initWithProperties:
        /// </tt>
        /// method for
        /// more information on the constraints imposed on the dictionary, and
        /// for descriptions of the supported keys and values.
        ///
        /// Returns: The dictionary representation of the receiver.
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(
            &self,
        ) -> Option<Retained<NSDictionary<NSHTTPCookiePropertyKey, AnyObject>>>;

        /// Returns the version of the receiver.
        ///
        /// Version 0 maps to "old-style" Netscape cookies.
        /// Version 1 maps to RFC2965 cookies. There may be future versions.
        ///
        /// Returns: the version of the receiver.
        #[method(version)]
        pub unsafe fn version(&self) -> NSUInteger;

        #[cfg(feature = "NSString")]
        /// Returns the name of the receiver.
        ///
        /// Returns: the name of the receiver.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Returns the value of the receiver.
        ///
        /// Returns: the value of the receiver.
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        #[cfg(feature = "NSDate")]
        /// Returns the expires date of the receiver.
        ///
        /// Returns: the expires date of the receiver.
        ///
        /// The expires date is the date when the cookie should be
        /// deleted. The result will be nil if there is no specific expires
        /// date. This will be the case only for "session-only" cookies.
        ///
        /// Returns: The expires date of the receiver.
        #[method_id(@__retain_semantics Other expiresDate)]
        pub unsafe fn expiresDate(&self) -> Option<Retained<NSDate>>;

        /// Returns whether the receiver is session-only.
        ///
        /// Returns: YES if this receiver should be discarded at the end of the
        /// session (regardless of expiration date), NO if receiver need not
        /// be discarded at the end of the session.
        #[method(isSessionOnly)]
        pub unsafe fn isSessionOnly(&self) -> bool;

        #[cfg(feature = "NSString")]
        /// Returns the domain of the receiver.
        ///
        /// This value specifies URL domain to which the cookie
        /// should be sent. A domain with a leading dot means the cookie
        /// should be sent to subdomains as well, assuming certain other
        /// restrictions are valid. See RFC 2965 for more detail.
        ///
        /// Returns: The domain of the receiver.
        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        /// Returns the path of the receiver.
        ///
        /// This value specifies the URL path under the cookie's
        /// domain for which this cookie should be sent. The cookie will also
        /// be sent for children of that path, so "/" is the most general.
        ///
        /// Returns: The path of the receiver.
        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Retained<NSString>;

        /// Returns whether the receiver should be sent only over
        /// secure channels
        ///
        /// Cookies may be marked secure by a server (or by a javascript).
        /// Cookies marked as such must only be sent via an encrypted connection to
        /// trusted servers (i.e. via SSL or TLS), and should not be delivered to any
        /// javascript applications to prevent cross-site scripting vulnerabilities.
        ///
        /// Returns: YES if this cookie should be sent only over secure channels,
        /// NO otherwise.
        #[method(isSecure)]
        pub unsafe fn isSecure(&self) -> bool;

        /// Returns whether the receiver should only be sent to HTTP servers
        /// per RFC 2965
        ///
        /// Cookies may be marked as HTTPOnly by a server (or by a javascript).
        /// Cookies marked as such must only be sent via HTTP Headers in HTTP Requests
        /// for URL's that match both the path and domain of the respective Cookies.
        /// Specifically these cookies should not be delivered to any javascript
        /// applications to prevent cross-site scripting vulnerabilities.
        ///
        /// Returns: YES if this cookie should only be sent via HTTP headers,
        /// NO otherwise.
        #[method(isHTTPOnly)]
        pub unsafe fn isHTTPOnly(&self) -> bool;

        #[cfg(feature = "NSString")]
        /// Returns the comment of the receiver.
        ///
        /// This value specifies a string which is suitable for
        /// presentation to the user explaining the contents and purpose of this
        /// cookie. It may be nil.
        ///
        /// Returns: The comment of the receiver, or nil if the receiver has no
        /// comment.
        #[method_id(@__retain_semantics Other comment)]
        pub unsafe fn comment(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSURL")]
        /// Returns the comment URL of the receiver.
        ///
        /// This value specifies a URL which is suitable for
        /// presentation to the user as a link for further information about
        /// this cookie. It may be nil.
        ///
        /// Returns: The comment URL of the receiver, or nil if the receiver
        /// has no comment URL.
        #[method_id(@__retain_semantics Other commentURL)]
        pub unsafe fn commentURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSArray", feature = "NSValue"))]
        /// Returns the list ports to which the receiver should be
        /// sent.
        ///
        /// This value specifies an NSArray of NSNumbers
        /// (containing integers) which specify the only ports to which this
        /// cookie should be sent.
        ///
        /// Returns: The list ports to which the receiver should be sent. The
        /// array may be nil, in which case this cookie can be sent to any
        /// port.
        #[method_id(@__retain_semantics Other portList)]
        pub unsafe fn portList(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[cfg(feature = "NSString")]
        /// Returns the value of the same site attribute on the cookie.
        ///
        /// Cookies can be marked with an attribute Strict or Lax.
        /// Cookies marked with "strict" (NSHTTPCookieSameSiteStrict) are not sent along with cross-site requests.
        /// Cookies marked with "lax" (NSHTTPCookieSameSiteLax) sent along cross-site requests provided the
        /// cross-site requests are top-level-requests (one that changes the url in the address bar).
        /// The attribute value is canonicalized and stored. Any value other than the default (strict and lax) will be ignored.
        ///
        /// Returns: strict or lax. The result could also be nil, in which case the
        /// cookie will be sent along with all cross-site requests.
        #[method_id(@__retain_semantics Other sameSitePolicy)]
        pub unsafe fn sameSitePolicy(&self) -> Option<Retained<NSHTTPCookieStringPolicy>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHTTPCookie {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
