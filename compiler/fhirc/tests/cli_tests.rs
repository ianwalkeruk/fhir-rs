use assert_cmd::Command;
use predicates::str::contains;

fn fhirc() -> Command {
    Command::cargo_bin("fhirc").unwrap()
}

#[test]
fn test_version_outputs_version_info() {
    fhirc()
        .arg("version")
        .assert()
        .success()
        .stdout(contains("fhirc"))
        .stdout(contains("version"))
        .stdout(contains("IR version"));
}

#[test]
fn test_schema_outputs_schema_version() {
    fhirc()
        .arg("schema")
        .assert()
        .success()
        .stdout(contains("IR schema"));
}

#[test]
fn test_validate_ir_valid_file_exits_zero() {
    fhirc()
        .arg("validate-ir")
        .arg("tests/data/valid.ir.yaml")
        .assert()
        .success();
}

#[test]
fn test_validate_ir_invalid_file_exits_non_zero() {
    fhirc()
        .arg("validate-ir")
        .arg("tests/data/invalid.ir.yaml")
        .assert()
        .failure();
}

#[test]
fn test_pretty_outputs_stable_format() {
    fhirc()
        .arg("pretty")
        .arg("tests/data/patient.ir.yaml")
        .assert()
        .success()
        .stdout(contains("Patient"))
        .stdout(contains("ir_version"));
}

#[test]
fn test_canonicalise_produces_deterministic_output() {
    fhirc()
        .arg("canonicalise")
        .arg("tests/data/patient.ir.yaml")
        .assert()
        .success()
        .stdout(contains("Patient"))
        .stdout(contains("ir_version"));
}

#[test]
fn test_parse_returns_not_implemented() {
    fhirc()
        .arg("parse")
        .assert()
        .success()
        .stdout(contains("not yet implemented"));
}

#[test]
fn test_pass_returns_not_implemented() {
    fhirc()
        .arg("pass")
        .arg("test-pass")
        .assert()
        .success()
        .stdout(contains("not yet implemented"));
}

#[test]
fn test_generate_returns_not_implemented() {
    fhirc()
        .arg("generate")
        .assert()
        .success()
        .stdout(contains("not yet implemented"));
}

#[test]
fn test_diff_returns_not_implemented() {
    fhirc()
        .arg("diff")
        .assert()
        .success()
        .stdout(contains("not yet implemented"));
}
