use sigmaos_libudev0_shim::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let shim = Libudev0Shim::new();
    // Act
    let ok = shim.check_symbol("udev_device_new_from_syspath").expect("symbol resolves");
    // Assert
    assert!(ok);
}
