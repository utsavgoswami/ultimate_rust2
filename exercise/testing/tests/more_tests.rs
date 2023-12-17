use testing::sploosh;
use testing::splish;

#[test]
fn splish_sploosh_integration() {
    let val = sploosh(splish(-1, 0), splish(1, 1), splish(3, 2));

    assert_eq!(val, 4);
}