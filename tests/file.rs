use std::fs::File;

use clearcheck::assertions::file::FileAssertion;

use tempdir::TempDir;

#[test]
fn should_be_a_valid_directory() {
    let temporary_directory = TempDir::new(".").unwrap();
    let file_path_junit = temporary_directory.path().join("junit.txt");
    let file_path_clearcheck = temporary_directory.path().join("clearcheck.txt");

    let _ = File::create(file_path_junit).unwrap();
    let _ = File::create(file_path_clearcheck).unwrap();

    temporary_directory.should_be_a_directory()
        .should_contain_all_file_names(&["junit.txt", "clearcheck.txt"])
        .should_not_be_zero_sized()
        .should_not_contain_file_name("gotest.txt");
}