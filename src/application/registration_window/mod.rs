use glib::Object;
use gtk::{gio, glib};

use crate::application::Application;

mod imp;

glib::wrapper! {
    pub struct RegistrationWindow(ObjectSubclass<imp::RegistrationWindow>)
        @extends gtk::Window, gtk::Widget,
        @implements
            gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
            gtk::Root, gtk::ShortcutManager;
}

impl RegistrationWindow {
    pub fn new(application: &Application) -> Self {
        Object::builder()
            .property("application", application)
            .build()
    }
}
