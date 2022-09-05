use std::rc::Rc;
use std::cell::Cell;

use glib;
use glib_macros::clone;
use gtk::{prelude::*, Orientation};
use gtk::{Application, ApplicationWindow, Button};

const APPLICATION_ID : &str = "org.troll.one_hour";

/*
    Widget gallery : https://docs.gtk.org/gtk4/visual_index.html
    
*/

fn main() {
    let app = Application::builder()
    .application_id(APPLICATION_ID)
    .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app : &Application) {
    let inc_button = Button::builder()
        .label("Increase")
        .build();

    let dec_button = Button::builder()
        .label("Decrease")
        .build();

    let num = Rc::new(Cell::new(0));

    inc_button.connect_clicked(clone!(@weak num, @weak dec_button => 
        move |_| {
            num.set(num.get() + 1);
            dec_button.set_label(&num.get().to_string());
    }));

    dec_button.connect_clicked(clone!(@weak inc_button => 
        move |_| {
            num.set(num.get() - 1);
            inc_button.set_label(&num.get().to_string());
    }));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    gtk_box.append(&inc_button);
    gtk_box.append(&dec_button);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("One Hour")
        .child(&gtk_box)
        .build();

    win.present();
}