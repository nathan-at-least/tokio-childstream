#![doc = include_str!("../README.md")]

mod options;

use self::options::Options;
use futures::StreamExt;
use tokio::process::Command;
use tokio_childstream::{
    ChildEvent::*,
    ChildStream,
    OutputSource::{self, *},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = Options::parse()?;

    let mut streams = vec![];
    for (ix, cmd) in opts.subcommands.into_iter().enumerate() {
        let stream = spawn(ix, cmd)?;
        streams.push(stream.map(move |event| (ix, event)));
    }

    let mut events = futures::stream::select_all(streams);
    while let Some((ix, evres)) = events.next().await {
        match evres {
            Ok(Output(source, line)) => {
                print_bytes(ix, source, line.as_ref());
            }
            Ok(Exit(status)) => {
                if let Some(code) = status.code() {
                    println!("{ix}> exit {code}");
                } else {
                    println!("{ix}> exit ???");
                }
            }
            Err(e) => {
                println!("{ix}-IO ERROR: {e}");
            }
        }
    }

    Ok(())
}

fn spawn(ix: usize, mut cmd: Command) -> std::io::Result<ChildStream> {
    use tokio_childstream::CommandExt;

    {
        let stdcmd = cmd.as_std();
        println!("{ix}> spawning {stdcmd:?}");
    }

    let stream = cmd.spawn_stream(true)?;

    {
        let pid = stream.id();
        println!("{ix}> PID {pid}");
    }
    Ok(stream)
}

fn print_bytes(ix: usize, source: OutputSource, line: &[u8]) {
    let tag = match source {
        Stdout => ' ',
        Stderr => '!',
    };
    let linestr = String::from_utf8_lossy(line);
    print!("{ix}{tag} {linestr}");
    if !linestr.ends_with('\n') {
        println!();
    }
}
