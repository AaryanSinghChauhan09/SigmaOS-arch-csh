// Integration test for SigmaOS-csh
// Verifies the crate builds and links as a dependency.

use sigmaos_csh::CShell;

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut shell = CShell::new();

    // Act
    let output = shell.run_line("echo hello");

    // Assert
    assert_eq!(output.unwrap(), "hello");
}
