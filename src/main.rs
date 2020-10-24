extern crate gio;
extern crate gtk;

mod core;
mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("1 HOUR - Timer");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 300);


    let button = gtk::Button::with_label("Start Project");
    let countdown_label = gtk::Label::new(Some("00:00"));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);

    button.connect_clicked(move |_btn|{
        _btn.set_label("Pause");
        countdown_label.set_text("00:01");
    });

    vbox.add(&button);
    vbox.add(&countdown_label);
    
    window.add(&vbox);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.open.onehour.timer"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
