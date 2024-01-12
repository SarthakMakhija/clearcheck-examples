use clearcheck::assertions::string::length::LengthAssertion;
use clearcheck::assertions::string::membership::MembershipAssertion;
use clearcheck::assertions::string::numeric::NumericAssertion;
use clearcheck::assertions::string::regex::RegularExpressionAssertion;
use regex::Regex;

#[test]
fn should_be_a_valid_passphrase() {
    let pass_phrase = "P@@sw0rd1 zebra alpha";
    pass_phrase.should_not_be_empty()
        .should_have_at_least_length(10)
        .should_contain_all_characters(vec!['@', ' '])
        .should_contain_a_digit()
        .should_not_contain_ignoring_case("pass")
        .should_not_contain_ignoring_case("word");
}

#[test]
fn should_be_a_valid_id() {
    let id = "PLMOQ8928R";
    id.should_not_be_empty()
        .should_have_length(10)
        .should_match(Regex::new(r"[A-Z]{5}[0-9]{4}[A-Z]").unwrap());
}

#[test]
fn should_be_a_valid_sequence_number() {
    let id = "4589458";
    id.should_not_be_empty()
        .should_be_numeric::<i32>()
        .should_have_at_least_length(4);
}
