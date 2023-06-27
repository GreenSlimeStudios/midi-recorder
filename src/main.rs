extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use std::fs;
use std::fs::File;

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rand::{prelude::ThreadRng, Rng};


const NOTE_SPEED: f32 = 5.0; // speed of floating notes
const STARTING_NOTE: i32 = 21; // the note value of the first note on your midi device
const ENDING_NOTE: i32 = 108; // the note value of the last note on your midi device
const NOTE_MARGIN: f32 = 2.0; // margin between notes
const WIDTH_ADJUST: bool = true; // if false the notes are going to have a fixed width if true it will adjust to the window width
const NOTE_WIDTH: f32 = 10.0; // applies if WIDTH_ADJUST is set to false
                              // const PEDAL_NOTE:i8 = 64; // change this for your pedal note so the program ignores it
const USE_PARTICLES: bool = false; // Particles
const ROUNDED_NOTE_EDGES: bool = true; // spawns 2 elipses at the top and the bottom of the note to make it have round edges
const NOTE_THEME: NoteThemes = NoteThemes::RainbowHorizontal; // color theme of the notes

fn main() {
    let mut ofile = File::create("info.txt").expect("unable to create file");
    ofile.write_all("".as_bytes()).expect("unable to write");

    std::thread::spawn(|| {
        match run() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
    });
    nannou::app(model).update(update).run();
    //     loop {

    //         let contents =
    //             fs::read_to_string("info.txt").expect("Something went wrong reading the file");
    //         let mut notes_string: Vec<&str> = contents.split("\n").collect();
    //         notes_string.pop();
    //         // println!("{:?}",contents);

    //         let notes: Vec<i32>;

    //         if !notes_string.is_empty(){
    //             notes = notes_string
    //             .iter()
    //             .map(|x| x.parse::<i32>().unwrap())
    //             .collect();
    //         }
    //         else {
    //             notes = Vec::new();
    //         }
    // 
    //         // println!("{:?}",notes);
    //         display_board(&notes);
    //     }

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
            print!("Please select input port:\n");
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


fn view(app: &App, model: &Model, frame: Frame) {
    // println!("view start");
    let draw = app.draw();

    let settings = &model.settings;
    let win = app.window_rect();
    draw.background().color(BLACK);
    // println!("gut4");

    let mut note_multiplier: f32 = settings.note_width;
    if settings.use_width_adjust == true {
        note_multiplier = win.w() / (settings.ending_note - settings.starting_note) as f32;
    }
    let half_width = win.w() / 2.0;

    // println!("gut5");
    for note in &model.keys {
        // if note.note == PEDAL_NOTE {
        //     continue;
        // }
        draw.rect()
            .x_y(
                (note.note as f32 * note_multiplier)
                    - half_width
                    - (settings.starting_note as f32 * note_multiplier),
                note.y - note.length / 2.0,
            )
            .w(note_multiplier - settings.note_margin)
            .h(note.length)
            .hsv(
                get_color_h(&note, &settings.theme, &settings.black_keys),
                get_color_s(&note, &settings.theme, &settings.black_keys),
                1.0,
            );
        //.rotate(note.length);
        if settings.use_rounded_edges == true {
            draw.ellipse()
                .x_y(
                    (note.note as f32 * note_multiplier)
                        - half_width
                        - (settings.starting_note as f32 * note_multiplier),
                    note.y,
                )
                .w(note_multiplier - settings.note_margin)
                .h(note_multiplier / 2.0)
                .hsv(
                    get_color_h(&note, &settings.theme, &settings.black_keys),
                    get_color_s(&note, &settings.theme, &settings.black_keys),
                    1.0,
                );
            draw.ellipse()
                .x_y(
                    (note.note as f32 * note_multiplier)
                        - half_width
                        - (settings.starting_note as f32 * note_multiplier),
                    note.y - note.length,
                )
                .w(note_multiplier - settings.note_margin)
                .h(note_multiplier / 2.0)
                .hsv(
                    get_color_h(&note, &settings.theme, &settings.black_keys),
                    get_color_s(&note, &settings.theme, &settings.black_keys),
                    1.0,
                );
        }
    }

    // println!("gut6");
    if settings.use_particles == true {
        for particle in &model.particles {
            draw.ellipse()
                .x_y(
                    (particle.x * note_multiplier)
                        - half_width
                        - settings.starting_note as f32 * note_multiplier,
                    particle.y,
                )
                .w(5.0)
                .h(5.0)
                .hsv(
                    get_color_h_p(&particle, &settings.theme, &settings.black_keys),
                    get_color_s_p(&particle, &settings.theme, &settings.black_keys),
                    1.0,
                );
        }
    }
    // println!("gut7");
    // [destroyed object]: error 7: failed to import supplied dmabufs: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a
    draw.to_frame(app, &frame).unwrap();
    // println!("gut8");
    model.egui.draw_to_frame(&frame).unwrap();
    // println!("gut9");
}
fn get_color_h(note: &Note, theme: &NoteThemes, blacks: &Vec<i32>) -> f32 {
    if theme == &NoteThemes::RainbowHorizontal {
        return note.note as f32 / 70.0;
    }
    if theme == &NoteThemes::RainbowVertical {
        return note.y as f32 / 1400.0;
    }
    if theme == &NoteThemes::Classic {
        if blacks.contains(&(note.note as i32)) {
            return 0.0;
        } else {
            return 0.5;
        }
    }
    if theme == &NoteThemes::Halo {
        if blacks.contains(&(note.note as i32)) {
            return 0.6;
        } else {
            return 0.5;
        }
    }
    return 1.0;
}
fn get_color_s(note: &Note, theme: &NoteThemes, blacks: &Vec<i32>) -> f32 {
    if theme == &NoteThemes::RainbowHorizontal {
        return 1.0;
    }
    if theme == &NoteThemes::RainbowVertical {
        return 1.0;
    }
    if theme == &NoteThemes::Classic {
        if blacks.contains(&(note.note as i32)) {
            return 1.0;
        } else {
            return 0.0;
        }
    }
    if theme == &NoteThemes::Halo {
        if blacks.contains(&(note.note as i32)) {
            return 1.0;
        } else {
            return 0.0;
        }
    }
    return 1.0;
}
fn get_color_h_p(particle: &Particle, theme: &NoteThemes, blacks: &Vec<i32>) -> f32 {
    if theme == &NoteThemes::RainbowHorizontal {
        return particle.x as f32 / 70.0;
    }
    if theme == &NoteThemes::RainbowVertical {
        return particle.y as f32 / 1400.0;
    }
    if theme == &NoteThemes::Classic {
        if blacks.contains(&(particle.note as i32)) {
            return 1.0;
        } else {
            return 1.0;
        }
    }
    if theme == &NoteThemes::Halo {
        if blacks.contains(&(particle.note as i32)) {
            return 0.6;
        } else {
            return 0.6;
        }
    }
    return 1.0;
}
fn get_color_s_p(particle: &Particle, theme: &NoteThemes, blacks: &Vec<i32>) -> f32 {
    if theme == &NoteThemes::RainbowHorizontal {
        return 1.0;
    }
    if theme == &NoteThemes::RainbowVertical {
        return 1.0;
    }
    if theme == &NoteThemes::Classic {
        if blacks.contains(&(particle.note as i32)) {
            return 1.0;
        } else {
            return 0.0;
        }
    }
    if theme == &NoteThemes::Halo {
        if blacks.contains(&(particle.note as i32)) {
            return 1.0;
        } else {
            return 0.0;
        }
    }
    return 1.0;
}
struct Note {
    note: i8,
    // x:f32,
    y: f32,
    length: f32,
}
impl Note {
    fn new(n: i8, y: f32, note_speed: f32) -> Self {
        Self {
            note: n,
            y,
            length: note_speed,
        }
    }
    fn update(&mut self, note_speed: &f32) {
        self.y += note_speed;
    }
}
struct Settings {
    note_speed: f32,
    starting_note: i32,
    ending_note: i32,
    note_margin: f32,
    use_width_adjust: bool,
    note_width: f32,
    use_particles: bool,
    theme: NoteThemes,
    use_rounded_edges: bool,
    show_save_files: bool,
    show_theme_options: bool,
    black_keys: Vec<i32>,
}
impl Settings {
    fn from_consts() -> Self {
        Settings {
            black_keys: Vec::new(),
            note_speed: NOTE_SPEED,
            starting_note: STARTING_NOTE,
            ending_note: ENDING_NOTE,
            note_margin: NOTE_MARGIN,
            use_width_adjust: WIDTH_ADJUST,
            note_width: NOTE_WIDTH,
            use_particles: USE_PARTICLES,
            theme: NOTE_THEME,
            use_rounded_edges: ROUNDED_NOTE_EDGES,
            show_save_files: false,
            show_theme_options: false,
        }
    }
}
#[derive(PartialEq)]
enum NoteThemes {
    RainbowHorizontal,
    Classic,
    RainbowVertical,
    Halo,
}
struct Particle {
    note: i8,
    x: f32,
    y: f32,
    lifetime: f32,
    velocity: Vec2,
}
impl Particle {
    fn new(note: &Note, x_vel: f32, iterator: i32, note_speed: f32) -> Self {
        Particle {
            velocity: Vec2::new(x_vel, 0.0),
            note: note.note,
            x: note.note as f32,
            y: note.y - (note_speed * iterator as f32),
            lifetime: 0.0,
        }
    }
}
impl Particle {
    fn update(&mut self) {
        // self.velocity.x += rng.gen_range(-0.2..0.2);
        self.velocity.y += -0.8;
        self.x += self.velocity.x;
        self.y += self.velocity.y;
        self.lifetime += 1.0;
    }
}
struct Model {
    _window: window::Id,
    keys: Vec<Note>,
    keys_prev: Vec<String>,
    frame: i128,
    particles: Vec<Particle>,
    rng: ThreadRng,
    settings: Settings,
    egui: Egui,
}
impl Model {
    fn new(_window: window::Id, settings: Settings, egui: Egui) -> Self {
        Self {
            _window,
            keys: Vec::new(),
            keys_prev: Vec::new(),
            frame: 0,
            particles: Vec::new(),
            rng: rand::thread_rng(),
            settings,
            egui,
        }
    }
}
fn read_settings_from_file(path: &str, settings: &mut Settings) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the config file");
    let config_items: Vec<&str> = contents.split("\n").collect();
    for item in config_items {
        let values: Vec<&str> = item.split(" ").collect();
        println!("setting {:?}", values);
        match values[0] {
            "note_speed:" => settings.note_speed = values[1].parse().unwrap(),
            "starting_note:" => settings.starting_note = values[1].parse().unwrap(),
            "ending_note:" => settings.ending_note = values[1].parse().unwrap(),
            "note_margin:" => settings.note_margin = values[1].parse().unwrap(),
            "use_width_adjust:" => settings.use_width_adjust = values[1].parse().unwrap(),
            "note_width:" => settings.note_width = values[1].parse().unwrap(),
            "use_particles:" => settings.use_particles = values[1].parse().unwrap(),
            "theme:" => match values[1] {
                "rainbow_horizontal" => settings.theme = NoteThemes::RainbowHorizontal,
                "rainbow_vertical" => settings.theme = NoteThemes::RainbowVertical,
                "classic" => settings.theme = NoteThemes::Classic,
                "halo" => settings.theme = NoteThemes::Halo,
                _ => settings.theme = NoteThemes::RainbowHorizontal,
            },
            "use_rounded_edges:" => settings.use_rounded_edges = values[1].parse().unwrap(),
            _ => (),
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let mut settings: Settings = Settings::from_consts();
    read_settings_from_file("config_user.txt", &mut settings);

    let mut blacks = Vec::new();
    for i in 0..10 {
        for j in 0..12 {
            if [1, 3, 6, 8, 10].contains(&j) {
                blacks.push(i * 12 + j);
            }
        }
    }

    settings.black_keys = blacks;

    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    return Model::new(_window, settings, egui);
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // println!("starting update");
    model.frame += 1;

    let egui = &mut model.egui;
    let settings = &mut model.settings;
    let win = app.window_rect();

    // egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    // println!("pre egui");
    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Note Speed:");
        ui.add(egui::Slider::new(&mut settings.note_speed, 1.0..=40.0));

        // Scale slider
        ui.label("Note Margin:");
        ui.add(egui::Slider::new(&mut settings.note_margin, 0.0..=5.0));

        ui.label("Strting Note:");
        ui.add(egui::Slider::new(&mut settings.starting_note, 0..=200));

        // Rotation slider
        ui.label("Ending Note:");
        ui.add(egui::Slider::new(&mut settings.ending_note, 0..=200));

        let _particles = ui.checkbox(&mut settings.use_particles, "Particles");
        let _edges = ui.checkbox(&mut settings.use_rounded_edges, "Rounded edges");
        let _width_adjust = ui.checkbox(&mut settings.use_width_adjust, "auto width adjust");

        if settings.use_width_adjust == false {
            ui.label("Note width:");
            ui.add(egui::Slider::new(&mut settings.note_width, 0.0..=50.0));
        }
        // println!("e1");
        // ui.label("Theme");
        ui.checkbox(&mut &mut settings.show_theme_options, "Show Themes");
        if settings.show_theme_options {
            let button = ui.button("Rainbow horizontal").clicked();
            if button {
                settings.theme = NoteThemes::RainbowHorizontal;
            }
            let button = ui.button("Rainbow vertical").clicked();
            if button {
                settings.theme = NoteThemes::RainbowVertical;
            }
            let button = ui.button("Classic").clicked();
            if button {
                settings.theme = NoteThemes::Classic;
            }
            let button = ui.button("Halo").clicked();
            if button {
                settings.theme = NoteThemes::Halo;
            }
        }
        ui.label("Save Load options");

        // println!("e1");
        let reset_settings = ui.button("Reset to default");
        if reset_settings.clicked() {
            read_settings_from_file("config1.txt", settings);
        }

        let load_user_settings = ui.button("Load from save file");
        if load_user_settings.clicked() {
            read_settings_from_file("config_user.txt", settings);
        }
        let save_settings = ui.button("Save Settings");
        if save_settings.clicked() {
            save_settings_to_file("config_user.txt", &settings);
        }
        let _show_save_files_resp = ui.checkbox(&mut settings.show_save_files, "more save slots");

        // println!("e2");
        if settings.show_save_files {
            for i in 0..6 {
                let mut load_label: String = "Load from slot ".to_string();
                load_label.push_str(i.to_string().as_str());

                let mut file: String = "config_slot_".to_string();
                file.push_str(i.to_string().as_str());
                file.push_str(".txt");

                let load_user_settings = ui.button(load_label);
                if load_user_settings.clicked() {
                    read_settings_from_file(&file, settings);
                }
            }
            for i in 0..6 {
                let mut file: String = "config_slot_".to_string();
                file.push_str(i.to_string().as_str());
                file.push_str(".txt");

                let mut save_label: String = "Save to slot  ".to_string();
                save_label.push_str(i.to_string().as_str());

                let save_settings = ui.button(save_label);
                if save_settings.clicked() {
                    save_settings_to_file(&file, &settings);
                }
            }
        }
        // println!("e3");
    });
    // println!("post egui");
    let contents =
        fs::read_to_string("info.txt").expect("Something went wrong reading the file");

    // println!("got contents from file");
    let mut notes_string: Vec<&str> = contents.split("\n").collect();

    // println!("pre pop");
    notes_string.pop();
    // println!("post pop");

    // println!("{:?}", notes_string);

    for n in &notes_string {
        let mut is_note_existant: bool = false;
        for i in 0..model.keys_prev.len() {
            if model.keys_prev[i] == n.to_string() {
                is_note_existant = true;
            }
        }

        if is_note_existant {
            let mut index: usize = 0;
            let most_len: f32 = 0.0;
            for i in 0..model.keys.len() {
                if model.keys[i].note == n.parse::<i8>().unwrap() {
                    if model.keys[i].length > most_len {
                        index = i;
                    }
                }
            }
            model.keys[index].length += settings.note_speed;
        } else {
            model.keys.push(Note::new(
                n.parse().unwrap(),
                -win.h() / 2.0,
                settings.note_speed,
            ));
        }
    }
    // println!("gut");
    let mut deleted = 0;
    for i in 0..model.keys.len() {
        if model.keys[i - deleted].y > win.h() + model.keys[i - deleted].length {
            if model.keys.len() > 1 {
                model.keys.remove(0);
                deleted += 1;
            }
        } else {
            model.keys[i - deleted].update(&settings.note_speed);
            if settings.use_particles == true {
                if model.frame % 2 == 0 {
                    for j in 0..(model.keys[i - deleted].length / settings.note_speed as f32) as i32
                    {
                        model.particles.push(Particle::new(
                            &model.keys[i as usize - deleted],
                            model.rng.gen_range(-0.3..0.3),
                            j,
                            settings.note_speed,
                        ))
                    }
                }
            }
        }
    }
    if settings.use_particles == true {
        model
            .particles
            .sort_by(|a, b| a.lifetime.partial_cmp(&b.lifetime).unwrap());
        deleted = 0;
        for i in 0..model.particles.len() {
            if model.particles[i - deleted].lifetime > 3.0 {
                if model.particles.len() > 1 {
                    model.particles.pop();
                    deleted += 1;
                }
            } else {
                model.particles[i - deleted].update();
            }
        }
    }
    // println!("gut2");
    model.keys_prev = notes_string.into_iter().map(|x| x.to_string()).collect();
    // println!("gut3");
}

fn save_settings_to_file(path: &str, settings: &Settings) {
    let mut out: String = String::new();

    let mut value: String = "note_speed: ".to_string();
    value.push_str(settings.note_speed.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "starting_note: ".to_string();
    value.push_str(settings.starting_note.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "ending_note: ".to_string();
    value.push_str(settings.ending_note.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "note_margin: ".to_string();
    value.push_str(settings.note_margin.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "use_width_adjust: ".to_string();
    value.push_str(settings.use_width_adjust.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "note_width: ".to_string();
    value.push_str(settings.note_width.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "use_particles: ".to_string();
    value.push_str(settings.use_particles.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "theme: ".to_string();
    match settings.theme {
        NoteThemes::RainbowHorizontal => value.push_str("rainbow_horizontal"),
        NoteThemes::RainbowVertical => value.push_str("rainbow_vertical"),
        NoteThemes::Classic => value.push_str("classic"),
        NoteThemes::Halo => value.push_str("halo"),
        _ => value.push_str("rainbow_horizontal"),
    }
    out.push_str(&value);
    out.push_str("\n");

    let mut value: String = "use_rounded_edges: ".to_string();
    value.push_str(settings.use_rounded_edges.to_string().as_str());
    out.push_str(&value);
    out.push_str("\n");

    let mut ofile = File::create(path).expect("unable to create file");
    ofile.write_all(out.as_bytes()).expect("unable to write");
}
