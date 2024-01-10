use clearcheck::assertions::collection::bound::BoundAssertion;
use clearcheck::assertions::collection::duplicate::DuplicateContentAssertion;
use clearcheck::assertions::collection::increasing_decreasing::IncreasingDecreasingAssertion;
use clearcheck::assertions::collection::membership::MembershipAssertion;
use clearcheck::assertions::collection::size::SizeAssertion;
use clearcheck::assertions::collection::sort::SortAssertion;

#[test]
fn should_be_a_valid_list_of_testing_keywords() {
    let keywords = ["testing", "automation", "clearcheck", "junit"];
    keywords.should_not_be_empty()
        .should_have_size_in_inclusive_range(4..=10)
        .should_not_contain_duplicates()
        .should_contain_any(&["junit", "clearcheck", "testing"])
        .should_not_contain_any(&["scalatest", "gotest"]);
}

#[test]
fn should_be_a_valid_sugar_content() {
    let sugar_contents = [12.45, 24.51, 36.78, 49.98, 121.89];
    sugar_contents.should_have_at_least_size(4)
        .should_have_upper_bound(&130.0)
        .should_be_sorted_ascending()
        .should_be_strictly_increasing();
}