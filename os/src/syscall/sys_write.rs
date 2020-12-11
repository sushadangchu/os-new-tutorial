pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> usize{
    if fd == 1 {
        let slice = unsafe { core::slice::from_raw_parts(buf, len) };
        let str = core::str::from_utf8(slice).unwrap();
        print!("{}", str);
    }
    len as usize
}