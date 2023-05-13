pub(crate) trait CleanupWith {
    fn cleanup_with(self, cleanup_result: anyhow::Result<()>) -> anyhow::Result<()>;
}

impl CleanupWith for anyhow::Result<()> {
    fn cleanup_with(self, cleanup_result: anyhow::Result<()>) -> anyhow::Result<()> {
        match (self, cleanup_result) {
            (Ok(()), Ok(())) => Ok(()),
            (original, Ok(())) => original,
            (Ok(()), cleanup) => cleanup,
            (Err(original), Err(cleanup)) => {
                Err(cleanup.context(format!("While cleaning up after: {original:#}")))
            }
        }
    }
}
