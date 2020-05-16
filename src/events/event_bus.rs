use ::std::sync::{Arc, Mutex};
use ::std::collections::HashMap;

use ::std::{
    thread::{Builder, JoinHandle},
};

use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Sender, Receiver},
};

#[derive(PartialEq, Hash, Debug)]
pub enum Event {
    PathChange,
    PathError,
    FileDeleted,
}

impl Eq for Event {}

pub type Callback = Box<dyn Fn() + Send + Sync>;
pub type EventCallbacks = HashMap<Event, Callback>;

pub struct EventBus {
    tx: Option<Sender<Event>>,
    handle: Option<JoinHandle<()>>,
    event_callbacks: Arc<Mutex<EventCallbacks>>,
}

impl EventBus {
    pub fn new () -> Self {
        let (tx, mut rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(1);
        let mut runtime = Runtime::new().expect("could not create async runtime");
        let event_callbacks: Arc<Mutex<EventCallbacks>> = Arc::new(Mutex::new(HashMap::new()));
        let handle = Builder::new().name("event_executor".into()).spawn({
            let event_callbacks = event_callbacks.clone();
            move || {
                runtime.block_on(async {
                    while let Some(event) = rx.recv().await {
                        if let Some(event_cb) = event_callbacks.lock().unwrap().get_mut(&event) {
                            event_cb()
                        }
                    }
                });
            }
        }).expect("could not create blinking handler");
        Self {
            tx: Some(tx),
            handle: Some(handle),
            event_callbacks,
        }
    }
    pub fn subscribe(&mut self, blink_event: Event, cb: Callback) {
        // TODO: support multiple subscribers
        self.event_callbacks.lock().unwrap().insert(blink_event, cb);
    }
    pub fn publish(&mut self, blink_event: Event) {
        &self.tx.as_mut().expect("could not find tx channel").try_send(blink_event);
    }
}

impl Drop for EventBus {
    fn drop (&mut self) {
        // Do the Option dance to be able to drop the sender so that the receiver finishes and the thread can be joined (credit: @ebroto)
        drop(self.tx.take().unwrap());
        self.handle.take().unwrap().join().unwrap();
    }
}
