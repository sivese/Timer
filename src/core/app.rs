use gtk::prelude::*;

use std::sync::atomic::AtomicUsize;

pub struct App{ }

impl App {
    fn app_window() -> gtk::Window {
        let win = gtk::Window::new(gtk::WindowType::Toplevel);
        
        //let hb = gtk::HeaderBar::new();
        //hb.set_title(Some("1 HOUR - Timer"));

        //win.set_icon(icon);
        win.set_title("1 HOUR - Timer");
        win.set_position(gtk::WindowPosition::Center);
        win.set_default_size(600, 500);
        win.set_resizable(false);
        
        win
    }

    /*
                About Some() function
    
        Option document : https://doc.rust-lang.org/std/option/
    */
    pub fn run() {
        let win = App::app_window();
        let ui_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let timer = gtk::Label::new(Some("00:00:00"));
        
        let header = gtk::HeaderBar::new();
        header.set_title(Some("1 HOUR"));
        header.set_show_close_button(true);

        let start_button = gtk::Button::with_label("Start");
        //start_button.get_style_context().

        start_button.connect_clicked(move |_button|{

        });

        ui_box.add(&start_button);
        ui_box.add(&timer);
        
        win.set_titlebar(Some(&header));
        win.add(&ui_box);
        win.show_all();
    }
}

pub struct Time(AtomicUsize, usize);

impl Time {

}