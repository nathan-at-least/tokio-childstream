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
    for (i, cmd) in opts.subcommands.into_iter().enumerate() {
        let (stream, state) = spawn(cmd)?;
        streams.push(stream.map(move |event| (i, event)));
        states.push(state);
    }

    let mut events = futures::stream::select_all(streams);
    while let Some((i, evres)) = events.next().await {
        let state = &mut states[i];
        let pid = state.pid;

        match evres {
            Ok(Output(source, bytes)) => {
                let buf = match source {
                    Stdout => &mut state.outbuf,
                    Stderr => &mut state.errbuf,
                };

                buf.extend(bytes);
                for line in buf.drain_lines() {
                    print_bytes(pid, source, line);
                }
            }
            Ok(Exit(status)) => {
                if let Some(code) = status.code() {
                    println!("{pid}> exit {code}");
                } else {
                    println!("{pid}> exit ???");
                }
            }
            Err(e) => {
                println!("{pid}-IO ERROR: {e}");
            }
        }
    }

    for state in states.into_iter() {
        for (source, buf) in [(Stdout, state.outbuf), (Stderr, state.errbuf)] {
            if let Some(bytes) = buf.drain_remainder() {
                print_bytes(state.pid, source, bytes);
                println!();
            }
        }
    }

    Ok(())
}

fn spawn(mut cmd: Command) -> std::io::Result<(ChildStream, ChildState)> {
    use tokio_childstream::CommandExt;

    let msg = format!("spawned {:?}", cmd.as_std());
    let stream = cmd.spawn_stream()?;
    let state = ChildState::new(stream.id());
    let pid = state.pid;
    println!("{pid}> {msg}");
    Ok((stream, state))
}

fn print_bytes(pid: u32, source: OutputSource, bytes: BytesMut) {
    let tag = match source {
        Stdout => ' ',
        Stderr => '!',
    };
    let linestr = String::from_utf8_lossy(bytes.as_ref());
    print!("{pid}{tag} {linestr}");
}
