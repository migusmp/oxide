pub mod json_responses;

pub struct HttpResponse {
    pub status_code: u16,
    pub content_type: String,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, content_type: &str, body: &str) -> Self {
        Self {
            status_code,
            content_type: content_type.to_string(),
            body: body.to_string(),
        }
    }

    pub fn to_raw_response(&self) -> String {
        let status_line = match self.status_code {
            200 => "HTTP/1.1 200 OK",
            404 => "HTTP/1.1 404 Not Found",
            500 => "HTTP/1.1 500 Internal Server Error",
            _ => "HTTP/1.1 200 OK",
        };

        format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            self.content_type,
            self.body.len(),
            self.body
        )
    }

    pub fn ok_plaintext(body: &str) -> String {
        Self::new(200, "text/json", body).to_raw_response()
    }

    pub fn ok_json(body: &str) -> String {
        HttpResponse::new(200, "application/json", body).to_raw_response()
    }

    pub fn not_found() -> String {
        HttpResponse::new(404, "text/plain", "404 Not Found").to_raw_response()
    }

    pub fn internal_error() -> String {
        HttpResponse::new(500, "text/plain", "500 Internal Server Error").to_raw_response()
    }

    pub fn method_not_allowed() -> String {
        HttpResponse::new(405, "text/plain", "405 Method not allowed").to_raw_response()
    }
}
