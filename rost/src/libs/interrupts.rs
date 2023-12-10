use core::any;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};

use lazy_static::lazy_static;

use crate::libs::gdt;
use crate::println;

use pic8259::ChainedPics;
use spin::Mutex;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(any_interrupt);
        idt.debug.set_handler_fn(any_interrupt);
        idt.non_maskable_interrupt.set_handler_fn(any_interrupt);
        idt.breakpoint.set_handler_fn(any_interrupt);
        idt.overflow.set_handler_fn(any_interrupt);
        idt.bound_range_exceeded.set_handler_fn(any_interrupt);
        idt.invalid_opcode.set_handler_fn(any_interrupt);
        idt.device_not_available.set_handler_fn(any_interrupt);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        };
        /* Coprocessor segment overrun */ // not in there, we are not in 1857
        idt.invalid_tss.set_handler_fn(any_interrupt_with_err_code);
        idt.segment_not_present.set_handler_fn(any_interrupt_with_err_code);
        idt.stack_segment_fault.set_handler_fn(any_interrupt_with_err_code);
        idt.general_protection_fault.set_handler_fn(any_interrupt_with_err_code);
        idt.page_fault.set_handler_fn(pagefault_handler);
        /* RESERVED */
        idt.x87_floating_point.set_handler_fn(any_interrupt);
        idt.alignment_check.set_handler_fn(any_interrupt_with_err_code);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(any_interrupt);
        idt.virtualization.set_handler_fn(any_interrupt);
        idt.cp_protection_exception.set_handler_fn(any_interrupt_with_err_code);
        /* RESERVED */
        idt.hv_injection_exception.set_handler_fn(any_interrupt);
        idt.vmm_communication_exception.set_handler_fn(any_interrupt_with_err_code);
        idt.security_exception.set_handler_fn(any_interrupt_with_err_code);
        /* RESERVED */
        /* TRIPLE FAULT */ // cant handle
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::SlaveInterrupt.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::SerialPort2.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::SerialPort1.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::SoundCard.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::FloppyDisk.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::Parralel1.as_usize()].set_handler_fn(any_interrupt);
    
        idt[InterruptIndex::RealTimeClock.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::ACPI.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::AnyPeripheral1.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::AnyPeripheral2.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::Mouse.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::CoProcessor.as_usize()].set_handler_fn(any_interrupt);
        idt[InterruptIndex::ATAPrimary.as_usize()].set_handler_fn(ataprimary_interrupt_handler);
        idt[InterruptIndex::ATASecondary.as_usize()].set_handler_fn(any_interrupt);

        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    SlaveInterrupt,
    SerialPort2, // & Serial port 4 if present
    SerialPort1, // & Serial port 3 if present
    SoundCard, // Or parralel port 3
    FloppyDisk,
    Parralel1, // shared with parralel 2 if present

    RealTimeClock = PIC_2_OFFSET,
    ACPI, // Advanced Configuration and Power Interface
    AnyPeripheral1,
    AnyPeripheral2,
    Mouse,
    CoProcessor, // Or FPU
    ATAPrimary,
    ATASecondary,

    AnyInterrupt,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn pagefault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {

    unsafe {
        PICS.lock().notify_end_of_interrupt(0xE);
    }

    panic!("EXCEPTION: PAGEFAULT : {:#?}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: PAGEFAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {

    use crate::drivers::keyboard;

    keyboard::run_handlers(&KEYBOARD);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn ataprimary_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Should we really ignore this?
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::ATAPrimary.as_u8());
    }
}

extern "x86-interrupt" fn any_interrupt(_stack_frame: InterruptStackFrame) {
    println!("Unknown interrupt just happened :(");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::AnyInterrupt.as_u8());
    }
}

extern "x86-interrupt" fn any_interrupt_with_err_code(_stack_frame: InterruptStackFrame, error_code: u64) {
    println!("Error coded {} interrupt just happened :(", error_code);
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::AnyInterrupt.as_u8());
    }
}

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> =
        Mutex::new(
            Keyboard::new(layouts::Azerty, ScancodeSet1, HandleControl::Ignore)
        );
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


// INIT

pub fn init_idt() {
    IDT.load();
}
