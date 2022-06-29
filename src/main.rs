use gdk::keys::constants::Escape;
use gdk::{ModifierType, WindowState};
use glib::translate::IntoGlib;
use gtk::prelude::*;
use gtk::{AccelFlags, AccelGroup, Button, ButtonBox, ColorButton, Window};
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    gtk::init().unwrap();

    let builder = gtk::Builder::from_string(include_str!("main.glade"));
    let window: Window = builder.object("window").unwrap();
    window.connect_hide(|_| gtk::main_quit());
    window.show_all();

    let bg_color = Rc::new(Cell::new(None));
    let accel_group = AccelGroup::new();
    {
        let window = window.clone();
        let bg_color = bg_color.clone();
        accel_group.connect_accel_group(
            Escape.into_glib(),
            ModifierType::empty(),
            AccelFlags::empty(),
            move |_, _, _, _| {
                if bg_color.get().is_some() {
                    window.unfullscreen();
                } else {
                    window.hide();
                }
                true
            },
        );
    }
    window.add_accel_group(&accel_group);

    let button_box: ButtonBox = builder.object("button_box").unwrap();
    let color_button: ColorButton = builder.object("color_button").unwrap();
    let fullscreen_button: Button = builder.object("fullscreen_button").unwrap();
    {
        let window = window.clone();
        fullscreen_button.connect_clicked(move |_| window.fullscreen());
    }

    {
        let bg_color = bg_color.clone();
        window.connect_window_state_event(move |_, event| {
            let state = event.new_window_state();
            if state.contains(WindowState::FULLSCREEN) {
                bg_color.set(Some(color_button.rgba()));
                button_box.hide();
            } else {
                bg_color.set(None);
                button_box.show();
            }
            Inhibit::default()
        });
    }
    {
        window.connect_draw(move |_, ctx| {
            if let Some(c) = bg_color.get() {
                ctx.set_source_rgba(c.red(), c.green(), c.blue(), c.alpha());
                ctx.paint().unwrap();
                Inhibit(true)
            } else {
                Inhibit::default()
            }
        });
    }

    gtk::main();
}
