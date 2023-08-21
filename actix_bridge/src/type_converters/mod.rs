use actix_web::HttpResponse;
use tonic::{Code, Status};
pub fn from_tonic(tonic_status: Status) -> HttpResponse {
    let mut status;
    if tonic_status.code() == Code::Ok {
        status = HttpResponse::Ok();
    } else if tonic_status.code() == Code::Internal {
        status = HttpResponse::InternalServerError();
    } else if tonic_status.code() == Code::InvalidArgument {
        status = HttpResponse::BadRequest();
    } else {
        status = HttpResponse::MethodNotAllowed();
    }
    return status.body(String::from(tonic_status.message()));
}
