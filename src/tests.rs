use http::Uri;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_http_proxy_no_auth() {
    let mut conn = crate::http::tunnel(
        Uri::from_static("http://httpbin.org:80"),
        "52.229.30.3:80".parse().unwrap(),
        None,
    )
    .await
    .unwrap();

    conn.write(b"GET /headers HTTP/1.1\r\nHost: httpbin.org:80\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut out = String::new();
    conn.read_to_string(&mut out).await.unwrap();
    println!("out: {}", out)
}
