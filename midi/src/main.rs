extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Read, Write};

use midir::{Ignore, MidiInput};
use std::fs::File;
use std::thread;
use std::time::Duration;

// use std::sync::mutex;

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
    let mut active_notes2: Vec<i32> = Vec::new();
    // t

    // thread::spawn(move|| {
    //     loop {
    //         thread::sleep(Duration::from_millis(50));
    //         // println!("");

    //         // println!("{:?}",active_notes);
    //     }
    // });
    // loop{
    // active_notes = active_notes.clone();
    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            if message.len() == 3usize {
                //println!("{:?}", message);
                // active_notes2 = active_notes.clone();
                if message[1] != 1 {
                    if message[1] != 64 {
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
    // let t = _conn_in.close();
    // active_notes = active_notes.clone();
    // println!("{:?}",t.1);

    // }

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
    // println!("{:?}",act_notes);
    // if act_notes.contains(&note) {
    //     act_notes.po
    // }
}

fn display_board(act_notes: &Vec<i32>) {
    for i in 21..=108 {
        if act_notes.contains(&(i as i32)) {
            print!("X");
            // print!("   ")
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
        // out_notes.push(note.to_string());
        out.push_str(&note.to_string());
        out.push_str("\n");
    }

    let mut ofile = File::create("info.txt").expect("unable to create file");
    ofile.write_all(out.as_bytes()).expect("unable to write");
}
