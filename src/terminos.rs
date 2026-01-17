use std::fs::File;
use std::os::unix::io::AsRawFd;
use termios::*;

pub fn disable_echo() {
    let stdin = File::open("/dev/stdin").expect("failed to open /dev/stdin");
    let fd = stdin.as_raw_fd();

    let mut original_termios = Termios::from_fd(fd).unwrap();
    tcgetattr(fd, &mut original_termios).expect("failed to set termios");
    // Create a copy of settings and disable ECHO
    let mut new_termios = original_termios;
    new_termios.c_lflag &= !ECHO; // Disable echo and canonical mode
    tcsetattr(fd, TCSANOW, &new_termios).expect("failed to set termios");
}

pub fn enable_echo() {
    let stdin = File::open("/dev/stdin").expect("failed to open /dev/stdin");
    let fd = stdin.as_raw_fd();

    let mut original_termios = Termios::from_fd(fd).unwrap();
    tcgetattr(fd, &mut original_termios).expect("failed to set termios");

    // Create a copy of settings and enable ECHO
    let mut new_termios = original_termios;
    new_termios.c_lflag &= ECHO; // enable echo and canonical mode
    tcsetattr(fd, TCSANOW, &new_termios).expect("failed to set termios");
}
