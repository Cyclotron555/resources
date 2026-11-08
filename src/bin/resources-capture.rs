//! Capture this application's own rendered window for documentation, without
//! capturing any other desktop window. Set RESOURCES_CAPTURE_PATH to a PNG path.
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gdk::prelude::PaintableExt, gio, glib};
use resources::application::Application;
use resources::ui::pages::gpu_comparison::ResGPUComparison;
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
    if std::env::var_os("RESOURCES_CAPTURE_VERIFY").is_some() {
        glib::timeout_add_local_once(Duration::from_secs(4), || {
            let app = gio::Application::default()
                .unwrap()
                .downcast::<Application>()
                .unwrap();
            let window = app.try_main_window().unwrap();
            let toolbar = window
                .imp()
                .content_stack
                .visible_child()
                .unwrap()
                .downcast::<adw::ToolbarView>()
                .unwrap();
            let comparison = toolbar
                .content()
                .unwrap()
                .downcast::<ResGPUComparison>()
                .unwrap();
            let panels = comparison.imp().panels.borrow();
            assert!(panels.len() >= 2);
            let panels: Vec<_> = panels.values().collect();
            let scale = panels[0].imp().temperature.graph().imp().max_y.get();
            assert!(scale.unwrap() >= 100.0);
            for panel in &panels {
                assert_eq!(panel.imp().temperature.graph().imp().max_y.get(), scale);
            }
            assert_ne!(
                panels[0].imp().gpu_usage.graph().imp().graph_color.get(),
                panels[1].imp().gpu_usage.graph().imp().graph_color.get()
            );
            let adjustments: Vec<_> = panels
                .iter()
                .map(|panel| {
                    panel
                        .child()
                        .and_downcast::<gtk::ScrolledWindow>()
                        .unwrap()
                        .vadjustment()
                })
                .collect();
            let target = adjustments
                .iter()
                .map(|a| a.upper() - a.page_size())
                .fold(64.0, f64::min);
            assert!(target > 0.0);
            adjustments[0].set_value(target);
            for adjustment in &adjustments {
                assert!((adjustment.value() - target).abs() < 1.0);
            }
            adjustments[1].set_value(target / 2.0);
            for adjustment in &adjustments {
                assert!((adjustment.value() - target / 2.0).abs() < 1.0);
            }
            adjustments[0].set_value(0.0);
            eprintln!(
                "Comparison checks passed: shared temperature range, distinct colors, synchronized scrolling"
            );
        });
    }
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
