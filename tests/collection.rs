use clearcheck::assertions::collection::bound::BoundAssertion;
use clearcheck::assertions::collection::duplicate::DuplicateContentAssertion;
use clearcheck::assertions::collection::equal::IgnoreCaseEqualityAssertion;
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
        .should_contain_any(vec!["junit", "clearcheck", "testing"])
        .should_not_contain_any(vec!["scalatest", "gotest"]);
}

#[test]
fn should_be_valid_list_of_release_months() {
    #[derive(Debug, Eq, PartialOrd, PartialEq)]
    enum ReleaseMonths {
        January = 1,
        March = 2,
        June = 3,
        August = 4,
    }

    let release_months = [ReleaseMonths::January, ReleaseMonths::March, ReleaseMonths::June, ReleaseMonths::August];
    release_months.should_not_be_empty()
        .should_not_contain_duplicates()
        .should_have_upper_bound(ReleaseMonths::August)
        .should_be_strictly_increasing();
}

#[test]
fn should_be_a_valid_list_of_sugar_contents() {
    let sugar_contents = [12.45, 24.51, 36.78, 49.98, 121.89];
    sugar_contents.should_have_at_least_size(4)
        .should_have_upper_bound(130.0)
        .should_be_sorted_ascending()
        .should_be_strictly_increasing();
}

#[test]
fn should_be_a_valid_list_of_storage_engines() {
    let supported_storage_engines = ["boltdb", "badgerdb", "speeddb", "rocksdb"];
    supported_storage_engines.should_not_be_empty()
        .should_not_contain_duplicates()
        .should_have_at_least_size(3)
        .should_be_equal_ignoring_case(["BoltDB", "BadgerDB", "SpeedDB", "RocksDB"]);
}