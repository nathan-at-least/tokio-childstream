use crate::Run;

#[derive(Debug, derive_new::new)]
pub struct Event {
    runix: usize,
    ev: tokio_childstream::StreamItem,
}

impl Event {
    pub(crate) fn insert_into(self, runs: &mut [Run]) -> anyhow::Result<()> {
        let runix = self.runix;
        let runlen = runs.len();
        let run = runs.get_mut(runix).ok_or_else(|| {
            anyhow::anyhow!("internal error: run ix out of bounds")
                .context(format!("run ix: {runix}"))
                .context(format!("runs: {runlen}"))
        })?;
        run.log_child_item(self.ev);
        Ok(())
    }
}
