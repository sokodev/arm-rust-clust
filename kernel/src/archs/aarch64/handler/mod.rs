
extern "C" {
    static vector_table_el1: *mut u64;
}

pub fn init() {
    use core::ptr::read_volatile;

    unsafe {
        init_vbar(vector_table_el1);
        // FIXME: ensures vector_table_el1 is linked
        let _x = read_volatile(vector_table_el1);
    }
}

fn init_vbar(vba: *mut u64) {
    // we need a stack
    unsafe {
        asm!("      msr vbar_el1, $0" :: "r"(vba) );
    }
}

#[no_mangle]
pub extern "C" fn handler() -> u64 {
    loop {}
}

pub fn svc() {
    unsafe {
        asm!("svc 1");
    }
}


global_asm!(
    r#"
    .section        .handler
    .global         vector_table_el1
    .extern         handler

    .balign         0x800
    vector_table_el1:

    /* Exception taken from EL1 with SP_EL0. */
    /* Synchronous */
    sp0_sync:       ldr     x0, 0
                    bl      handler
                    b       .
    /* IRQ or vIRQ */
    .balign         0x80
    sp0_irq:        ldr     x0, 0
                    b       .
    /* FIQ or vFIQ */
    .balign         0x80
    sp0_fiq:        ldr     x0, 0
                    b       .
    /* SError or vSError */
    .balign         0x80
    sp0_serror:     ldr     x0, 0
                    b       .

    .balign         0x80
    /* Exception taken from EL1 with SP_EL1. */
    /* Synchronous */
    sp1_sync:       ldr     x0, 0
                    b       .
    /* IRQ or vIRQ */
    .balign         0x80
    sp1_irq:        ldr     x0, 0
                    b       .
    /* FIQ or vFIQ */
    .balign         0x80
    sp1_fiq:        ldr     x0, 0
                    b       .
    /* SError or vSError */
    .balign         0x80
    sp1_serror:     ldr     x0, 0
                    b       .

    .balign         0x80
    /* Exception taken from EL0/AArch64. */
    /* Synchronous */
    el0_64_sync:    ldr     x0, 0
                    b       .
    /* IRQ or vIRQ */
    .balign         0x80
    el0_64_irq:     ldr     x0, 0
                    b       .
    /* FIQ or vFIQ */
    .balign         0x80
    el0_64_fiq:     ldr     x0, 0
                    b       .
    /* SError or vSError */
    .balign         0x80
    el0_64_serror:  ldr     x0, 0
                    b       .

    .balign         0x80
    /* Exception taken from EL0/AArch32. */
    /* Synchronous */
    el0_32_sync:    ldr     x0, 0
                    b       .
    /* IRQ or vIRQ */
    .balign         0x80
    el0_32_irq:     ldr     x0, 0
                    b       .
    /* FIQ or vFIQ */
    .balign         0x80
    el0_32_fiq:     ldr     x0, 0
                    b       .
    /* SError or vSError */
    .balign         0x80
    el0_32_serror:  ldr     x0, 0
                    b       .
    "#
);
