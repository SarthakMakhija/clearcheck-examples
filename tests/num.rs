use clearcheck::assertions::float::FloatAssertion;
use clearcheck::assertions::int::IntAssertion;
use clearcheck::assertions::ordered::OrderedAssertion;

#[test]
fn should_be_a_valid_sequence() {
    let sequence = 6767;
    sequence.should_not_be_zero()
        .should_be_positive()
        .should_be_in_inclusive_range(1000..=10000)
        .should_be_odd()
        .should_be_greater_than(&1000);
}

#[test]
fn should_be_a_valid_small_ticket_transaction_amount() {
    let money = 454.89;
    money.should_not_be_zero()
        .should_be_positive()
        .should_not_be_nan()
        .should_be_less_than(&500.0)
        .should_be_in_exclusive_range(100.0..500.0);
}