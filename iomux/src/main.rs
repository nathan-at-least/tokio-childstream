#![doc = include_str!("../README.md")]

mod childstate;
mod linebuf;
mod options;

use self::childstate::ChildState;
use self::options::Options;
use bytes::BytesMut;
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
    let mut states = vec![];
    for (ix, cmd) in opts.subcommands.into_iter().enumerate() {
        let (stream, state) = spawn(ix, cmd)?;
        streams.push(stream.map(move |event| (ix, event)));
        states.push(state);
    }

    let mut events = futures::stream::select_all(streams);
    while let Some((ix, evres)) = events.next().await {
        let state = &mut states[ix];

        match evres {
            Ok(Output(source, bytes)) => {
                let buf = match source {
                    Stdout => &mut state.outbuf,
                    Stderr => &mut state.errbuf,
                };

                buf.extend(bytes);
                for line in buf.drain_lines() {
                    print_bytes(ix, source, line);
                }
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

    for (ix, state) in states.into_iter().enumerate() {
        for (source, buf) in [(Stdout, state.outbuf), (Stderr, state.errbuf)] {
            if let Some(bytes) = buf.drain_remainder() {
                print_bytes(ix, source, bytes);
                println!();
            }
        }
    }

    Ok(())
}

fn spawn(ix: usize, mut cmd: Command) -> std::io::Result<(ChildStream, ChildState)> {
    use tokio_childstream::CommandExt;

    {
        let stdcmd = cmd.as_std();
        println!("{ix}> spawning {stdcmd:?}");
    }

    let stream = cmd.spawn_stream()?;

    {
        let pid = stream.id();
        println!("{ix}> PID {pid}");
    }
    let state = ChildState::new(stream.id());
    Ok((stream, state))
}

fn print_bytes(ix: usize, source: OutputSource, bytes: BytesMut) {
    let tag = match source {
        Stdout => ' ',
        Stderr => '!',
    };
    let linestr = String::from_utf8_lossy(bytes.as_ref());
    print!("{ix}{tag} {linestr}");
}
