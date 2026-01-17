mod terminos;

use crate::terminos::{disable_echo, enable_echo};
use clearscreen::clear;
use io_redirect::Redirectable;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{stderr, BufReader, ErrorKind, Read};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{exit, ChildStdout, Command, Stdio};
use std::thread;
use std::thread::spawn;
use viuer::{print_from_file, Config};

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

    clear().expect("TODO: panic message");
    for i in 1..10 {
        println!("You have {} seconds left", 10 - i);
        thread::sleep(std::time::Duration::from_millis(1000));
    }
    clear().expect("TODO: panic message");
    display_logo();
    spawn(|| {
        let stream = stream_command("sudo cat /dev/input/mouse1");
        let mut reader = BufReader::new(stream);
        loop {
            let mut buff: [u8; 1] = [0];
            reader
                .read(&mut buff)
                .expect("I guess you don't have a mouse lol");
            println!("bad no mouse");
            insult()
        }
    });
    let stream = stream_command("sudo cat /dev/input/by-path/platform-i8042-serio-0-event-kbd");
    let mut reader = BufReader::new(stream);
    let valid = [30, 22, 31, 20, 23, 49];
    let mut collected_keys: Vec<u8> = vec![];
    let mut index = 0;
    ctrlc::set_handler(move || {
        println!("Nice try");
    })
    .expect("Unable to configure cancel manager");
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

                insult()
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
fn display_logo() {
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
fn disable_input() {
    let regexs = [r"(pad\nKernel: *).*", r"(keyboard\nKernel: *).*"];
    for regex in regexs {
        let devices = run_command("libinput list-devices");
        let re = Regex::new(regex).expect("REASON");
        let mut results = vec![];
        for (test, [_]) in re.captures_iter(&*devices).map(|c| c.extract()) {
            results.push(test);
        }
        let mut new_command = Command::new("evtest");
        new_command.arg("--grab");
        new_command.arg(
            "/dev/input/".to_owned() + results.join("\n").split("/dev/input/").last().unwrap(),
        );
        let _output = new_command.stdout(Stdio::null()).spawn().expect("Fal");
    }
}
fn enable_input() {
    run_command("killall evtest");
}
fn insult() {
    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.

    disable_input();
    clear().expect("TODO: panic message");
    let conf = Config {
        // Set dimensions.
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };
    print_from_file("/etc/protec/fezaaa.jpg", &conf).expect("Image printing failed.");
    let path = PathBuf::from("/dev/null");
    stderr().redirect(path.as_path()).unwrap();
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    enable_echo();
    let what = BufReader::new(File::open("/etc/protec/what.wav").unwrap());
    let noooooo = BufReader::new(File::open("/etc/protec/NOOOOOOO.wav").unwrap());
    let pain = BufReader::new(File::open("/etc/protec/AAA.wav").unwrap());
    let bad = BufReader::new(File::open("/etc/protec/yournotagoodperson.wav").unwrap());
    // Note that the playback stops when the sink is dropped
    //run_command("amixer sset 'Master' 100%");
    let sink = rodio::play(&stream_handle.mixer(), what).unwrap();
    sink.sleep_until_end();
    let sink = rodio::play(&stream_handle.mixer(), noooooo).unwrap();
    sink.sleep_until_end();
    let sink = rodio::play(&stream_handle.mixer(), pain).unwrap();
    sink.sleep_until_end();
    thread::sleep(std::time::Duration::from_millis(2000));
    let sink = rodio::play(&stream_handle.mixer(), bad).unwrap();
    sink.set_volume(1.5);
    sink.sleep_until_end();
    enable_input();
    //run_command("systemctl suspend");
    exit(1)
}
