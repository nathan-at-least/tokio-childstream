// TODO: These tests are platform-specific, assuming unixy utilities are present.

use crate::{spawn_stream, ChildItem::*};
use futures::StreamExt;
use tokio::process::Command;

#[tokio::test]
async fn exit_0() {
    let mut stream = spawn_stream(&mut Command::new("true")).unwrap();
    let event = stream.next().await.unwrap();
    assert!(stream.next().await.is_none());
    match event {
        Ok(Exit(status)) => {
            assert_eq!(Some(0), status.code());
        }
        other => panic!("Unexpected event: {other:?}"),
    }
}

#[tokio::test]
async fn hello_world() {
    let mut stream = spawn_stream(&mut Command::new("echo").arg("hello").arg("world")).unwrap();
    let mut found_hw = false;
    let mut found_exit = false;
    while let Some(event) = stream.next().await {
        match event {
            Ok(Stdout(bytes)) => {
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

#[tokio::test]
async fn stderr_hello_world() {
    let mut stream =
        spawn_stream(&mut Command::new("bash").arg("-c").arg("echo 'hello world' >&2")).unwrap();
    let mut found_hw = false;
    let mut found_exit = false;
    while let Some(event) = stream.next().await {
        match event {
            Ok(Stderr(bytes)) => {
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
