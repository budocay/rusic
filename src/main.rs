extern crate gio;
extern crate gtk;
mod toolbar;

use std::ffi::c_char;
use gtk::{Application, ApplicationWindow, IconSize, Image, SeparatorToolItem, Toolbar, ToolButton};
use gtk::builders::ImageBuilder;
use gtk::ffi::{gtk_image_new_from_icon_name, GTK_STOCK_OPEN, GtkIconSize};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, ImageExt, ImageExtManual, ToolButtonExt, WidgetExt};
use gtk::traits::{ContainerExt};

const PLAY_STOCK: &str = "gtk-media-play";

fn main() {
    //Create gtk application

    let application = Application::builder().application_id("com.github.gtk.basic").build();
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(application);
        window.set_title("Rusic");
        let toolbar = Toolbar::new();



        window.add(&toolbar);
        window.show_all();
    });
    application.connect_activate(|_|{});
    application.run();
}
