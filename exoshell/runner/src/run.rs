use crate::formatrows::FormatRows;

#[derive(Debug, derive_new::new)]
pub struct Run {
    header: String,
    #[new(default)]
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

impl Run {
    pub fn format_header<N>(&self, max_width: N) -> &str
    where
        usize: From<N>,
    {
        FormatRows::new(usize::from(max_width), &self.header)
            .next()
            .unwrap()
    }

    pub fn format_log<N>(
        &self,
        max_width: N,
    ) -> impl DoubleEndedIterator<Item = (LogItemSource, &str)>
    where
        usize: From<N>,
    {
        let max_width = usize::from(max_width);
        self.log.iter().flat_map(move |LogItem { source, text }| {
            FormatRows::new(max_width, text.as_str())
                .map(|row| (*source, row))
                .collect::<Vec<_>>()
                .into_iter()
        })
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
