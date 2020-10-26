extern crate gtk;

use gtk::MessageDialog;

struct MessageBox;

impl MessageBox{
    fn show_msg(msg : String) {
        MessageDialog::new(None::<&gtk::Window>,
                            gtk::DialogFlags::empty(),
                            gtk::MessageType::Info,
                            gtk::ButtonsType::Ok,
                            msg.as_str());
    }
}