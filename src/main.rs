use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation, glib};
use std::cell::Cell;
use std::rc::Rc;
// use gtk4 as gtk;

const APP_ID: &str = "me.manish-ach.FirstGtk";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    button_increase.connect_clicked(glib::clone!(
        #[weak]
        number,
        #[weak]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(glib::clone!(
        #[weak]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        // .default_width(100)
        // .default_height(50)
        .title("HelloWorld")
        .child(&gtk_box)
        .build();

    // this is used in case of using a closure instead of function for the windowUI
    // let button = Button::with_label("click me");
    // button.connect_clicked(|_| {
    //     button.set_label("HelloWorld");
    // });
    //
    // window.set_child(Some(&button));

    window.present();
}
