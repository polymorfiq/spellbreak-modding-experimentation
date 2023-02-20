use dll_syringe::{Syringe, process::OwnedProcess};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut injected = false;

    // Inject into running server process
    let server_process = OwnedProcess::find_first_by_name("g3Server-Win64-Test");
    if server_process.is_some() {
        let syringe = Syringe::for_process(server_process.unwrap());
        syringe.inject("./target/debug/spellbreak_server_mod.dll").unwrap();
        println!("Injected into Spellbreak Server!");

        injected = true;
    }

    // Find running Spellbreak Client process
    let client_process = OwnedProcess::find_first_by_name("g3-Win64-Test");
    if client_process.is_some() {
        let syringe = Syringe::for_process(client_process.unwrap());
        syringe.inject("./target/debug/spellbreak_client_mod.dll").unwrap();
        println!("Injected into Spellbreak Client!");

        injected = true;
    }

    // If we found neither, let the user know
    if !injected {
        println!("Could not find any running Spellbreak executables!");
    }

    // Let's wait for CTRL+C so that we can output useful debug information.
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
}
