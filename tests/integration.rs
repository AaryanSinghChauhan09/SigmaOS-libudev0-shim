use sigmaos_libudev0_shim::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_libudev0_shim::Libudev0Shim::new("sigmaos-libudev0-shim").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-libudev0-shim"));
}
