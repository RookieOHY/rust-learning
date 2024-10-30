use ch02_os::search;

mod common; // 配合 cargo test 或者 cargo test --test integration_test
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, 4);
}

#[test]
fn test(){
    let query = "Rust";
    let content = "\
    Rust:
    safe,fast,productive.
    pick three.";
    assert_eq!(vec!["Rust:"],search(query,content));
}