#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::{
    fmt::{self, Display},
    time::Duration,
};

/// An Event
#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct Event {
    /// The event name if given
    pub event: Option<String>,
    /// The event data
    pub data: Option<String>,
    /// The event id if given
    pub id: Option<String>,
    /// Retry duration if given
    pub retry: Option<Duration>,
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(event) = &self.event {
            write!(f, "event: {}\n", event)?;
        }
        if let Some(data) = &self.data {
            let x = data.replace("\n", "\ndata: ");
            write!(f, "data: {}\n", x)?;
        }
        if let Some(id) = &self.id {
            write!(f, "id: {}\n", id)?;
        }
        if let Some(retry) = &self.retry {
            write!(f, "retry: {}\n", retry.as_millis())?;
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use futures::prelude::*;

    #[tokio::test]
    async fn test_render_1() {
        let src = "data: Hello, world!\n\n";
        let event = EventStream::new(futures::stream::iter(vec![Ok::<_, ()>(src)]))
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let event_str = event.first().unwrap().to_string();
        assert_eq!(event_str, src);
    }

    #[tokio::test]
    async fn test_render_2() {
        let src = "data: Hello,\ndata: world!\n\n";
        let event = EventStream::new(futures::stream::iter(vec![Ok::<_, ()>(src)]))
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        println!("{:?}", event);
        let event_str = event.iter().map(|elem| elem.to_string()).collect::<Vec<_>>().join("\n");
        assert_eq!(event_str, src);
    }
}
