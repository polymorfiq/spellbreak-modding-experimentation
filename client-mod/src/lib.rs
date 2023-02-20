#![feature(naked_functions)]
#![feature(asm_const)]
// use std::arch::asm;
use winapi::um::libloaderapi::GetModuleHandleA;

#[ctor::ctor]
fn ctor() {
    // Output debug message in Server's console
    println!("Succesfully injected spellbreak_client_mod into Spellbreak Client... This is in client's memory space.");

    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) };
    println!("INJECTED AT BASE ADDRESS: {:?}", base_addr);
}