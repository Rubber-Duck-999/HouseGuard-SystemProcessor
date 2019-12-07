use assert_cmd::prelude::*;
use glob::glob;
use predicates::boolean::PredicateBooleanExt;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn runs_without_arguments()
{
    let mut cmd = Command::cargo_bin("/bin/ls").unwrap();
    cmd.assert().success();
}
