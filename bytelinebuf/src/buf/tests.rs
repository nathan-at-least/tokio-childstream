use crate::testutil::assert_byte_vecs_eq;
use crate::ByteLineBuf;
use test_case::test_case;

#[test_case(b"hello world!".as_slice(), vec![b"hello world!".as_slice()])]
#[test_case(b"hello\nworld!".as_slice(), vec![b"hello\n".as_slice(), b"world!".as_slice()])]
#[test_case(b"hello\nworld!\n".as_slice(), vec![b"hello\n".as_slice(), b"world!\n".as_slice()])]
fn drain(bytes: &[u8], expected: Vec<&[u8]>) {
    let mut blb = ByteLineBuf::default();
    blb.extend(bytes);
    let actual: Vec<Vec<u8>> = blb.into_iter().collect();
    assert_byte_vecs_eq(actual, expected);
}
