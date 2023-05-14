#[derive(Debug)]
pub struct Run {
    cmdtext: String,
    log: Vec<LogItem>,
}

#[derive(Debug)]
struct LogItem {
    source: LogItemSource,
    text: String,
}

#[derive(Copy, Clone, Debug)]
pub enum LogItemSource {
    ExecutionError,
    ChildIO,
    ChildOut,
    ChildErr,
    ChildExit,
}
use LogItemSource::*;

impl From<String> for Run {
    fn from(cmdtext: String) -> Self {
        Run {
            cmdtext,
            log: vec![],
        }
    }
}

impl Run {
    pub fn command(&self) -> &str {
        &self.cmdtext
    }

    pub fn log_len(&self) -> usize {
        self.log.len()
    }

    pub fn log_items(&self) -> impl Iterator<Item = (LogItemSource, &str)> {
        self.log
            .iter()
            .map(|LogItem { source, text }| (*source, text.as_str()))
    }

    pub(crate) fn log_execution_error(&mut self, error: anyhow::Error) {
        self.log.push(LogItem {
            source: ExecutionError,
            text: format!("{error:#}"),
        });
    }

    pub(crate) fn log_child_item(&mut self, item: tokio_childstream::StreamItem) {
        use tokio_childstream::ChildEvent::*;
        use tokio_childstream::OutputSource::*;

        fn stringify<B>(bytes: B) -> String
        where
            B: AsRef<[u8]>,
        {
            String::from_utf8_lossy(bytes.as_ref()).into_owned()
        }

        let (source, text) = match item {
            Ok(Output(Stdout, bytes)) => (ChildOut, stringify(bytes)),
            Ok(Output(Stderr, bytes)) => (ChildErr, stringify(bytes)),
            Ok(Exit(status)) => (ChildExit, format!("{status:?}")),
            Err(e) => (ChildIO, format!("{e}")),
        };

        self.log.push(LogItem { source, text });
    }
}
