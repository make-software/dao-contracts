use std::{
    fs,
    process::{Command, Stdio},
};

#[test]
#[ignore]
fn tests() {
    let test_cases = trybuild::TestCases::new();
    test_cases.pass("tests/01-parse.rs");
    test_cases.pass("tests/02-contract-impl.rs");
    test_cases.pass("tests/03-create-caller.rs");
    test_cases.pass("tests/04-entry-points.rs");
    test_cases.pass("tests/05-caller-impl-interface.rs");
    test_cases.pass("tests/06-contract-test-interface.rs");
}

#[test]
fn check_casper_contract_interface_output() {
    let (expansion, template) = expand(
        &["expand", "--lib", "contract", "--features", "test-support"],
        "tests/templates/contract.template",
    );
    assert_eq!(expansion, template);
}

#[test]
fn check_casper_contract_bin_output() {
    let (expansion, template) = expand(
        &[
            "expand",
            "--bin",
            "casper_contract",
            "--features",
            "test-support",
        ],
        "tests/templates/bin.template",
    );
    assert_eq!(expansion, template);
}

fn expand(cmd_args: &[&str], template_path: &str) -> (String, String) {
    let expansion = Command::new("cargo")
        .current_dir("./sample-contract")
        .args(cmd_args)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    (
        String::from_utf8_lossy(&expansion.stdout).to_string(),
        fs::read_to_string(template_path).expect("Failed to read template file"),
    )
}
