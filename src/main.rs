use chrono::{Local, NaiveTime};
use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{fs, thread, time};

// condiguration struct
struct Config {
    crns: Vec<String>,
}

impl Config {
    fn new() -> io::Result<Self> {
        if !Path::new("config.txt").exists() {
            println!("This will only run once, filling out info");
            println!("Enter your CRNs one at a time and type 'done' when finished");
            let mut crns = Vec::new();
            let mut i = 1;

            loop {
                print!("Enter CRN {} (or type 'done' to finish): ", i);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                if input.to_lowercase() == "done" {
                    break;
                }

                crns.push(input.to_string());
                i += 1;
            }

            let mut file = File::create("config.txt")?;
            for (i, crn) in crns.iter().enumerate() {
                writeln!(file, "crn{}={}", i + 1, crn)?;
            }
            println!("Config file created. Exiting program.");

            // return config
            Ok(Config { crns })
        } else {
            let file = File::open("config.txt")?;
            let reader = BufReader::new(file);
            let mut crns = Vec::new();

            for line in reader.lines() {
                let line = line?;
                if let Some(crn) = line.split('=').nth(1) {
                    crns.push(crn.trim().to_string());
                }
            }
            Ok(Config { crns })
        }
    }
}

// callback fuction handler for rdev
fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("Could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);
}

// write a given string
fn write_text(text: &str) {
    for c in text.chars() {
        // Convert character to appropriate key
        if let Some(key) = char_to_key(c) {
            send(&EventType::KeyPress(key));
            send(&EventType::KeyRelease(key));
        }
    }
}

fn char_to_key(c: char) -> Option<Key> {
    match c {
        'a' => Some(Key::KeyA),
        'b' => Some(Key::KeyB),
        'c' => Some(Key::KeyC),
        'd' => Some(Key::KeyD),
        'e' => Some(Key::KeyE),
        'f' => Some(Key::KeyF),
        'g' => Some(Key::KeyG),
        'h' => Some(Key::KeyH),
        'i' => Some(Key::KeyI),
        'j' => Some(Key::KeyJ),
        'k' => Some(Key::KeyK),
        'l' => Some(Key::KeyL),
        'm' => Some(Key::KeyM),
        'n' => Some(Key::KeyN),
        'o' => Some(Key::KeyO),
        'p' => Some(Key::KeyP),
        'q' => Some(Key::KeyQ),
        'r' => Some(Key::KeyR),
        's' => Some(Key::KeyS),
        't' => Some(Key::KeyT),
        'u' => Some(Key::KeyU),
        'v' => Some(Key::KeyV),
        'w' => Some(Key::KeyW),
        'x' => Some(Key::KeyX),
        'y' => Some(Key::KeyY),
        'z' => Some(Key::KeyZ),
        'A' => Some(Key::KeyA),
        'B' => Some(Key::KeyB),
        'C' => Some(Key::KeyC),
        'D' => Some(Key::KeyD),
        'E' => Some(Key::KeyE),
        'F' => Some(Key::KeyF),
        'G' => Some(Key::KeyG),
        'H' => Some(Key::KeyH),
        'I' => Some(Key::KeyI),
        'J' => Some(Key::KeyJ),
        'K' => Some(Key::KeyK),
        'L' => Some(Key::KeyL),
        'M' => Some(Key::KeyM),
        'N' => Some(Key::KeyN),
        'O' => Some(Key::KeyO),
        'P' => Some(Key::KeyP),
        'Q' => Some(Key::KeyQ),
        'R' => Some(Key::KeyR),
        'S' => Some(Key::KeyS),
        'T' => Some(Key::KeyT),
        'U' => Some(Key::KeyU),
        'V' => Some(Key::KeyV),
        'W' => Some(Key::KeyW),
        'X' => Some(Key::KeyX),
        'Y' => Some(Key::KeyY),
        'Z' => Some(Key::KeyZ),
        '0' => Some(Key::Num0),
        '1' => Some(Key::Num1),
        '2' => Some(Key::Num2),
        '3' => Some(Key::Num3),
        '4' => Some(Key::Num4),
        '5' => Some(Key::Num5),
        '6' => Some(Key::Num6),
        '7' => Some(Key::Num7),
        '8' => Some(Key::Num8),
        '9' => Some(Key::Num9),
        _ => None,
    }
}

// takes in config and calls writer fun iton
fn automate_registration(config: &Config) {
    println!("Executing registration automation...");

    // with tab after each
    for (i, crn) in config.crns.iter().enumerate() {
        write_text(crn);

        if i < config.crns.len() - 1 {
            send(&EventType::KeyPress(Key::Tab));
            send(&EventType::KeyRelease(Key::Tab));
        }
    }

    println!("Completed entering CRNs");
}

// hotkey listeneer
fn listen_for_hotkey(config: Arc<Config>) {
    let triggered = Arc::new(AtomicBool::new(false));
    let triggered_clone = triggered.clone();

    // for the modifier keys
    let meta_pressed = Arc::new(AtomicBool::new(false));
    let shift_pressed = Arc::new(AtomicBool::new(false));
    let meta_pressed_clone = meta_pressed.clone();
    let shift_pressed_clone = shift_pressed.clone();

    println!("Listening for CMD+SHIFT+O hotkey. Press it to activate CRN entry.");

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(Key::MetaLeft) | EventType::KeyPress(Key::MetaRight) => {
                meta_pressed_clone.store(true, Ordering::SeqCst);
            }
            EventType::KeyRelease(Key::MetaLeft) | EventType::KeyRelease(Key::MetaRight) => {
                meta_pressed_clone.store(false, Ordering::SeqCst);
            }
            EventType::KeyPress(Key::ShiftLeft) | EventType::KeyPress(Key::ShiftRight) => {
                shift_pressed_clone.store(true, Ordering::SeqCst);
            }
            EventType::KeyRelease(Key::ShiftLeft) | EventType::KeyRelease(Key::ShiftRight) => {
                shift_pressed_clone.store(false, Ordering::SeqCst);
            }
            EventType::KeyPress(Key::KeyO) => {
                if meta_pressed_clone.load(Ordering::SeqCst)
                    && shift_pressed_clone.load(Ordering::SeqCst)
                {
                    println!("Hotkey detected! Starting CRN entry...");
                    // Add a delay to allow CMD+SHIFT keys to be released
                    thread::sleep(time::Duration::from_millis(100));
                    automate_registration(&config);
                    triggered_clone.store(true, Ordering::SeqCst);
                }
            }
            _ => {}
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn main() -> io::Result<()> {
    let config = Config::new()?;

    let config_file_existed = Path::new("config.txt").exists();

    if !config_file_existed {
        println!("Config file has been created. Run the program again to use it.");
        return Ok(());
    }

    let config_arc = Arc::new(config);

    listen_for_hotkey(config_arc);

    Ok(())
}
