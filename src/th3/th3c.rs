
/// The purpose of this is to try to implement a basic future, with 
/// our waker and executor. Depending on Poll directly
///
pub fn some_async() {
    // lets create our async future type here.

}

pub struct ThreadTimer {
    duration: std::time::Duration,
    // I am using tokio JoinHandle thinking that this will be using
    // Tokio and std Thread Handle
    thread_handle: Option<std::sync::Arc<std::sync::Mutex<tokio::task::JoinHandle<()>>>>,
    // This waker is what we will need to make eventually
    waker: std::sync::Arc<std::sync::Mutex<std::task::Waker>>,
    // just to know if a thread is complete or not.
    is_completed: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl ThreadTimer {
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            duration,
            thread_handle: None,
            // I wonder why clone works here. I would imagine we need to use to_owned instead
            // which also works. I just see clone takes &T and produces T.
            waker: std::sync::Arc::new(std::sync::Mutex::new(std::task::Waker::noop().clone())),
            is_completed: std::sync::Arc::new(std::sync::Mutex::new(false)),
        }
    }
}

