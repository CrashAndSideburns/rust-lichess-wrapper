use hyper::Body;
use tokio_stream::Stream;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use std::task::{ Context, Poll };
use std::error::Error;
use std::marker::{ PhantomData, Unpin };

/// A stream of NDJSON objects.
pub struct NDJsonStream<T> {
    buf: Vec<u8>,
    waiting: bool,
    body: Body,
    phantom: PhantomData<T>
}

impl<T> NDJsonStream<T> {
    pub fn new(body: Body) -> Self {
        NDJsonStream {
            buf: Vec::new(),
            waiting: false,
            body,
            phantom: PhantomData
        }
    }
}

impl<T> Stream for NDJsonStream<T>
where T: DeserializeOwned
{
    type Item = Result<T, Box<dyn Error>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if !self.waiting && !self.buf.is_empty() {

                // Find the location of the first newline in the buffer, if
                // one exists.
                match memchr::memchr(b'\n', &self.buf) {
                    Some(i) => {
                        // Split off the first i + 1 bytes of the buffer from
                        // the rest, and store the other bytes in rest. The
                        // newline character is at i, so we must take the first
                        // i + 1 bytes to avoid leaving the newline leading in
                        // rest.
                        let rest = self.buf.split_off(i + 1);

                        // Replace the current buffer, containing a valid JSON
                        // object, with rest. Deserialize the resultant line
                        // and return.
                        let line = String::from_utf8(std::mem::replace(&mut self.buf, rest))?;
                        return Poll::Ready(Some(Ok(serde_json::from_str(&line)?)));
                    },
                    None => {
                        // There is no newline in the buffer, so we must wait
                        // until polling the body gives us one.
                        self.waiting = true;
                    }
                }
            }

            // Either we are waiting, so the buffer does not contain a newline,
            // or the buffer is empty. Either way, we need more data to create
            // a new object, so poll the body for more data. If the body has
            // data, add it to the buffer and stop waiting. Otherwise just
            // propagate the body's state.
            match Pin::new(&mut self.body).poll_next(cx)? {
                Poll::Ready(Some(buf)) => {
                    self.buf.extend_from_slice(&buf);
                    self.waiting = false;
                },
                Poll::Ready(None) => {
                    return Poll::Ready(None);
                },
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}

// Override the default implementation of Unpin to accommodate the PhantomData.
impl<T> Unpin for NDJsonStream<T> {}
