#![feature(naked_functions)]
#![feature(asm_const)]
// use std::arch::asm;
use retour::static_detour;
use winapi::um::libloaderapi::GetModuleHandleA;

static NEXT_BOT_NAME_FN_OFFSET: usize = 0x9EDB70;

static_detour! {
    static NextBotName: extern "C" fn(usize, i32);
}

#[ctor::ctor]
fn ctor() {
    // Output debug message in Server's console
    println!("Succesfully injected spellbreak_mod into g3Server... This is in server's memory space.");

    let base_addr = unsafe { GetModuleHandleA(std::ptr::null()) };
    println!("INJECTED AT BASE ADDRESS: {:?}", base_addr);

    // Detour function call BotManager::set_NextBotNameIdx(self, value) so that it points at our own function
    unsafe {
        let next_bot_fn_addr = ((base_addr as usize) + NEXT_BOT_NAME_FN_OFFSET) as *const (usize, u32);
        NextBotName.initialize(std::mem::transmute(next_bot_fn_addr), next_bot_name_shiv).unwrap();
        NextBotName.enable().unwrap();
    }
}

fn next_bot_name_shiv(me: usize, value: i32) {
    // Let's do this instead of incrementing the counter
    println!("Overrode the thing! me: {:?}, value: {:?}", me, value);
}