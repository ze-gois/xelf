use crate::syscall;

pub unsafe fn print(mut b: *const u8) {
    while *b != 0 {
        syscall::write(1, b, 1);
        b = b.add(1);
    }
}

pub unsafe fn print_str(s: &str) {
    syscall::write(1, s.as_ptr(), s.len());
}

pub unsafe fn print_dec(value: u64) {
    let mut n = value;
    let mut buf = [0u8; 20]; // Max digits for u64
    let mut i = buf.len();

    if n == 0 {
        syscall::write(1, b"0".as_ptr(), 1).unwrap();
        return;
    }

    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }

    syscall::write(1, buf.as_ptr().add(i), buf.len() - i).unwrap();
}

pub unsafe fn print_hex(value: u64) {
    let mut buf = [0u8; 18]; // "0x" + 16 hex digits
    buf[0] = b'0';
    buf[1] = b'x';

    for i in 0..16 {
        let digit = ((value >> (60 - i * 4)) & 0xf) as u8;
        buf[i + 2] = if digit < 10 {
            b'0' + digit
        } else {
            b'a' + (digit - 10)
        };
    }

    syscall::write(1, buf.as_ptr(), buf.len()).unwrap();
}
