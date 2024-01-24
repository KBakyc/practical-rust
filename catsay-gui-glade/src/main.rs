use gtk::{gdk::Texture, gio, prelude::*};
use gtk4 as gtk;

fn main() {
    let application = gtk::Application::new(Some("com.catsay-gui-glade"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.object("applicationwindow1").unwrap();
    window.set_application(Some(app));
    window.set_resizable(true);
    // Inputs
    let message_input: gtk::Entry = builder.object("message_input").unwrap();
    // Submit button
    let button: gtk::Button = builder.object("generate_btn").unwrap();
    let button_label = gtk::Label::new(Some("Generate"));
    button_label.set_markup("<span font='16'>Generate</span>");
    button.set_child(Some(&button_label));
    // Outputs
    let message_output: gtk::Label = builder.object("message_output").unwrap();
    // Dead switch
    let is_dead_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();
    // Image output
    let image_output: gtk::Picture = builder.object("image_output").unwrap();
    // low-cost clone (reference) for moved into the closure
    let image_output_clone = image_output.clone();

    button.connect_clicked(move |_| {
        message_output.set_markup(&format!(
            "<span font='16'>{}</span>\n     \\\n      \\",
            message_input.text().as_str()
        ));

        let file = if is_dead_switch.is_active() {
            "images/cat_zombie_512x512.jpg"
        } else {
            "images/cat_512x512.jpg"
        };
        let texture = Texture::from_file(&gio::File::for_path(file)).unwrap();
        image_output_clone.set_paintable(Some(&texture));
        // Set the size request for the image widget
        image_output_clone.set_size_request(texture.width(), texture.height());
        image_output_clone.show();
    });

    window.show();
    image_output.hide();
}
