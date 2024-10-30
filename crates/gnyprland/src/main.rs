use astal::{Application, Exclusivity, Label, Window, WindowAnchor};

use astal::traits::WindowExt;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::{ContainerExt, LabelExt};
use gtk::{traits::*, CssProvider};

fn main() {
    gtk::init().unwrap();

    let app = Application::new();

    app.connect_activate(|app| {
        let window = Window::new();
        let label = Label::new();
        let css = CssProvider::new();

        css.load_from_data(b"label { font-size: 5rem; color: white; }")
            .unwrap();

        label.set_text("Hello, world!");
        label
            .style_context()
            .add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        window.set_anchor(WindowAnchor::Top + WindowAnchor::Left + WindowAnchor::Right);
        window.set_exclusivity(Exclusivity::Exclusive);
        window.set_child(Some(&label));

        app.add_window(&window);

        window.show_all();
    });

    app.run();
}
