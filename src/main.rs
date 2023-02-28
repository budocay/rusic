extern crate gio;
extern crate gtk;

use std::ffi::c_char;
use gtk::{Application, ApplicationWindow, IconSize, Image, Toolbar, ToolButton};
use gtk::builders::ImageBuilder;
use gtk::ffi::{gtk_image_new_from_icon_name, GTK_STOCK_OPEN, GtkIconSize};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, ImageExt, ToolButtonExt, WidgetExt};
use gtk::traits::{ContainerExt};

const PLAY_STOCK: &str = "gtk-media-play";

fn main() {
    //Create gtk application

    let application = Application::builder().application_id("com.github.gtk.basic").build();
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(application);
        window.set_title("Rusic");
        let toolbar = Toolbar::new();



        let open_image=  Image::new();
        open_image.set_from_file(Option::from("./src/ressources/open.png"));
        open_image.show();
        let open_button: ToolButton = ToolButton::new(Option::from(&open_image), Option::from("openButton"));
        toolbar.add(&open_button);
        window.add(&toolbar);
        window.show_all();
       // window.show()
    });
    application.connect_activate(|_|{});
    application.run();
}
