use exoshell_runner::{LogItemSource, PaneMeta, Status};
use std::process::ExitStatus;

pub(crate) trait Glyph {
    fn glyph(&self) -> char;
}

impl Glyph for char {
    fn glyph(&self) -> char {
        *self
    }
}

impl Glyph for Status {
    fn glyph(&self) -> char {
        use Status::*;

        match self {
            Running => '↻',
            FailedToLaunch => FAILED_TO_LAUNCH,
            Exited(exitstatus) => exitstatus.glyph(),
        }
    }
}

impl Glyph for ExitStatus {
    fn glyph(&self) -> char {
        match self.code() {
            None => '?',
            Some(0) => '✓',
            _ => '✗',
        }
    }
}

impl Glyph for LogItemSource {
    fn glyph(&self) -> char {
        use LogItemSource::*;

        match self {
            FailedToLaunch => FAILED_TO_LAUNCH,
            ChildIO => 'X',
            ChildOut => ' ',
            ChildErr => '⚠',
        }
    }
}

impl Glyph for PaneMeta {
    fn glyph(&self) -> char {
        use PaneMeta::*;

        match self {
            Header(x) => x.glyph(),
            Line(x) => x.glyph(),
            LineContinuation => '⮚',
        }
    }
}

const FAILED_TO_LAUNCH: char = '⚠';
