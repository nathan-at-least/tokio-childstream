use crate::formatrows::FormatRows;
use exoshell_aui::Pane;
use std::borrow::Cow;

#[derive(Debug, derive_new::new)]
pub struct Run {
    cmdtext: String,
    #[new(default)]
    status: Status,
    #[new(default)]
    log: Vec<(LogItemSource, String)>,
}

#[derive(Copy, Clone, Debug, Default)]
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
}

#[derive(Copy, Clone, Debug, derive_more::From)]
pub enum PaneMeta {
    Header(Status),
    Line(LogItemSource),
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

    pub fn render_into(&self, pane: &mut Pane<PaneMeta>) -> anyhow::Result<()> {
        pane.append_line(self.status, truncate_string(self.command(), pane.width()))?;

        for (meta, row) in self.wrap_log_lines(pane.width()) {
            pane.append_line(meta, row)?;
        }

        Ok(())
    }

    fn wrap_log_lines(&self, max_width: usize) -> impl Iterator<Item = (PaneMeta, &str)> {
        self.log.iter().flat_map(move |(source, text)| {
            FormatRows::new(max_width, text.as_str())
                .enumerate()
                .map(|(ix, row)| {
                    let meta = if ix == 0 {
                        PaneMeta::from(*source)
                    } else {
                        PaneMeta::LineContinuation
                    };
                    (meta, row)
                })
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

fn truncate_string(s: &str, maxlen: usize) -> Cow<'_, str> {
    if s.chars().count() > maxlen {
        let mut s: String = s.chars().take(maxlen - 1).collect();
        s.push('…');
        Cow::from(s)
    } else {
        Cow::from(s)
    }
}
