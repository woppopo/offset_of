#![feature(const_maybe_uninit_assume_init)]
#![feature(const_ptr_offset_from)]

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        const _OFFSET: isize = {
            use std::mem::MaybeUninit;
            let b: MaybeUninit<$base> = MaybeUninit::uninit();
            let b = $crate::_assume_init_ref(&b);
            $crate::_offset(
                &b.$field as *const _ as *const u8,
                b as *const _ as *const u8,
            )
        };
        _OFFSET as usize
    }};
}

#[doc(hidden)]
pub const fn _assume_init_ref<T>(uninit: &std::mem::MaybeUninit<T>) -> &T {
    unsafe { uninit.assume_init_ref() }
}

#[doc(hidden)]
pub const fn _offset<T>(ptr: *const T, base: *const T) -> isize {
    unsafe { ptr.offset_from(base) }
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
