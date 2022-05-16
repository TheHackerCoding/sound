mod ui;

use std::thread;

fn main() {
    ui::start_ui();
    let ui_thread = thread::spawn(move || {
        println!("pog");
    });

    ui_thread.join().unwrap();
    //println!("Hello, world!");
}
