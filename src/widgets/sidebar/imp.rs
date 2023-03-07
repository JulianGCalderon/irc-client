use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::once_cell::sync::Lazy;
use gtk::glib::ParamSpec;
use gtk::prelude::{ObjectExt, ToValue};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, ListView, SingleSelection, Stack};

use super::SidebarProperty;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jgcalderon/irc-client/ui/sidebar.ui")]
pub struct Sidebar {
    #[template_child]
    pub config_list: TemplateChild<ListView>,
    #[template_child]
    pub chat_list: TemplateChild<ListView>,
    pub config_selection: RefCell<Option<SingleSelection>>,
    pub chat_selection: RefCell<Option<SingleSelection>>,
    stack: RefCell<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for Sidebar {
    const NAME: &'static str = "Sidebar";
    type Type = super::Sidebar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("sidebar")
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Sidebar {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(SidebarProperty::vec);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match SidebarProperty::from(pspec.name()) {
            SidebarProperty::Stack => {
                let stack: Stack = value.get().unwrap();
                self.stack.replace(stack);
            }
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match SidebarProperty::from(pspec.name()) {
            SidebarProperty::Stack => self.stack.borrow().to_value(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let sidebar = self.obj();

        sidebar.connect_notify(Some("stack"), |sidebar, _| sidebar.setup_stack());
    }
}

impl WidgetImpl for Sidebar {}
impl BoxImpl for Sidebar {}