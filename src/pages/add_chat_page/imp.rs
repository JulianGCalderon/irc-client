use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::subclass::Signal;
use gtk::prelude::{ObjectExt, StaticType, ToValue};
use gtk::subclass::prelude::*;
use gtk::{glib, template_callbacks, CompositeTemplate, Entry};

use crate::utils::get_and_clear_entry;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/add-chat-page.ui")]
pub struct AddChatPage {}

#[glib::object_subclass]
impl ObjectSubclass for AddChatPage {
    const NAME: &'static str = "AddChatPage";
    type Type = super::AddChatPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("add-chat-page");
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for AddChatPage {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("add")
                .param_types([String::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for AddChatPage {}
impl BoxImpl for AddChatPage {}

#[template_callbacks]
impl AddChatPage {
    #[template_callback]
    fn add_chat(&self, entry: Entry) {
        if let Some(name) = get_and_clear_entry(entry) {
            self.obj().emit_by_name("add", &[&name.to_value()])
        }
    }
}