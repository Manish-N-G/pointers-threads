#![allow(unused)]
use trpl;

pub fn test_async_spawn() {
    println!("For one----");
    one();
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    println!("\nFor one_a----");
    one_a();
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    let (a, b) = (500u64, 500u64);
    println!("\nFor two: val1 {a} and val2 {b} ----");
    two(a,b);
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    let (a, b) = (1000u64, 500u64);
    println!("\nFor two: val1 {a} and val2 {b} ----");
    two(a,b);
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    println!("\nFor three----");
    three();
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("\nFor three_a----");
    three_a();
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    println!("\nFor four----");
    four();
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn one() {
    trpl::run( async {
        let tsk = trpl::spawn_task( async {
            for x in 1..10 {
                println!("[First-one]:{}", x);
                trpl::sleep(std::time::Duration::from_millis(500)).await
            }
        }); // this creates a task. and goes ot the next for loop
        // the task is running in the background and will run till the 
        // main function ends.
        for x in 1..5 {
            println!("[Second-one]:{}", x);
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }    
        // we tell this task (sub task spawned from main task) that we 
        // need to await the tsk here and cant proceed till its complete.
        tsk.await;
    });
}

fn one_a() {
    trpl::run( async {
        let tsk = trpl::spawn_task( async {
            for x in 1..10 {
                println!("[First-one_a]:{}", x);
                trpl::sleep(std::time::Duration::from_millis(500)).await
            }
        }); // this creates a task, and goes to the next for loop block.
        // This task is running in the background and will run till the end
        // of the trpl::run. It doesnt need be be till the end of main block
        for x in 1..5 {
            println!("[Second-one_a]:{}", x);
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }    
        // As you see,we dont use tsk await here, and this process will continue
        // upto this point. This will not leak into the main process as tsk get dropped
        // at this point. When dropped, this task gets concelled, and is not carried on
        // to the main task/process. This implementation is different form the thread
        // spawn, where, thread will linger in the back ground and the thread only gets 
        // dropped when the main process is completed.
    });
}

fn two(val1: u64, val2: u64) {
    trpl::run( async {
        let fut1 = async {
            for x in 1..10 {
                println!("[First-two]:{} for {val1}", x);
                trpl::sleep(std::time::Duration::from_millis(val1)).await
            }
        };
        let fut2 = async {
            for x in 1..5 {
                println!("[Secont-two]:{} for {val2}", x);
                trpl::sleep(std::time::Duration::from_millis(val2)).await
            }
        };
        // here we use thread join. From my understanding, we will not
        // be producing two spawned tasks, however, this is run inside the
        // main task. Here join works concurrently with fut1 and fut2, not
        // necessarly parallely. This means that join, internally, process a
        // single future, while consuming the other futures. and inside this
        // future, code is concurrently run. From my understanding, this code
        // what uses join should run a bit slower that if we were to 
        // implement task spawn separately.
        // Here, in terms of timing, total time is not val1 + val2, but
        // its approximately max( val1, val2).
        // Each await is not blocking in the task. So it will run concurrently
        // in that sence.
        trpl::join(fut1, fut2).await;
    });
}

fn three() {
    trpl::run( async {
        // notice, this happens sequentially as will not go to the next
        // for block till this one gets complete.
        for x in 1..10 {
            println!("[First-three]:{}", x);
            trpl::sleep(std::time::Duration::from_millis(500)).await
        }
        
        for x in 1..5 {
            println!("[Second-three]:{}", x);
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }    
    });
}

fn three_a() {
    trpl::run( async {
        // notice, this happens sequentially as will not go to the next
        // for block till this one gets complete.
        for x in 1..10 {
            println!("[First-three]:{}", x);
            trpl::sleep(std::time::Duration::from_millis(500)).await
        }
        
        let fut = async {
            for x in 1..5 {
                println!("[Second-three]:{}", x);
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }    
        };

        let tsk = trpl::spawn_task( async {
            for x in 1..5 {
                println!("[Thrid-three]:{}", x);
                trpl::sleep(std::time::Duration::from_millis(500)).await;
            }    
        });
        
        // calling await here means that the task tsk will continue to run
        // till this await point along with fut. Here we will see that the
        // concurrency element between fut on main task and new task with tsk
        // running.
        fut.await;
    });
}

fn four() {
    trpl::run( async {
        // as we await it here, and since they are futures with async blocks
        // and not tasks, this will run sequentially.
        async {
            for x in 1..10 {
                println!("[First-four]:{}", x);
                trpl::sleep(std::time::Duration::from_millis(500)).await
            }
        }.await;
        async {
            for x in 1..5 {
                println!("[Secont-four]:{}", x);
                trpl::sleep(std::time::Duration::from_millis(500)).await
            }
        }.await;
    });
}
