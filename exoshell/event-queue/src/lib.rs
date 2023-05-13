//! An `tokio` event queue system with in-band pre-emptive quit functionality
mod looprunner;
mod qevent;
mod sender;

pub use self::looprunner::LoopRunner;
pub use self::sender::Sender;

pub(crate) use self::qevent::QuitOrAppEvent;

/// Initialize a [LoopRunner] and associated [Sender]
pub fn init<Event>() -> (LoopRunner<Event>, Sender<Event>) {
    use tokio::sync::mpsc;

    let (s, r) = mpsc::unbounded_channel();
    let runner = LoopRunner::wrap(r);
    let sender = Sender::wrap(s);
    (runner, sender)
}
