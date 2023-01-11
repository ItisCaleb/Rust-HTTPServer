use strum_macros;
#[derive(strum_macros::EnumProperty)]
pub enum HttpStatus {
    #[strum(props(msg="OK",code="200"))]
    OK,
    #[strum(props(msg="Created",code="201"))]
    Created,
    #[strum(props(msg="Accepted",code="202"))]
    Accepted,
    #[strum(props(msg="Non Authoritative Information",code="203"))]
    NonAuthoritativeInformation,
    #[strum(props(msg="No Content",code="204"))]
    NoContent,
    #[strum(props(msg="Reset Content",code="205"))]
    ResetContent,
    #[strum(props(msg="Bad Request",code="400"))]
    BadRequest,
    #[strum(props(msg="Payment Required",code="402"))]
    PaymentRequired,
    #[strum(props(msg="Forbidden",code="403"))]
    Forbidden,
    #[strum(props(msg="Not Found",code="404"))]
    NotFound,
    #[strum(props(msg="Method Not Allowed",code="405"))]
    MethodsNotAllowed,
    #[strum(props(msg="Not Acceptable",code="406"))]
    NotAcceptable,
    #[strum(props(msg="Request Timeout",code="408"))]
    RequestTimeout,
    #[strum(props(msg="Conflict",code="409"))]
    Conflict,
    #[strum(props(msg="Gone",code="410"))]
    Gone,
    #[strum(props(msg="Length Required",code="411"))]
    LengthRequired,
    #[strum(props(msg="Payload Too Large",code="413"))]
    PayloadTooLarge,
    #[strum(props(msg="URI Too Long",code="414"))]
    URITooLong,
    #[strum(props(msg="Unsupported Media Type",code="415"))]
    UnsupportedMediaType,
    #[strum(props(msg="Expectation Failed",code="417"))]
    ExpectationFailed,
    #[strum(props(msg="Upgrade Required",code="426"))]
    UpgradeRequired,
    #[strum(props(msg="Internal Server Error",code="500"))]
    InternalServerError,
    #[strum(props(msg="Not Implemented",code="501"))]
    NotImplemented,
    #[strum(props(msg="Bad Gateway",code="502"))]
    BadGateway,
    #[strum(props(msg="Service Unavailable",code="503"))]
    ServiceUnavailable,
    #[strum(props(msg="Gateway Timeout",code="504"))]
    GatewayTimeout,
    #[strum(props(msg="HTTP Version Not Supported",code="505"))]
    HTTPVersionNotSupported
}