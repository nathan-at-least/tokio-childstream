use std::str::FromStr;
use tokio_childstream::ChildStream;

#[derive(Debug)]
pub struct Command(tokio::process::Command);

impl Command {
    pub(crate) fn spawn(mut self) -> anyhow::Result<ChildStream> {
        use tokio_childstream::CommandExt;

        let stream = self.0.spawn_stream(true)?;
        Ok(stream)
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(source: &str) -> anyhow::Result<Self> {
        let mut fields = source.split_ascii_whitespace();
        let cmdname = fields
            .next()
            .ok_or_else(|| anyhow::anyhow!("empty command"))?;
        let mut cmd = tokio::process::Command::new(cmdname);
        for field in fields {
            cmd.arg(field);
        }
        Ok(Command(cmd))
    }
}
