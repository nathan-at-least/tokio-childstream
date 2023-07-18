pub fn assert_byte_vecs_eq<A, X, B, Y>(expected: A, actual: B)
where
    A: AsRef<[X]>,
    X: AsRef<[u8]>,
    B: AsRef<[Y]>,
    Y: AsRef<[u8]>,
{
    fn bytes_to_string(b: &&[u8]) -> String {
        String::from_utf8_lossy(b).into_owned()
    }
    let expected = expected
        .as_ref()
        .iter()
        .map(|v| v.as_ref())
        .collect::<Vec<_>>();
    let actual = actual
        .as_ref()
        .iter()
        .map(|v| v.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(
        expected,
        actual,
        "\n -- expected --\n{:#?}\n -- actual --\n{:#?}\n",
        expected.iter().map(bytes_to_string).collect::<Vec<_>>(),
        actual.iter().map(bytes_to_string).collect::<Vec<_>>()
    );
}
