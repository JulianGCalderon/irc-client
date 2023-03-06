//! This modules defines [`UserData`]

mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use constant::UserDataProperty;

glib::wrapper! {
    /// Used to store user registration data
    ///
    /// Derives GObject, therefore it can comunicate well with Gtk4 rust bindings.
    pub struct UserData(ObjectSubclass<imp::UserData>);
}

impl UserData {
    /// Creates a new [`UserData`] with the given properties
    pub fn new(
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    ) -> Self {
        Object::builder()
            .property(&UserDataProperty::Nickname, nickname)
            .property(&UserDataProperty::Realname, realname)
            .property(&UserDataProperty::Username, username)
            .property(&UserDataProperty::Hostname, hostname)
            .property(&UserDataProperty::Servername, servername)
            .build()
    }

    /// Shortcut to access `nickname` property faster
    pub fn nickname(&self) -> String {
        self.property(&UserDataProperty::Nickname)
    }
}

impl Default for UserData {
    fn default() -> Self {
        Object::builder().build()
    }
}