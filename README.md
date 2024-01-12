[![clearcheck-examples](https://github.com/SarthakMakhija/clearcheck-examples/actions/workflows/build.yml/badge.svg)](https://github.com/SarthakMakhija/clearcheck-examples/actions/workflows/build.yml)

### clearcheck-examples

This repository provides various examples of [clearcheck](https://github.com/SarthakMakhija/clearcheck) assertions.

I have tried to keep the examples close to real-life.

### Examples

Some examples that are covered in this repository include:

#### String examples

```rust
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
fn should_be_a_valid_sequence() {
    let sequence = 6767;
    sequence.should_not_be_zero()
        .should_be_positive()
        .should_be_in_inclusive_range(1000..=10000)
        .should_be_odd()
        .should_be_greater_than(&1000);
}
```

#### Collection examples

```rust
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
fn should_be_a_valid_list_of_sugar_contents() {
    let sugar_contents = [12.45, 24.51, 36.78, 49.98, 121.89];
    sugar_contents.should_have_at_least_size(4)
        .should_have_upper_bound(130.0)
        .should_be_sorted_ascending()
        .should_be_strictly_increasing();
}
```

### Custom matchers

```rust
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
        MatchersBuilder::start_building_with_negated(be_empty().boxed())
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
```