use std::{io, thread, time};
use std::io::Write;

pub(crate) fn sleep(ms: u32) {
    let time = time::Duration::from_millis(ms.into());
    thread::sleep(time);
}

pub(crate) fn clearscreen() {
    print!("{esc}c", esc = 27 as char);
    io::stdout().flush().unwrap();
}