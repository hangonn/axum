use axum::{
    async_trait,
    extract::{rejection::ExtensionRejection, FromRequest},
    http::{StatusCode, Request},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_macros::FromRequest;

fn main() {
    let _: Router = Router::new().route("/", get(handler).post(handler_result));
}

async fn handler(_: MyExtractor) {}

async fn handler_result(_: Result<MyExtractor, MyRejection>) {}

#[derive(FromRequest)]
#[from_request(rejection(MyRejection))]
struct MyExtractor {
    one: Extension<String>,
    #[from_request(via(Extension))]
    two: String,
    three: OtherExtractor,
}

struct OtherExtractor;

#[async_trait]
impl<S, B> FromRequest<S, B> for OtherExtractor
where
    B: Send + 'static,
    S: Send + Sync,
{
    // this rejection doesn't implement `Display` and `Error`
    type Rejection = (StatusCode, String);

    async fn from_request(_req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}

struct MyRejection {}

impl From<ExtensionRejection> for MyRejection {
    fn from(_: ExtensionRejection) -> Self {
        todo!()
    }
}

impl From<(StatusCode, String)> for MyRejection {
    fn from(_: (StatusCode, String)) -> Self {
        todo!()
    }
}

impl IntoResponse for MyRejection {
    fn into_response(self) -> Response {
        todo!()
    }
}