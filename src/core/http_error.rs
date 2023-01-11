use crate::core::HttpStatus;

pub struct HttpError{
    pub(super) status: HttpStatus,
    pub(super) message: String
}

impl HttpError{
    pub(super) fn new(status: HttpStatus, message: &str)->HttpError{
        HttpError{status,message:message.to_string()}
    }
    pub(super) fn not_found()->HttpError{
        Self::new(HttpStatus::NotFound,"Nothing Here")
    }
    pub(super) fn bad_request()->HttpError{
        Self::new(HttpStatus::BadRequest,"Something went wrong")
    }
}