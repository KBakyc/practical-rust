use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.object("applicationwindow1").unwrap();
    window.set_application(Some(app));
    // Center the window
    window.set_position(gtk::WindowPosition::CenterAlways);
    // Inputs
    let message_input: gtk::Entry = builder.object("message_input").unwrap();
    // Submit button
    let button: gtk::Button = builder.object("generate_btn").unwrap();
    // Outputs
    let message_output: gtk::Label = builder.object("message_output").unwrap();
    // Dead switch
    let is_dead_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();
    // Image output
    let image_output: gtk::Image = builder.object("image_output").unwrap();
    // low-cost clone (reference) for moved into the closure
    let image_output_clone = image_output.clone();

    button.connect_clicked(move |_| {
        message_output.set_text(&format!(
            "{}\n     \\\n      \\",
            message_input.text().as_str()
        ));

        // for release build set path to images in folder
        let img_path = if cfg!(debug_assertions) {
            String::new()
        } else {
            String::from("images/")
        };

        if is_dead_switch.is_active() {
            image_output_clone.set_from_file(Some(img_path + "cat_zombie_512x512.jpg"))
        } else {
            image_output_clone.set_from_file(Some(img_path + "cat_512x512.jpg"))
        }
        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let application = gtk::Application::new(Some("com.catsay-gui-glade"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
