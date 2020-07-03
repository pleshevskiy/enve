use bytes::buf::BufExt;
use futures_util::{stream, StreamExt};
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use itconfig::config;

config! {
    HYPER {
        PREFER_SCHEMA: String => "http",

        HOST < (
            ADDR => "127.0.0.1",
            ":",
            PORT => 8000,
        ),

        static JSON_API_URL < (
            HYPER_PREFER_SCHEMA,
            "://",
            HYPER_HOST,
            "/json_api",
        ),
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type HyperResult<T> = std::result::Result<T, GenericError>;

const INDEX: &'static [u8] = b"<a href=\"test.html\">test.html</a>";
const INTERNAL_SERVER_ERROR: &'static [u8] = b"Internal Server Error";
const NOTFOUND: &'static [u8] = b"Not Found";
const POST_DATA: &'static str = r#"{"original": "data"}"#;

async fn client_request_response(client: &Client<HttpConnector>) -> HyperResult<Response<Body>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri(config::HYPER::JSON_API_URL())
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(POST_DATA))
        .unwrap();

    let web_res = client.request(req).await?;
    // Compare the JSON we sent (before) with what we received (after):
    let before = stream::once(async {
        Ok(format!(
            "<b>POST request body</b>: {}<br><b>Response</b>: ",
            POST_DATA,
        )
        .into())
    });
    let after = web_res.into_body();
    let body = Body::wrap_stream(before.chain(after));

    Ok(Response::new(body))
}

async fn api_post_response(req: Request<Body>) -> HyperResult<Response<Body>> {
    // Aggregate the body...
    let whole_body = hyper::body::aggregate(req).await?;
    // Decode as JSON...
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    // Change the JSON...
    data["test"] = serde_json::Value::from("test_value");
    // And respond with the new JSON.
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;
    Ok(response)
}

async fn api_get_response() -> HyperResult<Response<Body>> {
    let data = vec!["foo", "bar"];
    let res = match serde_json::to_string(&data) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}

async fn response_examples(
    req: Request<Body>,
    client: Client<HttpConnector>,
) -> HyperResult<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(INDEX.into())),
        (&Method::GET, "/test.html") => client_request_response(&client).await,
        (&Method::POST, "/json_api") => api_post_response(req).await,
        (&Method::GET, "/json_api") => api_get_response().await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(NOTFOUND))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> HyperResult<()> {
    config::init();
    pretty_env_logger::init();

    let addr = config::HYPER::HOST().parse().unwrap();

    // Share a `Client` with all `Service`s
    let client = Client::new();

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let client = client.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                response_examples(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
