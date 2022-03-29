use pretty_assertions::assert_eq;
use std::{
    fs,
    process::{Command, Stdio},
};

#[test]
fn check_casper_contract_interface_output() {
    let (expansion, template) = expand(
        &["expand", "--lib", "contract", "--features", "test-support"],
        "tests/templates/contract.template",
    );
    assert_eq!(template, expansion);
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
    assert_eq!(template, expansion);
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
