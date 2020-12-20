use std::fs;
use std::path::Path;
use std::process::Command;

fn run_crate(path: &Path, args: &[&str]) {
    let output = Command::new("cargo")
        .current_dir(path)
        .args(&["run"])
        .arg("--")
        .args(args)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout:\n{}\n", stdout);
    eprintln!("stderr:\n{}\n", stderr);
    assert!(
        output.status.success(),
        "build failed: {} args: {:?}",
        path.display(),
        args
    );
}

#[test]
fn run_test_crates() {
    let tests = Path::new("tests");

    let crate_path = tests.join("test-crate-name-vs-pkg-name");
    let build_path = Path::new("build");
    let _ = fs::remove_dir_all(build_path);
    run_crate(&crate_path, &["build"]);
    assert!(
        build_path.exists(),
        "test for `crate-name-vs-pkg-name` failed"
    );

    let crate_path = tests.join("test-default-build-path");
    let build_path = crate_path.join("public");
    let _ = fs::remove_dir_all(&build_path);
    run_crate(&crate_path, &["build"]);
    assert!(build_path.exists(), "test for `default_build_path` failed");

    let crate_path = tests.join("test-prebuilt-wasm-opt");
    let build_path = crate_path.join("build");
    let _ = fs::remove_dir_all(&build_path);
    run_crate(&crate_path, &["build"]);
    assert!(build_path.exists(), "test for `prebuilt-wasm-opt` failed");

    let crate_path = tests.join("test-no-serve");
    let build_path = crate_path.join("build");
    let _ = fs::remove_dir_all(&build_path);
    run_crate(&crate_path, &["build"]);
    assert!(build_path.exists(), "test for `no-serve` failed");
}