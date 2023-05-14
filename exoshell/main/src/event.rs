pub(crate) type EventReader = exoshell_event_queue::Reader<Event>;
pub(crate) type EventSender = exoshell_event_queue::Sender<Event>;

pub(crate) use exoshell_event_queue::init as init_queue;

#[derive(Debug, derive_more::From, derive_more::TryInto)]
pub(crate) enum Event {
    Terminal(std::io::Result<crossterm::event::Event>),
    Child(ChildEvent),
}

#[derive(Debug, derive_new::new)]
pub(crate) struct ChildEvent {
    #[allow(dead_code)]
    runix: usize,
    #[allow(dead_code)]
    ev: tokio_childstream::StreamItem,
}
