extern "C" {
    fn nocashMessage(msg: *const u8);
    fn consoleDemoInit();
    pub fn printf(msg: *const u8);
}
pub fn no_cash_message(msg: &str) {
    let mut msg_str: [u8; 256] = unsafe { core::mem::zeroed() };
    for (i, b) in msg.bytes().take(255).enumerate() {
        msg_str[i] = b;
    }
    // SAFETY: msg_str was originally filled with 256 0's, but we copied up to 255 bytes from msg to it. So at least the last byte is still 0
    unsafe {
        nocashMessage(msg_str.as_ptr());
        printf(msg.as_ptr());
    };
}
pub unsafe fn console_demo_init() {
    consoleDemoInit();
}
