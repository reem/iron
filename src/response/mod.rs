use http::server::request::Request;
use http::server::response::ResponseWriter;
use http::headers::response::HeaderCollection;
use http::status::Status;

pub mod ironresponse;

pub trait Response: Writer {
    fn headers_mut<'a>(&'a mut self) -> &'a mut Box<HeaderCollection>;
    fn status_mut<'a>(&'a mut self) -> &'a mut Status;

    fn request<'a>(&'a self) -> &'a Request;
    fn headers<'a>(&'a self) -> &'a HeaderCollection;
    fn status<'a>(&'a self) -> &'a Status;
}

pub trait HttpResponse<'a, 'b> {
    fn from_http(&'a mut ResponseWriter<'b>) -> Self;
}
