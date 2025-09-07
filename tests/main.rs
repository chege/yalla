// tests/it_cli.rs
use snapbox::cmd::{Command, cargo_bin};
use tempfile::tempdir;

/// Root (no args): clap shows help on stderr and exits with code 2.
#[test]
fn root_help() {
    let output = Command::new(cargo_bin("yalla"))
        .current_dir("tests/fixtures/basic")
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(2),
        "root should require a subcommand"
    );

    let err = String::from_utf8_lossy(&output.stderr);
    assert!(err.contains("Usage: yalla <COMMAND>"), "stderr:\n{err}");
    assert!(
        err.contains("script  Project scripts (also runnable)"),
        "stderr:\n{err}"
    );
    assert!(err.contains("tools   Developer tooling"), "stderr:\n{err}");
}

/// Namespace without child: `yalla ci` prints its own help and exits 2.
#[test]
fn namespace_help() {
    let output = Command::new(cargo_bin("yalla"))
        .current_dir("tests/fixtures/basic")
        .args(["ci"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(err.contains("Usage: yalla ci <COMMAND>"), "stderr:\n{err}");
    assert!(err.contains("build"), "stderr:\n{err}");
    assert!(err.contains("test"), "stderr:\n{err}");
}

/// Leaf runs: `yalla tools ls` should succeed and list the Yallafile in that dir.
#[test]
fn leaf_exec_tools_ls() {
    let output = Command::new(cargo_bin("yalla"))
        .current_dir("tests/fixtures/basic")
        .args(["tools", "ls"])
        .output()
        .unwrap();

    assert!(output.status.success(), "status: {:?}", output.status);
    let out = String::from_utf8_lossy(&output.stdout);
    assert!(
        out.lines().any(|l| l == "Yallafile"),
        "expected to see 'Yallafile' in stdout:\n{out}"
    );
}

/// Typo suggestion from clap.
#[test]
fn typo_suggestion() {
    let output = Command::new(cargo_bin("yalla"))
        .current_dir("tests/fixtures/basic")
        .args(["ci", "buil"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("unrecognized subcommand 'buil'"),
        "stderr:\n{err}"
    );
    assert!(
        err.contains("a similar subcommand exists: 'build'"),
        "stderr:\n{err}"
    );
}

/// Missing Yallafile: your app exits 0 and prints nothing.
#[test]
fn yallafile_missing_is_noop() {
    let tmp = tempdir().unwrap();

    let output = Command::new(cargo_bin("yalla"))
        .current_dir(tmp.path())
        .output()
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(0),
        "should exit 0 when no Yallafile"
    );
    assert!(
        output.stdout.is_empty() && output.stderr.is_empty(),
        "expected no output; got\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Unknown top-level subcommand errors (clap), code 2.
#[test]
fn unknown_top_level() {
    let output = Command::new(cargo_bin("yalla"))
        .current_dir("tests/fixtures/basic")
        .args(["nope"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(
        err.contains("unrecognized subcommand 'nope'"),
        "stderr:\n{err}"
    );
}
