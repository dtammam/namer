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

#[test]
fn number_flag_5_produces_exactly_5_lines() {
    let output = namer()
        .args(["--number", "5"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 5, "expected 5 lines, got {}", lines.len());
    for line in &lines {
        assert!(!line.is_empty(), "unexpected empty line in output");
    }
}

#[test]
fn no_number_flag_produces_exactly_1_line() {
    let output = namer().output().expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "expected 1 line, got {}", lines.len());
}

#[test]
fn number_flag_1_produces_exactly_1_line() {
    let output = namer()
        .args(["--number", "1"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "expected 1 line, got {}", lines.len());
}

#[test]
fn number_flag_0_exits_with_nonzero_status() {
    let output = namer()
        .args(["--number", "0"])
        .output()
        .expect("failed to run namer");
    assert!(
        !output.status.success(),
        "expected non-zero exit for --number 0"
    );
}

#[test]
fn number_flag_1001_exits_with_nonzero_status() {
    let output = namer()
        .args(["--number", "1001"])
        .output()
        .expect("failed to run namer");
    assert!(
        !output.status.success(),
        "expected non-zero exit for --number 1001"
    );
}

#[test]
fn number_flag_5_with_lower_produces_5_lowercase_lines() {
    let output = namer()
        .args(["--number", "5", "--lower"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 5, "expected 5 lines, got {}", lines.len());
    for line in &lines {
        assert!(
            line.chars().all(|c| c.is_ascii_lowercase()),
            "line {line:?} is not all lowercase"
        );
    }
}

#[test]
fn number_flag_5_with_delimiter_produces_5_hyphen_delimited_lines() {
    let output = namer()
        .args(["--number", "5", "--delimiter", "-"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 5, "expected 5 lines, got {}", lines.len());
    for line in &lines {
        assert!(
            line.contains('-'),
            "line {line:?} does not contain hyphen delimiter"
        );
    }
}

#[test]
fn help_output_contains_number_flag() {
    let output = namer().arg("--help").output().expect("failed to run namer");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--number"),
        "--help output does not mention --number"
    );
}

// --things flag: standalone category tests

#[test]
fn things_flag_objects_produces_output() {
    let output = namer()
        .args(["--things", "objects"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "expected non-empty output for --things objects"
    );
}

#[test]
fn things_flag_produce_produces_output() {
    let output = namer()
        .args(["--things", "produce"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "expected non-empty output for --things produce"
    );
}

#[test]
fn things_flag_animals_produces_output() {
    let output = namer()
        .args(["--things", "animals"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "expected non-empty output for --things animals"
    );
}

#[test]
fn things_flag_defaults_to_objects() {
    // Running with no --things flag should succeed and produce output (default is objects).
    let output = namer().output().expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.trim().is_empty(),
        "expected non-empty output when --things is omitted (default objects)"
    );
}

#[test]
fn things_flag_invalid_value_exits_nonzero() {
    let output = namer()
        .args(["--things", "invalid"])
        .output()
        .expect("failed to run namer");
    assert!(
        !output.status.success(),
        "expected non-zero exit for --things invalid"
    );
}

// --things flag: help documentation

#[test]
fn help_output_mentions_things_flag_with_accepted_values() {
    let output = namer().arg("--help").output().expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--things"),
        "--help output does not mention --things"
    );
    assert!(
        stdout.contains("objects"),
        "--help output does not list 'objects' as an accepted value"
    );
    assert!(
        stdout.contains("produce"),
        "--help output does not list 'produce' as an accepted value"
    );
    assert!(
        stdout.contains("animals"),
        "--help output does not list 'animals' as an accepted value"
    );
}

// --things flag: combination tests with --lower and --delimiter

#[test]
fn things_flag_produce_with_lower_and_delimiter_hyphen() {
    let output = namer()
        .args(["--things", "produce", "--lower", "--delimiter", "-"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(name.contains('-'), "expected hyphen delimiter, got: {name}");
    let parts: Vec<&str> = name.split('-').collect();
    assert_eq!(
        parts.len(),
        2,
        "expected two parts separated by hyphen, got: {name}"
    );
    for part in &parts {
        assert!(
            part.chars().all(|c| c.is_ascii_lowercase()),
            "expected lowercase part, got: {part}"
        );
    }
}

#[test]
fn things_flag_animals_with_lower_and_delimiter_underscore() {
    let output = namer()
        .args(["--things", "animals", "--lower", "--delimiter", "_"])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(
        name.contains('_'),
        "expected underscore delimiter, got: {name}"
    );
    let parts: Vec<&str> = name.split('_').collect();
    assert_eq!(
        parts.len(),
        2,
        "expected two parts separated by underscore, got: {name}"
    );
    for part in &parts {
        assert!(
            part.chars().all(|c| c.is_ascii_lowercase()),
            "expected lowercase part, got: {part}"
        );
    }
}

#[test]
fn things_flag_objects_with_lower_and_delimiter_dot() {
    let output = namer()
        .args(["--things", "objects", "--lower", "--delimiter", "."])
        .output()
        .expect("failed to run namer");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = stdout.trim();
    assert!(name.contains('.'), "expected dot delimiter, got: {name}");
    let parts: Vec<&str> = name.split('.').collect();
    assert_eq!(
        parts.len(),
        2,
        "expected two parts separated by dot, got: {name}"
    );
    for part in &parts {
        assert!(
            part.chars().all(|c| c.is_ascii_lowercase()),
            "expected lowercase part, got: {part}"
        );
    }
}
