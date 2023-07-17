pub fn assert_byte_vecs_eq<A, X, B, Y>(expected: A, actual: B)
where
    A: AsRef<[X]>,
    X: AsRef<[u8]>,
    B: AsRef<[Y]>,
    Y: AsRef<[u8]>,
{
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
        expected
            .iter()
            .map(|b| String::from_utf8_lossy(b).to_owned())
            .collect::<Vec<_>>(),
        actual
            .iter()
            .map(|b| String::from_utf8_lossy(b).to_owned())
            .collect::<Vec<_>>()
    );
}
