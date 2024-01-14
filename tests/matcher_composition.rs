#[cfg(test)]
mod custom_string_matchers_tests {
    use std::fmt::Debug;

    use clearcheck::matchers::{BoxWrap, Should};
    use clearcheck::matchers::compose::{Matchers, MatchersBuilder};
    use clearcheck::matchers::string::boundary::begin_with;
    use clearcheck::matchers::string::empty::be_empty;
    use clearcheck::matchers::string::length::have_atleast_same_length;
    use clearcheck::matchers::string::membership::{contain_a_digit, contain_any_of_characters, contain_ignoring_case};

    fn be_a_valid_password<T: AsRef<str> + Debug>() -> Matchers<T> {
        MatchersBuilder::start_building_with_inverted(be_empty().boxed())
            .push(have_atleast_same_length(10).boxed())
            .push(contain_a_digit().boxed())
            .push(contain_any_of_characters(vec!['@', '#']).boxed())
            .push_inverted(begin_with("pass").boxed())
            .push_inverted(contain_ignoring_case("pass").boxed())
            .push_inverted(contain_ignoring_case("word").boxed())
            .combine_as_and()
    }

    trait PasswordAssertion {
        fn should_be_a_valid_password(&self) -> &Self;
    }

    impl PasswordAssertion for &str {
        fn should_be_a_valid_password(&self) -> &Self {
            self.should(&be_a_valid_password());
            self
        }
    }

    #[test]
    fn should_be_a_valid_password() {
        let password = "P@@sw0rd9082";
        password.should_be_a_valid_password();
    }

    #[test]
    #[should_panic]
    fn should_not_be_a_valid_password() {
        let password = "P@@sword9082";
        password.should_be_a_valid_password();
    }
}

#[cfg(test)]
mod custom_collection_matchers_tests {
    use std::fmt::Debug;

    use clearcheck::matchers::{BoxWrap, Should};
    use clearcheck::matchers::collection::empty::be_empty;
    use clearcheck::matchers::collection::length::have_atleast_same_length;
    use clearcheck::matchers::collection::membership::contain_all;
    use clearcheck::matchers::collection::sort::be_sorted_ascending;
    use clearcheck::matchers::compose::{Matchers, MatchersBuilder};

    #[derive(Debug, Eq, PartialEq, PartialOrd)]
    enum LaptopBrands {
        Apple,
        Asus,
        Dell,
        Lenovo,
    }

    fn be_valid_laptop_brands(all: Vec<LaptopBrands>) -> Matchers<Vec<LaptopBrands>> {
        let empty = be_empty();
        let size = have_atleast_same_length(3);
        let contain_all = contain_all(all);
        let sorted = be_sorted_ascending();

        MatchersBuilder::<Vec<LaptopBrands>>::start_building_with_inverted(empty.boxed())
            .push(size.boxed())
            .push(contain_all.boxed())
            .push(sorted.boxed())
            .combine_as_and()
    }

    trait LaptopAssertion {
        fn should_be_valid_laptop_brands(&self) -> &Self;
    }

    impl LaptopAssertion for Vec<LaptopBrands> {
        fn should_be_valid_laptop_brands(&self) -> &Self {
            self.should(&be_valid_laptop_brands(vec![LaptopBrands::Dell]));
            self
        }
    }

    #[test]
    fn should_be_a_valid_collection_of_laptop_brands() {
        let brands = vec![LaptopBrands::Apple, LaptopBrands::Asus, LaptopBrands::Dell, LaptopBrands::Lenovo];
        brands.should_be_valid_laptop_brands();
    }

    #[test]
    #[should_panic]
    fn should_not_be_a_valid_collection_of_laptop_brands() {
        let brands = vec![LaptopBrands::Apple, LaptopBrands::Asus, LaptopBrands::Lenovo];
        brands.should_be_valid_laptop_brands();
    }
}