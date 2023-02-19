#![feature(naked_functions)]
#![feature(asm_const)]
use std::arch::asm;
use std::sync::atomic::{AtomicUsize, Ordering};
use retour::static_detour;

static bp_count: AtomicUsize = AtomicUsize::new(0);

static IDA_BASE_ADDR: usize = 0x7FF627040000;
static BASE_ADDR: usize = 0x7FF6B6A60000;
static NEXT_BOT_NAME_FN_OFFSET: usize = 0x9EDB70;

type NextBotNameFn = extern "C" fn(usize, i32);
static_detour! {
    static NextBotName: extern "C" fn(usize, i32);
}

#[ctor::ctor]
fn ctor() {
    // Output debug message in Server's console
    println!("Succesfully injected spellbreak_mod into g3Server... This is in server's memory space.");

    // Detour function call BotManager::set_NextBotNameIdx(self, value) so that it points at our own function
    unsafe {
        let next_bot_fn_addr = (BASE_ADDR + NEXT_BOT_NAME_FN_OFFSET) as *const (usize, u32);
        let fn_ptr: NextBotNameFn = std::mem::transmute(next_bot_fn_addr);
        NextBotName.initialize(fn_ptr, nextBotNameShiv).unwrap();
        NextBotName.enable();
    }
}

fn nextBotNameShiv(me: usize, value: i32) {
    // This occurs directly after incrementing the index. Let's decrement it
    unsafe {
        asm!("sub edx, 1");
    }

    // Let's do something more complicated and print a message from the server
    println!("Overrode the thing!");
}