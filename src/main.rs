use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[inline(always)]
fn resolve_status(path: &[u8]) -> StatusCode {
    if path == b"/" {
        return StatusCode::OK;
    }

    if path.len() == 4 && path[0] == b'/' {
        let h = path[1].wrapping_sub(b'0');
        let t = path[2].wrapping_sub(b'0');
        let o = path[3].wrapping_sub(b'0');
        if h <= 9 && t <= 9 && o <= 9 {
            let code = (h as u16) * 100 + (t as u16) * 10 + (o as u16);
            if let Ok(s) = StatusCode::from_u16(code) {
                return s;
            }
        }
    }
    StatusCode::BAD_REQUEST
}

struct StatusSvc;

impl hyper::service::Service<Request<hyper::body::Incoming>> for StatusSvc {
    type Response = Response<Empty<Bytes>>;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request<hyper::body::Incoming>) -> Self::Future {
        let status = resolve_status(req.uri().path().as_bytes());
        let mut resp = Response::new(Empty::new());
        *resp.status_mut() = status;
        std::future::ready(Ok(resp))
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        stream.set_nodelay(true).ok();

        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            http1::Builder::new()
                .auto_date_header(false)
                .keep_alive(true)
                .pipeline_flush(true)
                .writev(false)
                .header_read_timeout(None)
                .serve_connection(io, StatusSvc)
                .await
                .ok();
        });
    }
}
