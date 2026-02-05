mod terminos;

use crate::terminos::{disable_echo, enable_echo};
use clearscreen::clear;
use include_dir::include_dir;
use io_redirect::Redirectable;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stderr, BufReader, Cursor, ErrorKind, Read, Write};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{exit, ChildStdout, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::spawn;
use std::{env, fs};
use viuer::{print, Config};
#[derive(Serialize, Deserialize)]
struct ProgConfig {
    password: Vec<u8>,
}
fn main() {
    let my_user = run_command("/usr/bin/whoami");
    if my_user.trim() != "root" {
        println!(
            "This project must be ran as root to function properly. Attempting to restart as root"
        );
        let cmdlineargs: Vec<String> = env::args().collect();
        let _output = Command::new("sudo").args(&cmdlineargs).exec(); // Bye bye never returns
        println!("Unknown error escalating to root!");
        exit(2);
    }
    let cmdlineargs: Vec<String> = env::args().collect();
    for arg in cmdlineargs.iter() {
        if arg.contains("init") {
            init();
            exit(0)
        }
    }
    let keyboards = get_keyboards();
    let mut config_opt = get_config();
    match config_opt.take() {
        Some(config) => {
            if !verify_config(&config) {
                config_fail();
            }
            ctrlc::set_handler(move || exit(0)).expect("Unable to configure cancel manager");
            clear().expect("TODO: panic message");
            for i in 1..10 {
                println!("You have {} seconds left", 10 - i);
                thread::sleep(std::time::Duration::from_millis(1000));
            }
            spawn(|| {
                let stream = stream_command("/usr/bin/sudo /usr/bin/cat /dev/input/mice");
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
            for keyboard in keyboards {
                let passwd = config.password.clone();
                spawn(move || {
                    let valid = passwd.clone();
                    let kbd_location = keyboard;
                    let stream = stream_command(
                        &*("/usr/bin/sudo /usr/bin/cat ".to_owned() + &*kbd_location),
                    );
                    let mut reader = BufReader::new(stream);
                    let mut collected_keys: Vec<u8> = vec![];
                    let mut index = 0;
                    loop {
                        disable_echo();
                        let mut buff: [u8; 100] = [0; 100];
                        reader.read(&mut buff).expect("Unable to open keyboard");
                        if !collected_keys.contains(&buff[20]) && valid[index] == buff[20] {
                            collected_keys.push(buff[20]);
                            index += 1;
                            if index >= valid.len() {
                                clear().expect("TODO: panic message");
                                thread::sleep(std::time::Duration::from_millis(10));
                                enable_input();
                                enable_echo();
                                println!("Authenticated!");
                                exit(0);
                            }
                        } else {
                            if !collected_keys.contains(&buff[20]) {
                                print!("invalid key");

                                insult()
                            }
                        }
                    }
                });
            }
        }
        None => config_fail(),
    }
    thread::sleep(std::time::Duration::from_millis(50));
    clear().expect("TODO: panic message");
    display_logo();
    loop {}
}
pub fn run_command(command: &str) -> String {
    let mut new_command = Command::new("/usr/bin/bash");
    new_command.arg("-c");
    new_command.arg(command);
    let output = new_command
        .output()
        .expect(&*("Failed to execute command: ".to_owned() + command));
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
}
pub fn stream_command(command: &str) -> ChildStdout {
    let mut new_command = Command::new("/usr/bin/bash");
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
        let protec = include_bytes!("../assets/fezprotec.png");
        let thing = image::load_from_memory(protec).unwrap();
        print(&thing, &conf).expect("Can't display logo");
    });
}
fn disable_input() {
    let mut keyboards = get_keyboards();
    let mut mice = get_mice();
    let mut devices: Vec<String> = get_mice();
    devices.append(&mut mice);
    devices.append(&mut keyboards);
    for device in devices {
        let mut new_command = Command::new("/usr/bin/evtest");
        new_command.arg("--grab");
        new_command.arg(device);
        let _output = new_command.stdout(Stdio::null()).spawn().expect("Fal");
    }
}
fn enable_input() {
    run_command("/usr/bin/killall evtest");
}
fn insult() {
    disable_input();
    clear().expect("TODO: panic message");
    let conf = Config {
        // Set dimensions.
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };
    let protec = include_bytes!("../assets/fezaaa.jpg");
    let thing = image::load_from_memory(protec).unwrap();
    print(&thing, &conf).expect("Can't display thing");
    let path = PathBuf::from("/dev/null");
    stderr().redirect(path.as_path()).unwrap();
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    enable_echo();
    let assets = include_dir!("assets");
    let wav_files = ["what.wav", "NOOOOOOO.wav", "AAA.wav"];
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());
    for file in wav_files {
        let cursor = Cursor::new(
            assets
                .get_file(file)
                .expect("Can't fetch audio file")
                .contents(),
        );
        let source = rodio::Decoder::new(BufReader::new(cursor)).expect("Can't start rodio");
        sink.append(source);
    }
    sink.sleep_until_end();
    thread::sleep(std::time::Duration::from_millis(2000));
    let cursor = Cursor::new(
        assets
            .get_file("yournotagoodperson.wav")
            .expect("Can't fetch audio file")
            .contents(),
    );
    let source = rodio::Decoder::new(BufReader::new(cursor)).expect("Can't start rodio");
    sink.append(source);
    sink.sleep_until_end();
    enable_input();
    run_command("systemctl suspend");
    exit(0)
}
fn get_config() -> Option<ProgConfig> {
    let content = fs::read_to_string("/etc/protec/config");
    match content {
        Ok(content) => {
            let config = serde_json::from_str::<ProgConfig>(content.as_str());
            match config {
                Ok(config) => Option::from(config),
                Err(_e) => None,
            }
        }
        Err(_e) => None,
    }
}
impl Default for ProgConfig {
    fn default() -> Self {
        let config: ProgConfig = ProgConfig { password: vec![] };
        config
    }
}
fn init() {
    if !fs::exists("/etc/protec").expect("Can't check for path") {
        run_command("mkdir /etc/protec");
    }
    let mut file = File::create("/etc/protec/config").expect("Can't create config file");
    let mut conf = ProgConfig::default();
    let mut valid_passwd = false;
    while !valid_passwd {
        println!(
            "Please enter the password you want to use to disarm the system. \nPress ESCAPE to confirm, all other keypresses will be included in the passwod"
        );
        conf.password = get_keys();
        println!("Type it again to verify");
        let temp = get_keys();
        if temp == conf.password {
            valid_passwd = true;
        } else {
            println!("Passwords did not match")
        }
    }
    let json = serde_json::to_string(&conf).expect("Can't serialize config");
    file.write(json.as_bytes()).expect("Can't write config");
}
fn verify_config(_config: &ProgConfig) -> bool {
    true
}
fn config_fail() {
    let cmdlineargs: Vec<String> = env::args().collect();
    let mut should_init = false;
    for arg in cmdlineargs.iter() {
        if arg.contains("init") {
            should_init = true;
            init();
            exit(0)
        }
    }
    println!("Config file is invalid!!! run protec init");
}
fn get_keyboards() -> Vec<String> {
    let mut keyboards: Vec<String> = vec![];
    let devices = run_command("/usr/bin/libinput list-devices");
    for device in devices.split("\n\n") {
        if device.contains("keyboard") {
            for line in device.lines() {
                if line.contains("Kernel") {
                    keyboards.push(line.to_string().split(" ").last().unwrap().to_string());
                }
            }
        }
    }
    keyboards
}
fn get_mice() -> Vec<String> {
    let mut mice: Vec<String> = vec![];
    let devices = run_command("/usr/bin/libinput list-devices");
    for device in devices.split("\n\n") {
        if device.contains("pointer") {
            for line in device.lines() {
                if line.contains("Kernel") {
                    mice.push(line.to_string().split(" ").last().unwrap().to_string());
                }
            }
        }
    }
    mice
}
fn get_keys() -> Vec<u8> {
    let keyboards = get_keyboards();

    let mut receivers: Vec<Receiver<Vec<u8>>> = vec![];
    for keyboard in keyboards {
        let (ch_tx, ch_rx) = channel();
        receivers.push(ch_rx);
        spawn(move || {
            let tx: Sender<Vec<u8>> = ch_tx;
            let kbd_location = keyboard;
            let stream =
                stream_command(&*("/usr/bin/sudo /usr/bin/cat ".to_owned() + &*kbd_location));
            let mut reader = BufReader::new(stream);
            let mut collected_keys: Vec<u8> = vec![];
            loop {
                disable_echo();
                let mut buff: [u8; 100] = [0; 100];
                reader.read(&mut buff).expect("Unable to open keyboard");
                if buff[20] as usize
                    == keyboard_codes::mapping::standard::key_to_code(
                        keyboard_codes::Key::Escape,
                        keyboard_codes::current_platform(),
                    )
                {
                    break;
                }
                if !collected_keys.contains(&buff[20]) {
                    collected_keys.push(buff[20]);
                }
                thread::sleep(std::time::Duration::from_millis(10));
            }
            tx.send(collected_keys).expect("Can't send");
        });
    }
    loop {
        for receiver in receivers.iter_mut() {
            match receiver.try_recv() {
                Ok(key) => {
                    return key;
                }
                _ => {}
            }
        }
    }
}
