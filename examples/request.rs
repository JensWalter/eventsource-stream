use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use http::response::Builder;
use reqwest::Response;
use reqwest::ResponseBuilderExt;
use url::Url;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let url = Url::parse("https://example.com").unwrap();
    let response = Builder::new()
        .status(200)
        .url(url.clone())
        .body(
            "event: my-event\r\ndata:line1
data: line2
:
id: my-id
:should be ignored too\rretry:42

",
        )
        .unwrap();
    let response = Response::from(response);
    let mut stream = response.bytes_stream().eventsource();

    let event = stream.next().await.unwrap().unwrap();
    assert_eq!(Some("my-event".to_string()), event.event);
    assert_eq!(
        Some("line1\nline2".to_string()),
        event.data
    );
    assert_eq!(Some("my-id".to_string()), event.id);
    assert_eq!(std::time::Duration::from_millis(42), event.retry.unwrap());
}
