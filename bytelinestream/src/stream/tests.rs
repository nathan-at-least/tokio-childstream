use crate::ByteLineStream;
use bytelinebuf::testutil::assert_byte_vecs_eq;
use futures::stream::StreamExt;
use test_case::test_case;

#[test_case(
    b"hello world!".as_slice(),
    vec![b"hello world!".as_slice()]
    ; "single line"
)]
#[test_case(
    b"hello\nworld!".as_slice(),
    vec![
        b"hello\n".as_slice(),
        b"world!".as_slice(),
    ]
    ; "two lines non-terminated"
)]
#[test_case(
    b"hello\nworld!\n".as_slice(),
    vec![
        b"hello\n".as_slice(),
        b"world!\n".as_slice(),
    ]
    ; "two lines terminated"
)]
#[tokio::test]
async fn collect(bytes: &[u8], expected: Vec<&[u8]>) {
    let stream = ByteLineStream::from(futures::stream::iter(Some(
        Ok::<_, std::convert::Infallible>(Vec::from(bytes)),
    )));
    let actual: Vec<Vec<u8>> = stream.map(|r| r.unwrap()).collect().await;
    assert_byte_vecs_eq(expected, actual);
}
