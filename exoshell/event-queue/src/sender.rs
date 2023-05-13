use crate::QuitOrAppEvent;
use futures::stream::Stream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

/// Send events to the queue
#[derive(Debug)]
pub struct Sender<Event>(Inner<Event>);

type Inner<Event> = mpsc::UnboundedSender<QuitOrAppEvent<Event>>;

impl<Event> Sender<Event> {
    pub(super) fn wrap(s: Inner<Event>) -> Self {
        Sender(s)
    }

    /// Send `event`
    pub fn send<T>(&self, event: T) -> Result<(), SendError<T>>
    where
        Event: From<T>,
        T: TryFrom<Event>,
    {
        self.0
            .send(QuitOrAppEvent::from(Event::from(event)))
            .map_err(|SendError(qoae)| {
                use QuitOrAppEvent::*;

                match qoae {
                    AppEvent(ae) => match T::try_from(ae) {
                        Ok(original) => SendError(original),
                        _ => panic!("failed to convert `T::try_from(Event::from(t))``"),
                    },
                    Quit => panic!("send invariant failure: unexpected SendError(Quit)"),
                }
            })
    }

    /// Send all events from a stream
    pub fn send_stream<S, T>(&self, mut producer: S)
    where
        S: Stream<Item = T> + Send + std::marker::Unpin + 'static,
        Event: From<T> + Send + 'static,
        T: TryFrom<Event> + std::fmt::Debug,
    {
        let sender: Sender<Event> = self.clone();
        tokio::task::spawn(async move {
            use futures::stream::StreamExt;

            while let Some(item) = producer.next().await {
                sender.send(item).unwrap();
            }
        });
    }

    /// Send the in-band quit signal
    pub fn send_quit(&self) -> Result<(), SendError<()>> {
        self.0
            .send(QuitOrAppEvent::Quit)
            .map_err(|SendError(_)| SendError(()))
    }
}

impl<E> Clone for Sender<E> {
    fn clone(&self) -> Self {
        Sender(self.0.clone())
    }
}
