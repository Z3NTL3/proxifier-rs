use http::Uri;

#[tokio::main]
async fn main() {
    proxifier_rs::http::HttpProxy::tunnel(
        Uri::from_static("http://z3ntl3.com"),
        "183.215.23.242:9091".parse().unwrap(),
    )
    .await;
}
