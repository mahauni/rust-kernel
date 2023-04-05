use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt =  InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub fn init_idt() {
    IDT.load();
}

#[test_case]
fn test_breakpoint_exeception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
