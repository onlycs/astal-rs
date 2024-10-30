use astal::{Application, Exclusivity, Label, Window, WindowAnchor};

use astal::traits::{ApplicationExt, WindowExt};
use glib::prelude::*;
use gtk::prelude::{ContainerExt, LabelExt};
use gtk::traits::*;

fn main() {
    let mut app = Application::new();
    let window = Window::new();
    let label = Label::new();

    label.set_text("Hello, world!");

    window.set_anchor(WindowAnchor::Top + WindowAnchor::Left + WindowAnchor::Right);
    window.set_exclusivity(Exclusivity::Exclusive);
    window.set_child(Some(&label));

    app.add_window(&window);
}
