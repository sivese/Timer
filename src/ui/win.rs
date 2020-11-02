use gtk::*;

pub enum Msg{
    Quit,
    KeyPress
}

/*
pub struct Window {
    window : gtk::Window
}

impl Window {
    pub fn new() -> Window {
        let win = gtk::Window::new(gtk::WindowType::Toplevel);
        
        //let hb = gtk::HeaderBar::new();
        //hb.set_title(Some("1 HOUR - Timer"));

        //win.set_icon(icon);
        win.set_title("1 HOUR - Timer");
        win.set_position(gtk::WindowPosition::Center);
        win.set_default_size(600, 500);
        win.set_resizable(false);
        
        win.connect_delete_event(move |_, _| {
            gtk::main_quit();
            gtk::Inhibit(false);
        });
        
        Window{ win }
    }
}
*/

/*
    Hold off GTK header, now. It doesn't
    much make difference.
*/
struct Header {

}