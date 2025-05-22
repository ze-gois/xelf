pub mod stack;

pub unsafe fn copy(lhs: *mut u8, rhs: *const u8, n: usize) {
    (0..n).for_each(|i| *lhs.add(i) = *rhs.add(i));
}

pub unsafe fn set(former: *mut u8, value: u8, n: usize) {
    (0..n).for_each(|i| *former.add(i) = value);
}

pub unsafe fn length(s: *const u8) -> usize {
    let mut len = 0;

    while *s.add(len) != 0 {
        len += 1;
    }

    len
}
