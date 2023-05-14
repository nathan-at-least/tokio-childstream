//! An `tokio` event queue system with in-band pre-emptive quit functionality
mod qevent;
mod reader;
mod sender;

pub use self::reader::Reader;
pub use self::sender::Sender;

pub(crate) use self::qevent::QuitOrAppEvent;

/// Initialize a [Reader] and associated [Sender]
pub fn init<Event>() -> (Reader<Event>, Sender<Event>) {
    use tokio::sync::mpsc;

    let (s, r) = mpsc::unbounded_channel();
    let runner = Reader::wrap(r);
    let sender = Sender::wrap(s);
    (runner, sender)
}
