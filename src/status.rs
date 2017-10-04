//! Custom responders which allow arbitrary status codes.
//!
//! Rocket provides useful defaults for static content: just respond
//! with any type which implements Responder, and a it will be returned
//! to the client in a sensible manner with a 200 status code. You can
//! return an `Option<R: Responder>`, in which `None` maps to a default
//! 404 status code, or even `Result<R: Responder, E: Responder>`, in which
//! you can manually set the status codes on both types by using appropriate
//! responder wrappers. However, that's not sufficiently flexible for REST.
//!
//! Consider resource creation: You want to return 201 on success, or 400 on
//! invalid user input, or 500 on a real ISE such as database failure.
//! Returning `Result<status::Created<R>, status::BadRequest<R>>`
//! gets you most of the way there, but strips you of the ability to propagate
//! actual internal server errors.
//!
//! We need something more flexible, so we implement it here.
//!
//! The recommended usage is to use the `Status` enum to wrap return types
//! in arbitrary status codes, and the `status!` macro to generate the appropriate
//! types.
//!
//! # Example
//! ```rust
//! use status::*;
//! use rocket::http::hyper::header::Location;
//!
//! #[get("/status/<code>")]
//! fn status(code: u16) -> Status<&'static str> {
//!     match code {
//!         100 => status!(Continue, "100 Continue"),
//!         200 => status!(Ok, "200 Ok"),
//!         204 => status!(NoContent),
//!         300 => status!(MultipleChoices, "300 Multiple Choices"),
//!         301 => {
//!             status!(
//!                 MovedPermanently,
//!                 Location(String::from("/redirect")),
//!                 "301 Moved Permanently"
//!             )
//!         }
//!         400 => status!(BadRequest, "400 Bad Request"),
//!         _ => status!(InternalServerError, "500 Internal Server Error"),
//!     }
//! }
//! ```

pub use rocket::response::status::*;
use rocket::http::Status as HttpStatus;
use rocket::http::hyper::header::Location;
use rocket::request::Request;
use rocket::response::{Response, Responder};

/// This macro, based on the implementation of `rocket::response::status::Custom`,
/// simplifies the quick implementation of status codes which do nothing but set
/// the status of the response.
///
/// Note that the name assigned must match a const in `rocket::http::Status`.
macro_rules! bare_status {
        ($(#[$attr:meta])* $name:ident) => {
            $(#[$attr])* pub struct $name<R>(pub R);
            impl<'r, R: Responder<'r>> Responder<'r> for $name<R> {
                fn respond_to(self, req: &Request) -> Result<Response<'r>, HttpStatus> {
                    Response::build_from(self.0.respond_to(req)?)
                        .status(HttpStatus::$name)
                        .ok()
                }
            }
        }
    }

/// This macro simplifies quick implementation of status codes which require
/// the use of the `Location` header.
///
/// Note that the name assigned must match a const in `rocket::http::Status`.
macro_rules! status_loc {
        ($(#[$attr:meta])* $name:ident) => {
            $(#[$attr])* pub struct $name<R>(pub Location, pub R);
            impl<'r, R: Responder<'r>> Responder<'r> for $name<R> {
                fn respond_to(self, req: &Request) -> Result<Response<'r>, HttpStatus> {
                    Response::build_from(self.1.respond_to(req)?)
                        .status(HttpStatus::$name)
                        .header(self.0)
                        .ok()
                }
            }
        }
    }

// HTTP Status Codes
//
// Those codes already implemented by `rocket::response::status` are excluded.

bare_status!(
    /// Sets the status of the response to 100 (Continue)
    Continue
);
bare_status!(
    /// Sets the status of the response to 101 (Switching Protocols)
    SwitchingProtocols
);
bare_status!(
    /// Sets the status of the response to 102 (Processing)
    Processing
);
bare_status!(
    /// Sets the status of the response to 200 (OK)
    Ok
);
bare_status!(
    /// Sets the status of the response to 203 ((Non-Authoritative Information))
    NonAuthoritativeInformation
);
bare_status!(
    /// Sets the status of the response to 206 ((Partial Content))
    PartialContent
);
bare_status!(
    /// Sets the status of the response to 207 (Multi-Status)
    MultiStatus
);
bare_status!(
    /// Sets the status of the response to 208 (Already Reported)
    AlreadyReported
);
bare_status!(
    /// Sets the status of the response to 226 (IM Used)
    ImUsed
);
bare_status!(
    /// Sets the status of the response to 300 (Multiple Choices)
    MultipleChoices
);
status_loc!(
    /// Sets the status of the response to 301 (Moved Permanently)
    MovedPermanently
);
status_loc!(
    /// Sets the status of the response to 302 (Found)
    Found
);
status_loc!(
    /// Sets the status of the response to 303 (See Other)
    SeeOther
);
bare_status!(
    /// Sets the status of the response to 304 (Not Modified)
    NotModified
);
bare_status!(
    /// Sets the status of the response to 305 (Use Proxy)
    UseProxy
);
status_loc!(
    /// Sets the status of the response to 307 (Temporary Redirect)
    TemporaryRedirect
);
status_loc!(
    /// Sets the status of the response to 308 (Permanent Redirect)
    PermanentRedirect
);
bare_status!(
    /// Sets the status of the response to 400 (Bad Request)
    BadRequest
);
/// Sets the status of the response to 401 (Unauthorized)
///
/// Unlike many of the rest of these structs, this one takes two parameters:
/// the first is an `&str` which contains the `WWW-Authenticate` header content,
/// and the second is the responder.
///
/// It is the responsibility of the user to provide a valid `WWW-Authenticate`
/// header string; no attempt is made to build one automatically.
pub struct Unauthorized<R>(pub String, pub R);
impl<'r, R: Responder<'r>> Responder<'r> for Unauthorized<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, HttpStatus> {
        Response::build_from(self.1.respond_to(req)?)
            .status(HttpStatus::Unauthorized)
            .raw_header("WWW-Authenticate", self.0)
            .ok()
    }
}
bare_status!(
    /// Sets the status of the response to 402 (Payment Required)
    PaymentRequired
);
bare_status!(
    /// Sets the status of the response to 403 (Forbidden)
    Forbidden
);
bare_status!(
    /// Sets the status of the response to 405 (Method Not Allowed)
    MethodNotAllowed
);
bare_status!(
    /// Sets the status of the response to 406 (Not Acceptable)
    NotAcceptable
);
bare_status!(
    /// Sets the status of the response to 407 (Proxy Authentication Required)
    ProxyAuthenticationRequired
);
bare_status!(
    /// Sets the status of the response to 408 (Request Timeout)
    RequestTimeout
);
bare_status!(
    /// Sets the status of the response to 409 (Conflict)
    Conflict
);
bare_status!(
    /// Sets the status of the response to 410 (Gone)
    Gone
);
bare_status!(
    /// Sets the status of the response to 411 (Length Required)
    LengthRequired
);
bare_status!(
    /// Sets the status of the response to 412 (Precondition Failed)
    PreconditionFailed
);
bare_status!(
    /// Sets the status of the response to 413 (Payload Too Large)
    PayloadTooLarge
);
bare_status!(
    /// Sets the status of the response to 414 (URI Too Long)
    UriTooLong
);
bare_status!(
    /// Sets the status of the response to 415 (Unsupported Media Type)
    UnsupportedMediaType
);
bare_status!(
    /// Sets the status of the response to 416 (Range Not Satisfiable)
    RangeNotSatisfiable
);
bare_status!(
    /// Sets the status of the response to 417 (Expectation Failed)
    ExpectationFailed
);
bare_status!(
    /// Sets the status of the response to 418 (I'm a teapot)
    ImATeapot
);
bare_status!(
    /// Sets the status of the response to 421 (Misdirected Request)
    MisdirectedRequest
);
bare_status!(
    /// Sets the status of the response to 422 (Unprocessable Entity)
    UnprocessableEntity
);
bare_status!(
    /// Sets the status of the response to 423 (Locked)
    Locked
);
bare_status!(
    /// Sets the status of the response to 424 (Failed Dependency)
    FailedDependency
);
bare_status!(
    /// Sets the status of the response to 426 (Upgrade Required)
    UpgradeRequired
);
bare_status!(
    /// Sets the status of the response to 428 (Precondition Required)
    PreconditionRequired
);
bare_status!(
    /// Sets the status of the response to 429 (Too Many Requests)
    TooManyRequests
);
bare_status!(
    /// Sets the status of the response to 431 (Request Header Fields Too Large)
    RequestHeaderFieldsTooLarge
);
bare_status!(
    /// Sets the status of the response to 451 (Unavailable For Legal Reasons)
    UnavailableForLegalReasons
);
bare_status!(
    /// Sets the status of the response to 500 (Internal Server Error)
    InternalServerError
);
bare_status!(
    /// Sets the status of the response to 501 (Not Implemented)
    NotImplemented
);
bare_status!(
    /// Sets the status of the response to 502 (Bad Gateway)
    BadGateway
);
bare_status!(
    /// Sets the status of the response to 503 (Service Unavailable)
    ServiceUnavailable
);
bare_status!(
    /// Sets the status of the response to 504 (Gateway Timeout)
    GatewayTimeout
);
bare_status!(
    /// Sets the status of the response to 505 (HTTP Version Not Supported)
    HttpVersionNotSupported
);
bare_status!(
    /// Sets the status of the response to 506 (Variant Also Negotiates)
    VariantAlsoNegotiates
);
bare_status!(
    /// Sets the status of the response to 507 (Insufficient Storage)
    InsufficientStorage
);
bare_status!(
    /// Sets the status of the response to 508 (Loop Detected)
    LoopDetected
);
bare_status!(
    /// Sets the status of the response to 510 (Not Extended)
    NotExtended
);
bare_status!(
    /// Sets the status of the response to 511 (Network Authentication Required)
    NetworkAuthenticationRequired
);


/// Convenience macro to simplify use of the `Status` enum.
///
/// First parameter is the full name of the status code.
/// Second parameter is the wrapped content which will be returned
/// with the given status code.
///
/// Note that some status codes  don't accept content
/// to return, and others  require an additional parameter.
/// Simply omit the content or insert the parameter into the macro call
/// as required.
///
/// # Example
/// ```rust
/// use status::*;
///
/// #[get]
/// fn status -> Status<&'static str> {
///     match code {
///         100 (=> status!,)
///         200 (=> status!,)
///         204 (=> status!,)
///         300 (=> status!,)
///         301 (=> status!()
///             MovedPermanently,
///             Location(String::from,
///             "301 (Moved Permanently")
///         ),
///         400 (=> status!,)
///         _   => status!,
///     }
/// }
/// ```
#[macro_export]
macro_rules! status {
    ($name:ident) => {
        $crate::status::Status::$name($crate::status::$name)
    };
    ($name:ident, $content:expr) => {
        $crate::status::Status::$name($crate::status::$name($content))
    };
    ($name:ident, $param:expr, $content:expr) => {
        $crate::status::Status::$name($crate::status::$name($param, $content))
    };
}

/// Get a status by its numeric code
#[cfg(feature = "macro-idents")]
#[macro_export]
macro_rules! status_code {
    ($code:expr) => {
        status!(status_code_lookup!($code))
    };
    ($code:expr, $content:expr) => {
        status!(status_code_lookup!($code), $content)
    };
    ($code:expr, $param:expr, $content:expr) => {
        status!(status_code_lookup!($code), $param, $content)
    };
}

#[cfg(feature = "macro-idents")]
macro_rules! status_code_lookup {
    (100_u16) => {Continue};
    (101_u16) => {SwitchingProtocols};
    (102_u16) => {Processing};
    (200_u16) => {Ok};
    (203_u16) => {NonAuthoritativeInformation};
    (206_u16) => {PartialContent};
    (207_u16) => {MultiStatus};
    (208_u16) => {AlreadyReported};
    (226_u16) => {ImUsed};
    (300_u16) => {MultipleChoices};
    (301_u16) => {MovedPermanently};
    (302_u16) => {Found};
    (303_u16) => {SeeOther};
    (304_u16) => {NotModified};
    (305_u16) => {UseProxy};
    (307_u16) => {TemporaryRedirect};
    (308_u16) => {PermanentRedirect};
    (400_u16) => {BadRequest};
    (401_u16) => {Unauthorized};
    (402_u16) => {PaymentRequired};
    (403_u16) => {Forbidden};
    (405_u16) => {MethodNotAllowed};
    (406_u16) => {NotAcceptable};
    (407_u16) => {ProxyAuthenticationRequired};
    (408_u16) => {RequestTimeout};
    (409_u16) => {Conflict};
    (410_u16) => {Gone};
    (411_u16) => {LengthRequired};
    (412_u16) => {PreconditionFailed};
    (413_u16) => {PayloadTooLarge};
    (414_u16) => {UriTooLong};
    (415_u16) => {UnsupportedMediaType};
    (416_u16) => {RangeNotSatisfiable};
    (417_u16) => {ExpectationFailed};
    (418_u16) => {ImATeapot};
    (421_u16) => {MisdirectedRequest};
    (422_u16) => {UnprocessableEntity};
    (423_u16) => {Locked};
    (424_u16) => {FailedDependency};
    (426_u16) => {UpgradeRequired};
    (428_u16) => {PreconditionRequired};
    (429_u16) => {TooManyRequests};
    (431_u16) => {RequestHeaderFieldsTooLarge};
    (451_u16) => {UnavailableForLegalReasons};
    (500_u16) => {InternalServerError};
    (501_u16) => {NotImplemented};
    (502_u16) => {BadGateway};
    (503_u16) => {ServiceUnavailable};
    (504_u16) => {GatewayTimeout};
    (505_u16) => {HttpVersionNotSupported};
    (506_u16) => {VariantAlsoNegotiates};
    (507_u16) => {InsufficientStorage};
    (508_u16) => {LoopDetected};
    (510_u16) => {NotExtended};
    (511_u16) => {NetworkAuthenticationRequired};
}

/// Responder which encapsulates all response types
///
/// If you need to conditionally return more than two status codes,
/// you can accomplish that by having it return a `Status<R>`. Then,
/// simply use the `status!` macro to return the status of your choice.
pub enum Status<R> {
    Continue(Continue<R>),
    SwitchingProtocols(SwitchingProtocols<R>),
    Processing(Processing<R>),
    Ok(Ok<R>),
    Created(Created<R>),
    Accepted(Accepted<R>),
    NonAuthoritativeInformation(NonAuthoritativeInformation<R>),
    NoContent(NoContent),
    Reset(Reset),
    PartialContent(PartialContent<R>),
    MultiStatus(MultiStatus<R>),
    AlreadyReported(AlreadyReported<R>),
    ImUsed(ImUsed<R>),
    MultipleChoices(MultipleChoices<R>),
    MovedPermanently(MovedPermanently<R>),
    Found(Found<R>),
    SeeOther(SeeOther<R>),
    NotModified(NotModified<R>),
    UseProxy(UseProxy<R>),
    TemporaryRedirect(TemporaryRedirect<R>),
    PermanentRedirect(PermanentRedirect<R>),
    BadRequest(BadRequest<R>),
    Unauthorized(Unauthorized<R>),
    PaymentRequired(PaymentRequired<R>),
    Forbidden(Forbidden<R>),
    NotFound(NotFound<R>),
    MethodNotAllowed(MethodNotAllowed<R>),
    NotAcceptable(NotAcceptable<R>),
    ProxyAuthenticationRequired(ProxyAuthenticationRequired<R>),
    RequestTimeout(RequestTimeout<R>),
    Conflict(Conflict<R>),
    Gone(Gone<R>),
    LengthRequired(LengthRequired<R>),
    PreconditionFailed(PreconditionFailed<R>),
    PayloadTooLarge(PayloadTooLarge<R>),
    UriTooLong(UriTooLong<R>),
    UnsupportedMediaType(UnsupportedMediaType<R>),
    RangeNotSatisfiable(RangeNotSatisfiable<R>),
    ExpectationFailed(ExpectationFailed<R>),
    ImATeapot(ImATeapot<R>),
    MisdirectedRequest(MisdirectedRequest<R>),
    UnprocessableEntity(UnprocessableEntity<R>),
    Locked(Locked<R>),
    FailedDependency(FailedDependency<R>),
    UpgradeRequired(UpgradeRequired<R>),
    PreconditionRequired(PreconditionRequired<R>),
    TooManyRequests(TooManyRequests<R>),
    RequestHeaderFieldsTooLarge(RequestHeaderFieldsTooLarge<R>),
    UnavailableForLegalReasons(UnavailableForLegalReasons<R>),
    InternalServerError(InternalServerError<R>),
    NotImplemented(NotImplemented<R>),
    BadGateway(BadGateway<R>),
    ServiceUnavailable(ServiceUnavailable<R>),
    GatewayTimeout(GatewayTimeout<R>),
    HttpVersionNotSupported(HttpVersionNotSupported<R>),
    VariantAlsoNegotiates(VariantAlsoNegotiates<R>),
    InsufficientStorage(InsufficientStorage<R>),
    LoopDetected(LoopDetected<R>),
    NotExtended(NotExtended<R>),
    NetworkAuthenticationRequired(NetworkAuthenticationRequired<R>),
    Custom(Custom<R>),
}


impl<'r, R: Responder<'r>> Responder<'r> for Status<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, HttpStatus> {
        match self {
            Status::Continue(r) => r.respond_to(req),
            Status::SwitchingProtocols(r) => r.respond_to(req),
            Status::Processing(r) => r.respond_to(req),
            Status::Ok(r) => r.respond_to(req),
            Status::Created(r) => r.respond_to(req),
            Status::Accepted(r) => r.respond_to(req),
            Status::NonAuthoritativeInformation(r) => r.respond_to(req),
            Status::NoContent(r) => r.respond_to(req),
            Status::Reset(r) => r.respond_to(req),
            Status::PartialContent(r) => r.respond_to(req),
            Status::MultiStatus(r) => r.respond_to(req),
            Status::AlreadyReported(r) => r.respond_to(req),
            Status::ImUsed(r) => r.respond_to(req),
            Status::MultipleChoices(r) => r.respond_to(req),
            Status::MovedPermanently(r) => r.respond_to(req),
            Status::Found(r) => r.respond_to(req),
            Status::SeeOther(r) => r.respond_to(req),
            Status::NotModified(r) => r.respond_to(req),
            Status::UseProxy(r) => r.respond_to(req),
            Status::TemporaryRedirect(r) => r.respond_to(req),
            Status::PermanentRedirect(r) => r.respond_to(req),
            Status::BadRequest(r) => r.respond_to(req),
            Status::Unauthorized(r) => r.respond_to(req),
            Status::PaymentRequired(r) => r.respond_to(req),
            Status::Forbidden(r) => r.respond_to(req),
            Status::NotFound(r) => r.respond_to(req),
            Status::MethodNotAllowed(r) => r.respond_to(req),
            Status::NotAcceptable(r) => r.respond_to(req),
            Status::ProxyAuthenticationRequired(r) => r.respond_to(req),
            Status::RequestTimeout(r) => r.respond_to(req),
            Status::Conflict(r) => r.respond_to(req),
            Status::Gone(r) => r.respond_to(req),
            Status::LengthRequired(r) => r.respond_to(req),
            Status::PreconditionFailed(r) => r.respond_to(req),
            Status::PayloadTooLarge(r) => r.respond_to(req),
            Status::UriTooLong(r) => r.respond_to(req),
            Status::UnsupportedMediaType(r) => r.respond_to(req),
            Status::RangeNotSatisfiable(r) => r.respond_to(req),
            Status::ExpectationFailed(r) => r.respond_to(req),
            Status::ImATeapot(r) => r.respond_to(req),
            Status::MisdirectedRequest(r) => r.respond_to(req),
            Status::UnprocessableEntity(r) => r.respond_to(req),
            Status::Locked(r) => r.respond_to(req),
            Status::FailedDependency(r) => r.respond_to(req),
            Status::UpgradeRequired(r) => r.respond_to(req),
            Status::PreconditionRequired(r) => r.respond_to(req),
            Status::TooManyRequests(r) => r.respond_to(req),
            Status::RequestHeaderFieldsTooLarge(r) => r.respond_to(req),
            Status::UnavailableForLegalReasons(r) => r.respond_to(req),
            Status::InternalServerError(r) => r.respond_to(req),
            Status::NotImplemented(r) => r.respond_to(req),
            Status::BadGateway(r) => r.respond_to(req),
            Status::ServiceUnavailable(r) => r.respond_to(req),
            Status::GatewayTimeout(r) => r.respond_to(req),
            Status::HttpVersionNotSupported(r) => r.respond_to(req),
            Status::VariantAlsoNegotiates(r) => r.respond_to(req),
            Status::InsufficientStorage(r) => r.respond_to(req),
            Status::LoopDetected(r) => r.respond_to(req),
            Status::NotExtended(r) => r.respond_to(req),
            Status::NetworkAuthenticationRequired(r) => r.respond_to(req),
            Status::Custom(r) => r.respond_to(req),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_status_macro() {
        use rocket::http::hyper::header::Location;

        fn status(code: u16) -> Status<&'static str> {
            match code {
                100 => status!(Continue, "100 Continue"),
                200 => status!(Ok, "200 Ok"),
                204 => status!(NoContent),
                300 => status!(MultipleChoices, "300 Multiple Choices"),
                301 => {
                    status!(
                        MovedPermanently,
                        Location(String::from("/redirect")),
                        "301 Moved Permanently"
                    )
                }
                400 => status!(BadRequest, "400 Bad Request"),
                418 => status!(ImATeapot, "A teapot, really?"),
                _ => status!(InternalServerError, "500 Internal Server Error"),
            }
        }
    }

    #[cfg(feature = "macro-idents")]
    #[test]
    fn test_use_status_code_macro() {
        use rocket::http::hyper::header::Location;

        fn status(code: u16) -> Status<&'static str> {
            match code {
                100 => status_code!(100, "100 Continue"),
                200 => status_code!(200, "200 Ok"),
                204 => status_code!(204),
                300 => status_code!(300, "300 Multiple Choices"),
                301 => {
                    status_code!(
                        301,
                        Location(String::from("/redirect")),
                        "301 Moved Permanently"
                    )
                }
                400 => status_code!(400, "400 Bad Request"),
                418 => status_code!(418, "A teapot, really?"),
                _ => status_code!(500, "500 Internal Server Error"),
            }
        }
    }
}
