// #![deny(warnings)]

#[macro_use]
extern crate log;
extern crate libc;
extern crate x11;

use std::process::Command;
use std::rc::Rc;
use std::sync::Mutex;

mod debug;
pub mod groups;
pub mod keys;
pub mod layout;
mod stack;
pub mod window;
pub mod x;

use groups::{Group, GroupWindow};
use keys::{KeyCombo, KeyHandler, KeyHandlers, ModKey};
use stack::Stack;
use window::Window;
use x::{Connection, Event, WindowId};


pub struct Config {
    pub keys: KeyHandlers,
}


pub struct RustWindowManager {
    connection: Rc<Connection>,

    config: Config,

    groups: Stack<Group>,
}

impl RustWindowManager {
    pub fn new(config: Config) -> Result<Self, String> {
        let connection = Connection::connect()?;
        connection.install_as_wm()?;
        let connection = Rc::new(connection);

        let mut groups = Stack::new();
        groups.push(Group::new("default", connection.clone()));

        Ok(RustWindowManager {
               connection: connection.clone(),

               config: config,

               groups: groups,
           })
    }

    pub fn group(&self) -> &Group {
        self.groups.focused().expect("No active group!")
    }

    pub fn group_mut(&mut self) -> &mut Group {
        self.groups.focused_mut().expect("No active group!")
    }

    pub fn run_event_loop(&mut self) {
        let event_loop_connection = self.connection.clone();
        let event_loop = event_loop_connection.get_event_loop();
        for event in event_loop {
            match event {
                Event::MapRequest(window_id) => self.on_map_request(window_id),
                Event::DestroyNotify(window_id) => self.on_destroy_notify(window_id),
                Event::KeyPress(key) => self.on_key_press(key),
                Event::EnterNotify(window_id) => self.on_enter_notify(window_id),
            }
        }
        info!("Event loop exiting");
    }

    fn on_map_request(&mut self, window_id: WindowId) {
        self.connection
            .register_window_events(&window_id, &self.config.keys);
        self.connection.map_window(&window_id);

        self.group_mut().add_window(window_id);
    }

    fn on_destroy_notify(&mut self, window_id: WindowId) {
        self.group_mut().remove_window(&window_id);
    }

    fn on_key_press(&mut self, key: KeyCombo) {
        self.config
            .keys
            .get(&key)
            .map(move |handler| (handler)(self));
    }

    fn on_enter_notify(&mut self, window_id: WindowId) {
        self.group_mut().focus(&window_id);
    }
}


pub fn close_window(wm: &mut RustWindowManager) {
    wm.group_mut().get_focused().map(|w| w.close());
}

pub fn focus_next(wm: &mut RustWindowManager) {
    wm.group_mut().focus_next();
}

pub fn focus_previous(wm: &mut RustWindowManager) {
    wm.group_mut().focus_previous();
}

pub fn shuffle_next(wm: &mut RustWindowManager) {
    wm.group_mut().shuffle_next();
}

pub fn shuffle_previous(wm: &mut RustWindowManager) {
    wm.group_mut().shuffle_previous();
}

pub fn layout_next(wm: &mut RustWindowManager) {
    wm.group_mut().layout_next();
}

pub fn spawn_command(command: Command) -> KeyHandler {
    let mutex = Mutex::new(command);
    Rc::new(move |wm| {
                let mut command = mutex.lock().unwrap();
                command.spawn().unwrap();
            })
}
