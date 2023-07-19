use test_case::test_case;

#[test_case(1 => (1, 0))]
#[test_case(2 => (1, 0))]
#[test_case(3 => (1, 1))]
#[test_case(4 => (1, 2))]
#[test_case(5 => (2, 2))]
#[test_case(6 => (2, 3))]
#[test_case(7 => (3, 3))]
fn truncation_heights(h: usize) -> (usize, usize) {
    super::truncation_heights(h)
}
