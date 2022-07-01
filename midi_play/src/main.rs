extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            let mut is_midir_found: bool = false;
            let mut midir_index = 100;
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
                if midi_out
                    .port_name(p)
                    .unwrap()
                    .starts_with("midir reading input")
                {
                    is_midir_found = true;
                    midir_index = i;
                }
            }
            if is_midir_found == true {
                println!("midir input port found, connecting...");
                out_ports
                    .get(midir_index)
                    .ok_or("invalid output port selected")?
            } else {
                println!("midir input port not found, make sure the midi program is running. If you are sure it is please select an output");
                print!("Please select output port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                out_ports
                    .get(input.trim().parse::<usize>()?)
                    .ok_or("invalid output port selected")?
            }
        }
    };

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);

            //let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        sleep(Duration::from_millis(4 * 150));

        loop {
            play_note(1);
            sleep(Duration::from_millis(50));
        }
    }
    sleep(Duration::from_millis(150));
    println!("\nClosing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
