use gtk::{IconSize, Image, SeparatorToolItem, Toolbar, ToolButton};
use gtk::prelude::{ContainerExt, ImageExt, ImageExtManual, WidgetExt};

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();
        // Open Folder Image and Button
        let open_image=  Image::new();
        open_image.set_from_file(Option::from("./src/ressources/OpenFolder.png"));
        open_image.set_icon_size(IconSize::SmallToolbar);
        open_image.show();
        let open_button: ToolButton = ToolButton::new(Option::from(&open_image), Option::from("openButton"));
        toolbar.add(&open_button);


        // Previous Image and button
        let previous_image=  Image::new();
        previous_image.set_from_file(Option::from("./src/ressources/previous.png"));
        previous_image.set_icon_size(IconSize::SmallToolbar);
        previous_image.show();
        let previous_button = ToolButton::new(Option::from(&previous_image), Option::from("previous"));
        toolbar.add(&previous_button);

        // Play Image and Button
        let play_image=  Image::new();
        play_image.set_from_file(Option::from("./src/ressources/play.png"));
        play_image.set_icon_size(IconSize::SmallToolbar);
        play_image.show();
        let play_button = ToolButton::new(Option::from(&play_image), Option::from("play"));
        toolbar.add(&play_button);

        // Stop Image and Button
        let stop_image=  Image::new();
        stop_image.set_from_file(Option::from("./src/ressources/stop.png"));
        stop_image.set_icon_size(IconSize::SmallToolbar);
        stop_image.show();
        let stop_button = ToolButton::new(Option::from(&stop_image), Option::from("stop"));
        toolbar.add(&stop_button);

        // Next Image and button
        let next_image=  Image::new();
        next_image.set_from_file(Option::from("./src/ressources/next.png"));
        next_image.set_icon_size(IconSize::SmallToolbar);
        next_image.show();
        let next_button = ToolButton::new(Option::from(&next_image), Option::from("next"));
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        // Remove Image and Button
        let remove_image=  Image::new();
        remove_image.set_from_file(Option::from("./src/ressources/remove.png"));
        remove_image.set_icon_size(IconSize::SmallToolbar);
        remove_image.show();
        let remove_button = ToolButton::new(Option::from(&remove_image), Option::from("remove"));
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        //Quit Image and Button
        let quit_image=  Image::new();
        quit_image.set_from_file(Option::from("./src/ressources/quit.png"));
        quit_image.set_icon_size(IconSize::SmallToolbar);
        quit_image.show();
        let quit_button = ToolButton::new(Option::from(&quit_image), Option::from("quit"));
        toolbar.add(&quit_button);

        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar
        }
    }

    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}