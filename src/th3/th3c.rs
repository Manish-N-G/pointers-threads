/// The purpose of this is to try to implement a basic future, with 
/// our waker and executor. Depending on Poll directly
///
pub fn some_async() {
    // lets create our async future type here.
    // 1st we start by creating our future
    let fut = ThreadTimer::new(std::time::Duration::from_secs(3));
    println!("some_async res is {:?}",  our_executor(fut) );

}


fn our_executor<F: Future>(future: F) -> F::Output {
    // we need to pin our future in order to make sure that we dont
    // have issues with it in the future
    let mut fut_pin = std::pin::pin!(future);
    // Now we know that we should be able to call the poll method here

    // Next we an pass either our own waker or waker noop, which is a
    // no operation waker.
    // let waker = std::task::Waker::noop();
    // Here we prefer to make our own waker using thread park
    // and unpark to better utilize the cpu
    let waker: std::task::Waker = std::task::Waker::from(
        std::sync::Arc::new(ThreadWaker::get_current_thread_waker())
    );
    // could also do let waker = Arc::new(ThreadWaker::get_current_thread_waker()).into()
    let mut counter = 1;
    let val = loop {
        match fut_pin
            .as_mut()
            // .poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
            .poll(&mut std::task::Context::from_waker(&waker))
        {
            std::task::Poll::Ready(val) => break val,
            _  => {
                counter += 1;
                // we should not wake here, cause it doesnt really do anything.
                // we need to be parking thread thread and allow the waker that
                // is passed to the thread handle for the future, to wake the 
                // thread when its ready.
                // waker.clone().wake(); // works, but better to do the followin
                // waker.wake_by_ref();

                // updating this to thread park allows us to get the desired value
                // and doesnt unnecessary consume cpu time.
                // There can also be sporious waking up of threads which is possible.
                // and we need to keep a mind of this.
                std::thread::park();
            } 
        }
    };

    println!("broke loop for our_executor: counter is {:#}", counter);
    val
}


// Waker struct for Futures
// ==================================
// create custom waker that can be used for executors
// we pass the Thread directly here
struct ThreadWaker(std::thread::Thread);
// is generally struct Thread { inner: Pin<Arc<Inner, _>> }

impl ThreadWaker {
    // we will be able to attach the waker for the current
    // thread when we pass the Thread here
    fn get_current_thread_waker() -> Self {
        Self(std::thread::current())
    }
}

impl std::task::Wake for ThreadWaker {
    fn wake(self: std::sync::Arc<Self>) {
        self.0.unpark();
    }
}


// Future struct and method
// ==================================
// Created a TreadTimer type and implemented a future for it.
pub struct ThreadTimer {
    // initially was SystemTime, changed to Duration, should directly have systemtime
    duration: std::time::Duration,
    // I tried using tokio JoinHandle thinking that this will be ok as we are using
    // Tokio and std Thread Handle
    // After working with the code, I correct method is usind std::threads
    // thread_handle: Option<tokio::task::JoinHandle<()>>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    // This waker is what we will need to make eventually
    waker: std::sync::Arc<std::sync::Mutex<std::task::Waker>>,
    // just to know if a thread is complete or not.
    is_completed: std::sync::Arc<std::sync::Mutex<bool>>,
}


impl ThreadTimer {
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            // I didnt know if we needed systemtime of duration. I tested for 
            // systemtime, and getting some issues so swtiched it back to duration
            duration,
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
        // NOTE: this implementation, despite it working but wasnt 
        // sleeping the tread, is not the desired method and hence wrong
        // I have updated the implementation and not using the comented one.
        // if self.duration <= std::time::SystemTime::now() {
        //     *self.is_completed.lock().unwrap() = true;
        //     std::task::Poll::Ready(())
        // } else {
        //     // had to add thread park here at the end to make this
        //     // sleep till we get the value we want
        //     *self.waker.lock().unwrap() = cx.waker().clone();
        //     std::task::Poll::Pending
        // }

        let timer = self.get_mut();
        // its best to clone this part too. a concrete type is needed here
        *timer.waker.lock().unwrap() = cx.waker().clone();

        if timer.thread_handle.is_none() {
            let duration = timer.duration;
            let waker = timer.waker.clone();
            let is_completed = timer.is_completed.clone();

            timer.thread_handle = Some( std::thread::spawn( move || {
                // note, we dont need to have a tokio version here? 
                // still need to verify if there is even any
                // Also, thread sleep is not completely accurate. It could be longer
                std::thread::sleep( duration );
                *is_completed.lock().unwrap() = true;
                // we use wake by reference cause its not comsuming
                waker.lock().unwrap().wake_by_ref();
            }));
        }

        if *timer.is_completed.lock().unwrap() {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}


// ============================================================
pub struct TokioTimer {
    duration: std::time::Duration,
    thread_handle: Option<tokio::task::JoinHandle<()>>,
    waker: std::sync::Arc<std::sync::Mutex<std::task::Waker>>,
    is_completed: std::sync::Arc<std::sync::Mutex<bool>>,
}


impl TokioTimer {
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            duration,
            thread_handle: None,
            waker: std::sync::Arc::new(std::sync::Mutex::new(std::task::Waker::noop().clone())),
            is_completed: std::sync::Arc::new(std::sync::Mutex::new(false)),
        }
    }
}


impl Future for TokioTimer {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output>
    {
        let timer = self.get_mut();
        *timer.waker.lock().unwrap() = cx.waker().clone();

        if timer.thread_handle.is_none() {
            let duration = timer.duration;
            let waker = timer.waker.clone();
            let is_completed = timer.is_completed.clone();

            timer.thread_handle = Some( tokio::task::spawn( async move {
                // note: Here we use await. If this is not called, the async fn sleep
                // will not execute. awaiting the thread actually starts the sleep
                // timer. Also await is not thread blocking to the runtime can schedule
                // other tasks while this one is waiting.
                tokio::time::sleep(duration).await;
                // there is also tokio::sync::Mutex, but for our example, we currently
                // dont need it. it takes more resources as well, and the reason you
                // would want to use it is if we have locking in other tasks for
                // the .await points.
                *is_completed.lock().unwrap() = true;
                waker.lock().unwrap().wake_by_ref();
            }));
        }

        if *timer.is_completed.lock().unwrap() {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}


struct TaskWaker(std::task::Waker);

impl TaskWaker {
    fn from_context(cx: &std::task::Context<'_>) -> Self {
        Self(cx.waker().clone())
    }

    fn wake(self) {
        self.0.wake();
    }
}



// struct ThreadWaker(std::thread::Thread);
// // is generally struct Thread { inner: Pin<Arc<Inner, _>> }
//
// impl ThreadWaker {
//     // we will be able to attach the waker for the current
//     // thread when we pass the Thread here
//     fn get_current_thread_waker() -> Self {
//         Self(std::thread::current())
//     }
// }
//
// impl std::task::Wake for ThreadWaker {
//     fn wake(self: std::sync::Arc<Self>) {
//         self.0.unpark();
//     }
// }



fn our_tokio_executor<F: Future>(future: F) -> F::Output {
    let mut fut_pin = std::pin::pin!(future);
    // Now we know that we should be able to call the poll method here

    // Next we an pass either our own waker or waker noop, which is a
    // no operation waker.
    // let waker = std::task::Waker::noop();
    // Here we prefer to make our own waker using thread park
    // and unpark to better utilize the cpu
    let waker: std::task::Waker = std::task::Waker::from(
        std::sync::Arc::new(ThreadWaker::get_current_thread_waker())
    );
    // could also do let waker = Arc::new(ThreadWaker::get_current_thread_waker()).into()
    let mut counter = 1;
    let val = loop {
        match fut_pin
            .as_mut()
            // .poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
            .poll(&mut std::task::Context::from_waker(&waker))
        {
            std::task::Poll::Ready(val) => break val,
            _  => {
                counter += 1;
                // we should not wake here, cause it doesnt really do anything.
                // we need to be parking thread thread and allow the waker that
                // is passed to the thread handle for the future, to wake the 
                // thread when its ready.
                // waker.clone().wake(); // works, but better to do the followin
                // waker.wake_by_ref();

                // updating this to thread park allows us to get the desired value
                // and doesnt unnecessary consume cpu time.
                // There can also be sporious waking up of threads which is possible.
                // and we need to keep a mind of this.
                std::thread::park();
            } 
        }
    };

    println!("broke loop for our_executor: counter is {:#}", counter);
    val
}
