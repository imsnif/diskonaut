use ::std::sync::{Arc, Weak, Mutex};
use ::std::time;
use ::tui::backend::Backend;
use ::std::thread::park_timeout;

use crate::app::App;

pub struct Blinker <B>
where B: Backend + 'static + Send
{
    app: Weak<Mutex<App<B>>>
}

impl <B>Blinker <B>
where B: Backend + 'static + Send
{
    pub fn new (app: &Arc<Mutex<App<B>>>) -> Self {
        let app = Arc::downgrade(app);
        Self {
            app
        }
    }

    pub fn blink_path_green (&self) -> Box<dyn Fn() + Send + Sync>
        where B: Backend + 'static + Send
    {
        Box::new({
            let app = self.app.clone();
            move || {
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.set_frame_around_current_path();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(50));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.remove_frame_around_current_path();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(50));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.set_frame_around_current_path();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(100));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.remove_frame_around_current_path();
                    app.render();
                }
            }
        })
    }

    pub fn blink_path_red (&self) -> Box<dyn Fn() + Send + Sync>
        where B: Backend + 'static + Send
    {
        Box::new({
            let app = self.app.clone();
            move || {
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.set_path_to_red();
                    app.set_frame_around_current_path();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(50));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.remove_frame_around_current_path();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(50));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.set_frame_around_current_path();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(100));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.reset_current_path_color();
                    app.remove_frame_around_current_path();
                    app.render();
                }
            }
        })
    }
    pub fn blink_space_freed(&self) -> Box<dyn Fn() + Send + Sync>
        where B: Backend + 'static + Send
    {
        Box::new({
            let app = self.app.clone();
            move || {
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.set_frame_around_space_freed();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(50));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.remove_frame_around_space_freed();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(50));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.set_frame_around_space_freed();
                    app.render();
                }
                park_timeout(time::Duration::from_millis(100));
                if let Some(app) = app.upgrade() {
                    let mut app = app.lock().unwrap();
                    app.remove_frame_around_space_freed();
                    app.render();
                }
            }
        })
    }
}
