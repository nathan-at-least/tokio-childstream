mod options;

use self::options::Options;
use futures::StreamExt;
use tokio_childstream::{ChildItem::*, CommandExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = Options::parse()?;

    let mut streams = vec![];
    for (i, mut cmd) in opts.subcommands.into_iter().enumerate() {
        println!("{}I: Spawning {:?}", i, cmd.as_std());
        let stream = cmd.spawn_stream()?;
        streams.push(stream.map(move |event| (i, event)));
    }

    let mut events = futures::stream::select_all(streams);
    while let Some((i, evres)) = events.next().await {
        match evres {
            Ok(Stdout(bytes)) => {
                println!("{}O: {}", i, String::from_utf8_lossy(bytes.as_ref()));
            }
            Ok(Stderr(bytes)) => {
                println!("{}E: {}", i, String::from_utf8_lossy(bytes.as_ref()));
            }
            Ok(Exit(status)) => {
                if let Some(code) = status.code() {
                    println!("{}X: exit {}", i, code);
                } else {
                    println!("{}X: exit ???", i);
                }
            }
            Err(e) => {
                println!("{}!: {}", i, e);
            }
        }
    }
    Ok(())
}
