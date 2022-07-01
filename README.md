# midi-recorder
A program that prints out the notes in your terminal & graphicly like in a nice visual representation

# Usage
go to **midi** folder and run `cargo run --release` then pick your input midi device with a number.
Then if you want a visual representatio in the terminal then go to **midi_play** folder and run `cargo run --release` and pick the new `midir reading input:midir-read-input` option.
After that there will start apperaing notes in the first terminal.
![image](https://user-images.githubusercontent.com/86601983/176659743-edd98498-944a-45b5-bc77-5ea751fa0625.png)
Now if you want a graphical representation then open another terminal and go to the **nannou_test** folder and run `cargo run --release`
this will open a nice window with falling notes!
![image](https://user-images.githubusercontent.com/86601983/176660475-6a0de4ca-90b4-4fb8-9aa3-a7b95ce52fa0.png)
You can configure options at with the constants at the start of the **nannou_test/src/main.rs** file.
Thats all have fun!
