extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use std::fs;
use std::fs::File;

fn main() {
    let mut ofile = File::create("info.txt").expect("unable to create file");
    ofile.write_all("".as_bytes()).expect("unable to write");

    // let mut active_notes:Vec<i32> = Vec::new();
    {
        match run() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut is_debug: bool = false;

    if args.contains(&"d".to_string()) {
        println!("Debug Mode is on");
        is_debug = true;
    }

    let contents =
        fs::read_to_string("whitelist.txt").expect("Something went wrong reading the config file");
    let mut whitelisted_inputs: Vec<&str> = contents.split("\n").collect();
    whitelisted_inputs.pop();
    // let whitelisted_inputs_u8: Vec<u8> = whitelisted_inputs.into().map(|x| x.parse.unwrap());

    let whitelisted_inputs_u8: Vec<u8> = whitelisted_inputs
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Whitelisted inputs\n{:?}", whitelisted_inputs_u8);

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let mut active_notes: Vec<i32> = Vec::new();

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            if message.len() == 3usize {
                //println!("{:?}", message);
                if message[1] != 1 {
                    if is_debug == true {
                        println!("{:?}", message);
                    }
                    if whitelisted_inputs_u8.contains(&message[0]) {
                        // this checks if the midi input is a note
                        handle_note(message[1].into(), &mut active_notes);
                        write_notes_to_file(&active_notes);
                    }
                } else {
                    display_board(&active_notes);
                    // println!("{:?}",active_notes);
                }
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

fn handle_note(note: i32, act_notes: &mut Vec<i32>) {
    // println!("{}",note);
    let mut had_note = false;
    for i in 0..act_notes.len() {
        if act_notes[i] == note {
            act_notes.remove(i);
            had_note = true;
            break;
        }
    }
    if had_note == false {
        act_notes.push(note);
    }
}

fn display_board(act_notes: &Vec<i32>) {
    for i in 21..=108 {
        if act_notes.contains(&(i as i32)) {
            print!("X");
        } else {
            print!(" ");
        }
    }
    println!();
    // write_notes_to_file(act_notes);
}
fn write_notes_to_file(act_notes: &Vec<i32>) {
    let mut out: String = String::new();

    for note in act_notes {
        out.push_str(&note.to_string());
        out.push_str("\n");
    }

    let mut ofile = File::create("info.txt").expect("unable to create file");
    ofile.write_all(out.as_bytes()).expect("unable to write");
}
