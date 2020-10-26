use gtk::prelude::*;

pub struct App{ }

impl App {
    fn app_window() -> gtk::Window {
        let win = gtk::Window::new(gtk::WindowType::Toplevel);
        
        //let hb = gtk::HeaderBar::new();
        //hb.set_title(Some("1 HOUR - Timer"));

        //win.set_icon(icon);
        win.set_title("1 HOUR - Timer");
        win.set_position(gtk::WindowPosition::Center);
        win.set_default_size(400, 600);
        win.set_resizable(false);
        
        win
    }

    pub fn run() {
        let win = App::app_window();
        let start_button = gtk::Button::with_label("Start");
        
        start_button.connect_clicked(move |_button|{

        });

        win.add(&start_button);
        win.show_all();
    }
}