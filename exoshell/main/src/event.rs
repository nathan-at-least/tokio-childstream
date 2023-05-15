pub(crate) use exoshell_event_queue::init as init_queue;
pub(crate) type EventReader = exoshell_event_queue::Reader<Event>;

#[derive(Debug, derive_more::From, derive_more::TryInto)]
pub(crate) enum Event {
    Tick(tokio::time::Instant),
    Terminal(std::io::Result<crossterm::event::Event>),
    Child(exoshell_runner::Event),
}
