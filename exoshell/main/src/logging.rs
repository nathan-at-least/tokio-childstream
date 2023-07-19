use std::path::Path;

pub(crate) fn init<P>(log: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let logpath = log.as_ref();
    let logfile = std::fs::File::create(logpath)?;

    tracing_subscriber::fmt()
        .with_writer(logfile)
        .with_max_level(tracing::Level::DEBUG)
        .try_init()
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    tracing::debug!("log initialized in {:?}", logpath.display());
    Ok(())
}
