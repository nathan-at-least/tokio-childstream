// TODO: These tests are platform-specific, assuming unixy utilities are present.

use crate::{
    ChildEvent::{Exit, Output},
    CommandExt,
    OutputSource::{Stderr, Stdout},
};
use futures::StreamExt;
use test_case::test_case;
use tokio::process::Command;

// If this test compiles, it already succeeds:
#[tokio::test]
async fn check_traits() {
    use crate::{CommandExt, StreamItem};
    use futures::Stream;
    use tokio::process::Command;

    fn constrain<S>(_: S)
    where
        S: Stream<Item = StreamItem> + Send,
    {
        // It worked!
    }

    constrain(Command::new("true").spawn_stream(false).unwrap());
}

#[test_case(false ; "no-line-buffering")]
#[test_case(true ; "line-buffering")]
#[tokio::test]
async fn exit_0(line_buffering: bool) {
    let mut stream = Command::new("true").spawn_stream(line_buffering).unwrap();
    let event = stream.next().await.unwrap();
    assert!(stream.next().await.is_none());
    match event {
        Ok(Exit(status)) => {
            assert_eq!(Some(0), status.code());
        }
        other => panic!("Unexpected event: {other:?}"),
    }
}

#[test_case(false ; "no-line-buffering")]
#[test_case(true ; "line-buffering")]
#[tokio::test]
async fn hello_world(line_buffering: bool) {
    let mut stream = Command::new("echo")
        .arg("hello")
        .arg("world")
        .spawn_stream(line_buffering)
        .unwrap();
    let mut found_hw = false;
    let mut found_exit = false;
    while let Some(event) = stream.next().await {
        match event {
            Ok(Output(Stdout, bytes)) => {
                assert_eq!(b"hello world\n", bytes.as_ref(),);
                found_hw = true;
            }
            Ok(Exit(status)) => {
                assert_eq!(Some(0), status.code(),);
                found_exit = true;
            }
            other => panic!("Unexpected event: {other:?}"),
        }
    }
    assert!(found_hw && found_exit);
}

#[test_case(false ; "no-line-buffering")]
#[test_case(true ; "line-buffering")]
#[tokio::test]
async fn two_lines(line_buffering: bool) {
    let mut stream = Command::new("echo")
        .arg("-e")
        .arg(r#"hello world\nsecond line"#)
        .spawn_stream(line_buffering)
        .unwrap();
    let mut found_outputs = vec![];
    let mut found_exit = false;
    while let Some(event) = stream.next().await {
        match event {
            Ok(Output(Stdout, bytes)) => {
                found_outputs.push(Vec::from(bytes.as_ref()));
            }
            Ok(Exit(status)) => {
                assert_eq!(Some(0), status.code(),);
                found_exit = true;
            }
            other => panic!("Unexpected event: {other:?}"),
        }
    }
    let expected = if line_buffering {
        vec![&b"hello world\n"[..], &b"second line\n"[..]]
    } else {
        vec![&b"hello world\nsecond line\n"[..]]
    };
    assert_eq!(
        expected,
        found_outputs,
        "\n  --- expected ---\n{:#?}\n  --- actual ---\n{:#?}\n",
        expected
            .iter()
            .map(|b| String::from_utf8_lossy(b).to_owned())
            .collect::<Vec<_>>(),
        found_outputs
            .iter()
            .map(|b| String::from_utf8_lossy(b).to_owned())
            .collect::<Vec<_>>(),
    );
    assert!(found_exit);
}

#[test_case(false ; "no-line-buffering")]
#[test_case(true ; "line-buffering")]
#[tokio::test]
async fn stderr_hello_world(line_buffering: bool) {
    let mut stream = Command::new("bash")
        .arg("-c")
        .arg("echo 'hello world' >&2")
        .spawn_stream(line_buffering)
        .unwrap();
    let mut found_hw = false;
    let mut found_exit = false;
    while let Some(event) = stream.next().await {
        match event {
            Ok(Output(Stderr, bytes)) => {
                assert_eq!(b"hello world\n", bytes.as_ref(),);
                found_hw = true;
            }
            Ok(Exit(status)) => {
                assert_eq!(Some(0), status.code(),);
                found_exit = true;
            }
            other => panic!("Unexpected event: {other:?}"),
        }
    }
    assert!(found_hw && found_exit);
}
