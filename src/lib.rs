#![feature(const_maybe_uninit_assume_init)]
#![feature(const_ptr_offset_from)]

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        const _OFFSET: isize = {
            use std::mem::MaybeUninit;
            let b: MaybeUninit<$base> = MaybeUninit::uninit();
            unsafe {
                let b = b.assume_init_ref();
                (&b.$field as *const _ as *const u8).offset_from(b as *const _ as *const u8)
            }
        };
        _OFFSET as usize
    }};
}

#[test]
fn test() {
    #[repr(C)]
    struct Test {
        test1: u32,
        test2: u32,
    }

    assert!(offset_of!(Test, test2) == std::mem::size_of::<u32>());
}
