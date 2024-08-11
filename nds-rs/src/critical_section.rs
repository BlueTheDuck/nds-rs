use critical_section::RawRestoreState;

const IME: *mut u32 = 0x0400_0208 as *mut _;

#[inline]
fn read_ime() -> bool {
    unsafe { IME.read_volatile() != 0 }
}

#[inline]
fn enable_ime() {
    unsafe { IME.write_volatile(1) }
}

#[inline]
fn disable_ime() {
    unsafe { IME.write_volatile(0) }
}

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        read_ime()
    }

    unsafe fn release(restore_state: RawRestoreState) {
        if restore_state {
            enable_ime();
        } else {
            disable_ime();
        }
    }
}
