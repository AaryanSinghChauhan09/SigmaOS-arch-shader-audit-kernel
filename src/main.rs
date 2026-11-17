//! SigmaOS-arch-shader-audit-kernel — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_arch_shader_audit_kernel_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-arch-shader-audit-kernel: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-arch-shader-audit-kernel: ok");
}
