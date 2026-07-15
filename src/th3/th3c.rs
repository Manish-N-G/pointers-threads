
/// The purpose of this is to try to implement a basic future, with 
/// our waker and executor. Depending on Poll directly
///
pub fn some_async() {
    // lets create our async future type here.

}





// ==================================
// Created a TreadTimer type and implemented a future for it.
pub struct ThreadTimer {
    // initially was duration, but we need system time here
    duration: std::time::SystemTime,
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
            // we need system time to see if its elapsed or not
            duration: std::time::SystemTime::now() + duration,
            thread_handle: None,
            // I wonder why clone works here. I would imagine we need to use to_owned instead
            // which also works. I just see clone takes &T and produces T.
            waker: std::sync::Arc::new(std::sync::Mutex::new(std::task::Waker::noop().clone())),
            is_completed: std::sync::Arc::new(std::sync::Mutex::new(false)),
        }
    }
}

impl Future for ThreadTimer {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output>
    {
        if self.duration <= std::time::SystemTime::now() {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}
