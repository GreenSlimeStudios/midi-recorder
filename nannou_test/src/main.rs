use nannou::prelude::*;
use rand::Rng;
use std::fs;

const NOTE_SPEED: f32 = 5.0; // speed of floating notes
const STARTING_NOTE: i32 = 21; // the note value of the first note on your midi device
const ENDING_NOTE: i32 = 108; // the note value of the last note on your midi device
const NOTE_MARGIN: f32 = 2.0; // margin between notes
const WIDTH_ADJUST: bool = true; // if false the notes are going to have a fixed width if true it will adjust to the window width
const NOTE_WIDTH: f32 = 10.0; // applies if WIDTH_ADJUST is set to false
                              // const PEDAL_NOTE:i8 = 64; // change this for your pedal note so the program ignores it
const USE_PARTICLES: bool = false; // Particles

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let win = app.window_rect();
    draw.background().color(BLACK);

    let mut note_multiplier: f32 = NOTE_WIDTH;
    if WIDTH_ADJUST == true {
        note_multiplier = win.w() / (ENDING_NOTE - STARTING_NOTE) as f32;
    }
    let half_width = win.w() / 2.0;

    for note in &_model.keys {
        // if note.note == PEDAL_NOTE {
        //     continue;
        // }
        draw.rect()
            .x_y(
                (note.note as f32 * note_multiplier)
                    - half_width
                    - (STARTING_NOTE as f32 * note_multiplier),
                note.y,
            )
            .w(note_multiplier - NOTE_MARGIN)
            .h(7.0)
            .hsv(note.note as f32 / 70.0, 1.0, 1.0);
    }
    if USE_PARTICLES == true {
        for particle in &_model.particles {
            draw.ellipse()
                .x_y(
                    (particle.x * note_multiplier)
                        - half_width
                        - STARTING_NOTE as f32 * note_multiplier,
                    particle.y,
                )
                .w(5.0)
                .h(5.0)
                .hsv(particle.x as f32 / 70.0, 1.0, 1.0);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
struct Note {
    note: i8,
    // x:f32,
    y: f32,
}
impl Note {
    fn new(n: i8, y: f32) -> Self {
        Self { note: n, y: y }
    }
    fn update(&mut self) {
        self.y += NOTE_SPEED;
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
}
#[derive(PartialEq)]
enum NoteThemes {
    RainbowHorizontal,
    Classic,
    RainbowVertical,
}
struct Particle {
    x: f32,
    y: f32,
    lifetime: f32,
    velocity: Vec2,
}
impl Particle {
    fn new(note: &Note, x_vel: f32) -> Self {
        Particle {
            velocity: Vec2::new(x_vel, 0.0),
            x: note.note as f32,
            y: note.y,
            lifetime: 0.0,
        }
    }
}
impl Particle {
    fn update(&mut self) {
        let mut rng = rand::thread_rng();
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
    frame: i128,
    particles: Vec<Particle>,
    // settings: Settings,
}
impl Model {
    fn new(_window: window::Id) -> Self {
        Self {
            _window: _window,
            keys: Vec::new(),
            frame: 0,
            particles: Vec::new(),
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    return Model::new(_window);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.frame += 1;

    let win = app.window_rect();

    let contents =
        fs::read_to_string("../midi/info.txt").expect("Something went wrong reading the file");

    let mut notes_string: Vec<&str> = contents.split("\n").collect();

    notes_string.pop();

    println!("{:?}", notes_string);

    for n in notes_string {
        model
            .keys
            .push(Note::new(n.parse().unwrap(), -win.h() / 2.0));
    }
    let mut deleted = 0;
    for i in 0..model.keys.len() {
        if model.keys[i - deleted].y > win.h() {
            if model.keys.len() > 1 {
                model.keys.remove(0);
                deleted += 1;
            }
        } else {
            model.keys[i - deleted].update();
            if USE_PARTICLES == true {
                if model.frame % 2 == 0 {
                    let mut rng = rand::thread_rng();
                    model.particles.push(Particle::new(
                        &model.keys[i - deleted],
                        rng.gen_range(-0.2..0.2),
                    ))
                }
            }
        }
    }
    if USE_PARTICLES == true {
        model
            .particles
            .sort_by(|a, b| a.lifetime.partial_cmp(&b.lifetime).unwrap());
        deleted = 0;
        for i in 0..model.particles.len() {
            if model.particles[i - deleted].lifetime > 5.0 {
                // if model.particles.len() > 1{
                // model.particles.sort_by(|a, b| a.lifetime.partial_cmp(&b.lifetime).unwrap());
                model.particles.pop();
                deleted += 1;
                // }
            } else {
                model.particles[i - deleted].update();
            }
        }
    }
}
