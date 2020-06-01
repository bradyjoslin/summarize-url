use assert_cmd::Command;

#[test]
fn it_helps() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn it_works() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("http://techcrunch.com/2015/03/12/algorithmia-launches-with-more-than-800-algorithms-on-its-marketplace/")
        .assert()
        .success();
}

#[test]
fn it_errors_without_input() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn it_fails_on_bad_command() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("foo")
        .assert()
        .failure();
}
