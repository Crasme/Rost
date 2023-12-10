pub const KILO_BYTE: usize = 1024;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
