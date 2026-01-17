mod terminos;

use clearscreen::clear;
use std::env;
use std::io::{BufReader, ErrorKind, Read};
use std::os::unix::process::CommandExt;
use std::process::{exit, ChildStdout, Command, Stdio};
use std::thread;
use std::thread::spawn;
use viuer::{print_from_file, Config};
use crate::terminos::{disable_echo, enable_echo};

fn main() {
    let my_user = run_command("whoami");
    if my_user.trim() != "root" {
        println!(
            "This project must be ran as root to function properly. Attempting to restart as root"
        );
        let cmdlineargs: Vec<String> = env::args().collect();
        let _output = Command::new("sudo").args(&cmdlineargs).exec(); // Bye bye never returns
        println!("Unknown error escalating to root!");
        exit(2);
    }
    ctrlc::set_handler(move || {
        println!("Cancelling!");
        enable_echo();
        exit(130)
    }).expect("Unable to configure cancel manager");
    clear().expect("TODO: panic message");
    for i in 1..10 {
        println!("You have {} seconds left", 10 - i);
        thread::sleep(std::time::Duration::from_millis(1000));
    }
    clear().expect("TODO: panic message");
    diplay_logo();
    spawn(|| {
        let stream = stream_command("sudo cat /dev/input/mouse1");
        let mut reader = BufReader::new(stream);
        loop {
            let mut buff: [u8; 1] = [0];
            reader
                .read(&mut buff)
                .expect("I guess you don't have a mouse lol");
            println!("bad no mouse");
            enable_echo();
            run_command("systemctl suspend");
            exit(1)
        }
    });
    let stream = stream_command("sudo cat /dev/input/by-path/platform-i8042-serio-0-event-kbd");
    let mut reader = BufReader::new(stream);
    let valid = [30, 22, 31, 20, 23, 49];
    let mut collected_keys: Vec<u8> = vec![];
    let mut index = 0;
    let mut try_num=0;
    loop {
        disable_echo();
        let mut buff: [u8; 100] = [0; 100];
        reader.read(&mut buff).expect("Unable to open keyboard");
        if !collected_keys.contains(&buff[20]) && valid[index] == buff[20] {
            collected_keys.push(buff[20]);
            index += 1;
            if index >= valid.len() {
                println!("Authenticated!");
                enable_echo();
                exit(0);
            }
        } else {
            if !collected_keys.contains(&buff[20]) {
                print!("invalid key");
                if try_num >1 {
                    enable_echo();
                    run_command("systemctl suspend");
                    exit(1)
                }
                else {
                    try_num+=1;
                }
            }
        }
    }
}
pub fn run_command(command: &str) -> String {
    let mut new_command = Command::new("bash");
    new_command.arg("-c");
    new_command.arg(command);
    let output = new_command
        .output()
        .expect(&*("Failed to execute command: ".to_owned() + command));
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
}
pub fn stream_command(command: &str) -> ChildStdout {
    let mut new_command = Command::new("bash");
    new_command.arg("-c");
    new_command.arg(command);
    let output = new_command
        .stdout(Stdio::piped())
        .spawn()
        .expect(&*("Failed to execute command: ".to_owned() + command))
        .stdout
        .ok_or_else(|| std::io::Error::new(ErrorKind::Other, "Could not capture standard output."));
    output.unwrap()
}
fn diplay_logo() {
    spawn(|| {
        let conf = Config {
            // Set dimensions.
            width: Some(40),
            height: Some(25),
            ..Default::default()
        };
        print_from_file("/etc/protec/fezprotec.png", &conf).expect("Image printing failed.");
    });
}
