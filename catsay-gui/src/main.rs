use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation, WindowPosition};
fn main() {
    let app = Application::new(Some("com.shinglyu.catsay-gui"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(480, 480);
        window.set_position(WindowPosition::Center);

        let layout_box = GtkBox::new(Orientation::Vertical, 0);
        let label = Label::new(Some("Meow!\n \\\n \\"));
        layout_box.add(&label);

        // let cat_image = Image::from_file("./images/cat.jpg");
        let pixbuf = Pixbuf::from_file_at_scale("./images/cat.jpg", 480, 480, true).unwrap();
        let cat_image = gtk::Image::from_pixbuf(Some(&pixbuf));

        layout_box.add(&cat_image);
        window.add(&layout_box);

        window.show_all();
    });
    app.run();
}
