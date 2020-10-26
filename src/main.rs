extern crate gio;
extern crate gtk;

mod core;
mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

use crate::core::app;

fn main() {
    env_logger::init();

    if let Ok(_) = gtk::init() {
        app::App::run();
        gtk::main();
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
