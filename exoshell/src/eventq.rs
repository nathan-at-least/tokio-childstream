mod reader;
mod sender;

pub use self::reader::Reader;
pub use self::sender::Sender;

pub fn init() -> (Reader, Sender) {
    use tokio::sync::mpsc;

    let (s, r) = mpsc::unbounded_channel();
    let reader = Reader::wrap(r);
    let sender = Sender::wrap(s);

    sender.add_producer(crossterm::event::EventStream::default());
    (reader, sender)
}
