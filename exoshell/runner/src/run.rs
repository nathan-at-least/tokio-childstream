use crate::formatrows::FormatRows;

#[derive(Debug, derive_new::new)]
pub struct Run {
    cmdtext: String,
    #[new(default)]
    status: Status,
    #[new(default)]
    log: Vec<(LogItemSource, String)>,
}

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Running,
    FailedToLaunch,
    Exited(std::process::ExitStatus),
}

#[derive(Copy, Clone, Debug)]
pub enum LogItemSource {
    FailedToLaunch,
    ChildIO,
    ChildOut,
    ChildErr,
    LineContinuation,
}

impl Run {
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn command(&self) -> &str {
        &self.cmdtext
    }

    pub fn log(&self) -> &[(LogItemSource, String)] {
        &self.log
    }

    pub fn log_length(&self) -> usize {
        self.log.len()
    }

    pub fn layout_reverse_log<N>(&self, max_width: N) -> impl Iterator<Item = (LogItemSource, &str)>
    where
        usize: From<N>,
    {
        let max_width = usize::from(max_width);

        self.log.iter().rev().flat_map(move |(source, text)| {
            FormatRows::new(max_width, text.as_str())
                .fold(vec![], |mut rows, row| {
                    rows.push((
                        if rows.is_empty() {
                            *source
                        } else {
                            LogItemSource::LineContinuation
                        },
                        row,
                    ));
                    rows
                })
                .into_iter()
                .rev()
        })
    }

    pub(crate) fn log_execution_error(&mut self, error: anyhow::Error) {
        self.status = Status::FailedToLaunch;
        self.log
            .push((LogItemSource::FailedToLaunch, format!("{error:#}")));
    }

    pub(crate) fn log_child_item(&mut self, item: tokio_childstream::StreamItem) {
        use tokio_childstream::ChildEvent::*;
        use tokio_childstream::OutputSource::*;
        use LogItemSource::*;

        fn stringify<B>(bytes: B) -> String
        where
            B: AsRef<[u8]>,
        {
            String::from_utf8_lossy(bytes.as_ref()).into_owned()
        }

        match item {
            Ok(Output(Stdout, bytes)) => {
                self.log.push((ChildOut, stringify(bytes)));
            }
            Ok(Output(Stderr, bytes)) => {
                self.log.push((ChildErr, stringify(bytes)));
            }
            Ok(Exit(status)) => {
                self.status = Status::Exited(status);
            }
            Err(e) => {
                self.log.push((ChildIO, format!("{e}")));
            }
        }
    }
}
