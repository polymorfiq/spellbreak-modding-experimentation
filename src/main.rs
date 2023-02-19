use dll_syringe::{Syringe, process::OwnedProcess};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Find running Spellbreak Server process
    let target_process = OwnedProcess::find_first_by_name("g3Server").unwrap();
    

    // Get ready to inject DLL into server process
    let syringe = Syringe::for_process(target_process);
    
    // Inject DLL into running server process
    syringe.inject("./target/debug/spellbreak_mod.dll").unwrap();
    
    // Let's wait for CTRL+C so that we can output useful debug information.
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
}
