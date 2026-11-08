//! Capture this application's own rendered window for documentation, without
//! capturing any other desktop window. Set RESOURCES_CAPTURE_PATH to a PNG path.
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk::prelude::PaintableExt, gio, glib};
use resources::application::Application;
use std::time::Duration;

fn main() {
    let path = std::env::var("RESOURCES_CAPTURE_PATH")
        .expect("Set RESOURCES_CAPTURE_PATH to the output PNG path");
    glib::timeout_add_local_once(Duration::from_secs(1), || {
        let app = gio::Application::default()
            .unwrap()
            .downcast::<Application>()
            .unwrap();
        let window = app.try_main_window().unwrap();
        window.set_default_size(1550, 1100);
        let stack = &window.imp().content_stack;
        for page in stack.pages().iter::<gtk::StackPage>().flatten() {
            let toolbar = page.child().downcast::<adw::ToolbarView>().unwrap();
            let id = toolbar.content().unwrap().property::<String>("tab-id");
            if resources::gui::ARGS.open_tab_id.as_deref() == Some(&id) {
                stack.set_visible_child(&toolbar);
                window
                    .imp()
                    .resources_sidebar
                    .set_selected_list_item_by_tab_id(&id);
            }
        }
    });
    glib::timeout_add_local_once(Duration::from_secs(10), move || {
        let app = gio::Application::default()
            .unwrap()
            .downcast::<Application>()
            .unwrap();
        let window = app.try_main_window().unwrap();
        let paintable = gtk::WidgetPaintable::new(Some(&window));
        let snapshot = gtk::Snapshot::new();
        paintable.snapshot(&snapshot, window.width() as f64, window.height() as f64);
        let node = snapshot.to_node().expect("Window has no rendered content");
        let renderer = window.renderer().expect("Window has no renderer");
        renderer
            .render_texture(&node, None)
            .save_to_png(&path)
            .unwrap();
        app.quit();
    });
    resources::gui::main();
}
