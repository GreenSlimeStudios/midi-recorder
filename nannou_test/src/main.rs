use nannou::prelude::*;
use std::fs;

const NOTE_SPEED:f32 = 5.0;
const STARTING_NOTE:i32 = 21;
const ENDING_NOTE:i32 = 108;
const NOTE_MARGIN:f32 = 2.0;
const WIDTH_ADJUST:bool = false;
const NOTE_WIDTH:f32 = 10.0; //applies if WIDTH_ADJUST is set to false

fn main() {
    // let info = GlobalInfo{frame:10,notes:Vec::new};
    nannou::app(model).simple_window(view).update(update).run();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let win = app.window_rect();
    draw.background().color(BLACK);

    let mut note_multiplier:f32 = NOTE_WIDTH;
    if WIDTH_ADJUST == true{
        note_multiplier = win.w() / (ENDING_NOTE - STARTING_NOTE) as f32;
    }
    
    for note in &_model.keys {
        draw.rect()
            .x_y(
                (note.note as f32 * note_multiplier)
                    - (win.w() / 2.0)
                    - (STARTING_NOTE as f32 * note_multiplier),
                note.y,
            )
            .w(note_multiplier - NOTE_MARGIN)
            .h(7.0)
            .hsv(note.note as f32 / 70.0, 1.0, 1.0);
    }

    draw.to_frame(app, &frame).unwrap();
}
// #[derive(PartialEq)]
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

struct Model {
    _window: window::Id,
    keys: Vec<Note>,
    frame: i128,
}
impl Model {
    fn new(_window: window::Id) -> Self {
        Self {
            _window: _window,
            keys: Vec::new(),
            frame: 0,
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    return Model::new(_window);
}

fn update(app: &App, model: &mut Model, _update: Update) {
    //    _model.
    model.frame += 1;
    //if model.frame % 2 == 0
    {
        let win = app.window_rect();

        let contents =
            fs::read_to_string("../midi/info.txt").expect("Something went wrong reading the file");

        let mut notes_string: Vec<&str> = contents.split("\n").collect();
        // let mut info = Info::new();

        notes_string.pop();

        println!("{:?}", notes_string);

        for n in notes_string {
            model
                .keys
                .push(Note::new(n.parse().unwrap(), -win.h() / 2.0));
            // println!("gut {}", n);
        }
        let mut deleted = 0;
        for i in 0..model.keys.len() {
            // note.update();
            if model.keys[i - deleted].y > 1000.0 {
                if model.keys.len() > 1 {
                    model.keys.remove(0);
                    deleted += 1;
                }
            } else {
                model.keys[i - deleted].update();
            }
        }
    }
}
