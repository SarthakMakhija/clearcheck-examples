use chrono::NaiveDate;
use clearcheck::assertions::date::DateAssertion;
use clearcheck::assertions::ordered::OrderedAssertion;

#[test]
fn should_be_a_valid_release_date() {
    let release_date = NaiveDate::from_ymd_opt(2024, 01, 30).unwrap();
    release_date.should_have_year(2024)
        .should_be_greater_than(&NaiveDate::from_ymd_opt(2023, 01, 30).unwrap())
        .should_be_less_than_equal_to(&NaiveDate::from_ymd_opt(2025, 01, 30).unwrap())
        .should_be_a_leap_year();
}

#[test]
fn should_be_a_valid_new_year_date() {
    let current_year = 2024;
    let new_year_date = NaiveDate::from_ymd_opt(current_year + 1, 01, 01).unwrap();
    new_year_date.should_have_year(current_year + 1)
        .should_have_day(1)
        .should_have_month(1);
}

