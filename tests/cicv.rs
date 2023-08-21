use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cicvverify() {
    Command::cargo_bin("arceos")
        .unwrap()
        .args(&["--nocapture", "cicvverify"]) 
        // .current_dir("exercises")
        .assert()
        .success();
}


// cargo test --package rustlings --test cicv -- cicvverify --exact --nocapture 
