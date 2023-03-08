//! This modules defines the [`PasswordField`] widget

mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::ObjectExt;

glib::wrapper! {
    /// This widgets is used to ask for user for a password, indicating the name of the variable.
    ///
    /// Password is hidden by default, although user can manually invert it.
    ///
    /// Subclassifies [´gtk::Box`]
    pub struct PasswordField(ObjectSubclass<imp::PasswordField>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl PasswordField {
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Gets the user input
    pub fn input(&self) -> String {
        let input: String = self.property("input");

        input
    }
}

impl Default for PasswordField {
    fn default() -> Self {
        Self::new()
    }
}