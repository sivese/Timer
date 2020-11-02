extern crate gio;
extern crate gtk;

mod core;
mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

use crate::core::app;

use chrono::prelude::*;
use chrono::offset::LocalResult;
use std::{thread, time};

fn main() {
    env_logger::init();

    if let Ok(_) = gtk::init() {
    //    app::App::run();
    //    gtk::main();
    }

    thread::sleep(time::Duration::from_millis(500));

    let pivot = chrono::Local::now().timestamp_millis();

    loop {
        let current = chrono::Local::now().timestamp_millis() - pivot;
        let sec_unit = current/1000;
        let under_sec = current/10;
        println!("{}.{} seconds passed..", current/1000, under_sec - sec_unit*10);
    }

    /*
    let application =
        gtk::Application::new(Some("com.open.onehour.timer"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
    */
}
