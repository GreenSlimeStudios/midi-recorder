# midi-recorder
A program that prints out the notes in your terminal & graphicly like in a nice visual representation

# Usage
go to `midi` folder and run `cargo run --release` then pick your input midi device with a number.

To get a graphical representation then open another terminal and go to the **nannou_test** folder and run `cargo run --release`
this will open a nice window with falling notes!

# Themes
Here are some example themes screenshots for the program

rinbow_horizontal
![nice_rainbow_vertical](https://user-images.githubusercontent.com/86601983/177411099-a873c149-2c74-4fed-afc8-1133d8b53bdb.png)

rainbow_vertical
![nice_rainbow_horizontal](https://user-images.githubusercontent.com/86601983/177411042-dc343d2c-e707-4c73-9081-e7bf75314f8f.png)

classic (black notes on piano are red)
![nice_classic2](https://user-images.githubusercontent.com/86601983/177411143-a010c090-8629-471f-a174-7abc1b392d38.png)

There are more!

# Saving config files
Explore all of the options to fit your needs! (including particles, speed, margin and others)
You can configure options inside the expandable ui in the window or go into and configure the `nnou_test/config_user.txt` file.
this configuration loads on startup but you can create up to 6 other files in `nnou_test/config_slot_<0-5>.txt`
The configuration you get when clicking reset to defaults is coming from the `nannou_test/config1.txt` file
example file
```
note_speed: 5
starting_note: 21
ending_note: 108
note_margin: 2
use_width_adjust: true
note_width: 10
use_particles: false
theme: rainbow_horizontal
use_rounded_edges: true
```
# Notes in the terminal
You can also have a visual representatio in the terminal if you want to check that out go to **midi_play** folder and run `cargo run --release` and pick the new `midir reading input:midir-read-input` option.
After that there will start apperaing notes in the first terminal.
![image](https://user-images.githubusercontent.com/86601983/176659743-edd98498-944a-45b5-bc77-5ea751fa0625.png)
Thats all have fun!
