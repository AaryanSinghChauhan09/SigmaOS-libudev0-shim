//! Integration tests for SigmaOS-libudev0-shim.

use sigmaos_libudev0_shim_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
