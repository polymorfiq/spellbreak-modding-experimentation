#![feature(naked_functions)]
#![feature(asm_const)]
use std::arch::asm;
use retour::static_detour;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

// IDA base address: 0x7FF627040000
static NEXT_BOT_NAME_FN_OFFSET: usize = 0x9EDB70;

type NextBotNameFn = extern "C" fn(usize, i32);
static_detour! {
    static NextBotName: extern "C" fn(usize, i32);
}

#[ctor::ctor]
fn ctor() {
    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) };
    println!("INJECTING AT BASE ADDRESS: {:?}", base_addr);

    // Output debug message in Server's console
    println!("Succesfully injected spellbreak_mod into g3Server... This is in server's memory space.");

    // Detour function call BotManager::set_NextBotNameIdx(self, value) so that it points at our own function
    unsafe {
        let next_bot_fn_addr = ((base_addr as usize) + NEXT_BOT_NAME_FN_OFFSET) as *const (usize, u32);
        let fn_ptr: NextBotNameFn = std::mem::transmute(next_bot_fn_addr);
        NextBotName.initialize(fn_ptr, nextBotNameShiv).unwrap();
        NextBotName.enable();
    }
}

fn nextBotNameShiv(me: usize, value: i32) {
    // Let's do this instead of incrementing the counter
    println!("Overrode the thing!");
}