use std::process::Command;

fn namer() -> Command {
    Command::new(env!("CARGO_BIN_EXE_namer"))
}

#[test]
fn default_output_is_uppercase_no_delimiter() {
    let output = namer().output().expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(!name.is_empty());
    assert!(
        name.chars().all(|c| c.is_ascii_uppercase()),
        "expected all caps, got: {name}"
    );
}

#[test]
fn lower_flag_produces_lowercase() {
    let output = namer()
        .arg("--lower")
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(
        name.chars().all(|c| c.is_ascii_lowercase()),
        "expected lowercase, got: {name}"
    );
}

#[test]
fn delimiter_flag_inserts_separator() {
    let output = namer()
        .args(["--delimiter", "-"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(name.contains('-'), "expected hyphen in output, got: {name}");
}

#[test]
fn lower_and_delimiter_combined() {
    let output = namer()
        .args(["--lower", "--delimiter", "_"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(name.contains('_'), "expected underscore, got: {name}");
    let parts: Vec<&str> = name.split('_').collect();
    assert_eq!(parts.len(), 2, "expected two parts, got: {name}");
    for part in &parts {
        assert!(
            part.chars().all(|c| c.is_ascii_lowercase()),
            "expected lowercase part, got: {part}"
        );
    }
}

#[test]
fn help_flag_exits_successfully() {
    let output = namer().arg("--help").output().expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage"),
        "expected usage info in --help output"
    );
}
