use gtk::gdk::Texture;
use gtk::gio::{self, ApplicationFlags};
use gtk::{prelude::*, Picture};
use gtk::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation};
use gtk4 as gtk;

fn main() {
    let application = Application::new(
        Some("com.shinglyu.catsay-gui"),
        ApplicationFlags::FLAGS_NONE,
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Catsay")
            .default_width(512)
            .default_height(552)
            .build();

        let layout_box = GtkBox::builder().orientation(Orientation::Vertical).build();
        let label = Label::builder().label("Meow!\n \\\n \\").build();
        layout_box.append(&label);

        let file = gio::File::for_path("./images/cat.jpg");
        let texture = Texture::from_file(&file).unwrap();
        let cat_image = Picture::new();
        cat_image.set_paintable(Some(&texture));
        cat_image.set_keep_aspect_ratio(true);
        layout_box.append(&cat_image);

        window.set_child(Some(&layout_box));
        window.show();
    });

    application.run();
}
