//! This modue contains all chat related structures

mod imp;

use glib::Object;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::traits::WidgetExt;
use gtk::{glib, Align};

use crate::components::Message;

glib::wrapper! {
    /// Window associated to a particular chat in the client. Can be a private chat or a channel.
    ///
    /// Displays chat information and message history.
    ///
    /// User may send messages to given chat, emiting the 'send' signal.
    ///
    /// Has a single css node 'chat'
    ///
    /// Subclassifies [´gtk::Box´]
    pub struct Chat(ObjectSubclass<imp::Chat>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Chat {
    /// Creates a new [`Chat`] with the given name
    pub fn new(name: String) -> Self {
        Object::builder().property("name", name).build()
    }

    /// Connects to the `close` signal.
    pub fn connect_close<F>(&self, f: F)
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_local("close", true, move |args| {
            let chat: Chat = args[0].get().unwrap();
            f(&chat);
            None
        });
    }

    /// Connects to the `send` signal.
    pub fn connect_send<F>(&self, f: F)
    where
        F: Fn(&Self, String) + 'static,
    {
        self.connect_local("send", true, move |args| {
            let chat: Chat = args[0].get().unwrap();
            let message: String = args[1].get().unwrap();
            f(&chat, message);
            None
        });
    }

    /// Adds an external message to the chat.
    /// Does not have a sender, used only for private chats
    pub fn add_message(&self, message: String) {
        let message = create_external_message(message);

        self.imp().messages.append(&message);
    }

    /// Adds an external message to the chat,
    /// A sender is specified, used only for channel chats.
    pub fn add_message_with_sender(&self, message: String, sender: String) {
        let message = create_external_message(message);
        message.set_sender(sender);

        self.imp().messages.append(&message);
    }
}

/// Creates an external message
fn create_external_message(message: String) -> Message {
    let message = Message::new(message);
    message.set_halign(Align::Start);
    message.add_css_class("external");
    message
}

fn create_own_message(message: String) -> Message {
    let message = Message::new(message);
    message.set_halign(Align::End);
    message.add_css_class("own");
    message
}