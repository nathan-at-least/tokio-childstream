use super::FormatRows;
use test_case::test_case;

#[test_case("", &[])]
#[test_case("ab", &["ab"])]
#[test_case("abcdefg", &["abcde", "fg"])]
#[test_case("abc\ndefg", &["abc", "defg"])]
#[test_case("abcdef\n\nghi", &["abcde", "f", "", "ghi"])]
fn format_rows(input: &str, expected: &[&str]) {
    let actual: Vec<_> = FormatRows::new(5, input).collect();
    assert_eq!(expected, actual);
}
