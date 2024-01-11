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
        .should_contain_all_characters(&['@', ' '])
        .should_contain_a_digit()
        .should_not_contain_ignoring_case("pass")
        .should_not_contain_ignoring_case("word");
}

#[test]
fn should_be_a_valid_sequence_number() {
    let id = "4589458";
    id.should_not_be_empty()
        .should_be_numeric::<i32>()
        .should_have_at_least_length(4);
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
```
