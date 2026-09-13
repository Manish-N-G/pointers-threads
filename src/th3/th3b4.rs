use trpl::*;
use tokio::*;
use std::time::*;

#[derive(Clone, Copy)]
pub enum FutType {
    TrplJoin,
    TokioSelect,
    TokioJoin,
}

//NOTE: Here, at each await point, when a future is bring awaited, it checkes whether
//the futuer is ready or pending. If its ready, this means that we continue the statements
//without yielding. However, when we get a pending state, this would mean that the runtime
//with respect with tokio yeilds that future as sees that other futures could run. 
//When we combine this with join and select, considering we are talking about fut1 and fut2,
//fut1 processes till it get a pending state, and then will calls poll on fut2 to and processes
//fut2 till it finishes or get to another pending state. If we encounter another pending
//state, considering we are only talking about these 2 for the moment, it join or select
//will yield as a whole as all futures were polled and were at pending or complete. The
//runtime could revisit these join select types again if needed.

// here we will try to run select and trpl to see how we can use async to do something
pub fn multi_async() {
    let mut get_time = Instant::now();
    trpl::block_on(
        nested_async( FutType::TrplJoin )
    );
    println!( "1st time TrplJoin: {:?}", get_time.elapsed().as_millis()  );

    get_time = Instant::now();
    trpl::block_on(
        nested_async( FutType::TokioSelect )
    );
    println!( "2st time TokioSelect: {:?}", get_time.elapsed().as_millis()  );

    trpl::block_on(
        nested_async( FutType::TokioJoin )
    );
    println!( "1st time TokioJoin: {:?}", get_time.elapsed().as_millis()  );
}

// both work
async fn sleep_with_time_print(t: u32, stmt: &str) {
// fn sleep_with_time_print(t: u32, stmt: &str) -> impl Future<Output = ()> {
    // async move {
    println!("working with fut: Start fut{}, with time: {:?}", stmt, t);
    trpl::sleep( std::time::Duration::from_millis(t as u64) ).await;
    println!("working with fut: End fut{}, with time: {:?}", stmt, t);
    // }
}

async fn process_fut<A,B>( ftype: FutType, f1: A, f2: B )
where 
    A: Future,
    B: Future,
{
    match ftype {
        FutType::TrplJoin => { trpl::join!(f1, f2); },
        FutType::TokioJoin => { tokio::join!(f1, f2); },
        FutType::TokioSelect => {
            tokio::select!{
                _ = f1 => {}
                _ = f2 => {}
            }
        },
    };
}

async fn nested_async( ftype: FutType ) {

    let fut1 = async {
        let fut1_1 = sleep_with_time_print( 100, "1_1");
        fut1_1.await;
        
        let fut1_2 = async {
            sleep_with_time_print(20, "1_2").await;

            let fut1_2_1 = sleep_with_time_print(120, "1_2_1");
            let fut1_2_2 = sleep_with_time_print(30, "1_2_2");

            process_fut(ftype, fut1_2_1, fut1_2_2).await;

        };

        let fut1_3 = async {
            sleep_with_time_print( 20, "1_3").await;
        };

        process_fut(ftype, fut1_2, fut1_3).await;

        let fut1_4 = async {
            sleep_with_time_print( 40, "1_4").await;
        }.await;

    };

    let fut2 = async {
        sleep_with_time_print( 200, "2").await;
    }.await;

    let fut3 = async {
        let fut3_1 = sleep_with_time_print( 500, "3_1");
        
        let fut3_2 = async {

            let fut3_2_1 = sleep_with_time_print(320, "3_2_1");
            let fut3_2_2 = sleep_with_time_print(90, "3_2_2");

            sleep_with_time_print(20, "3_2").await;

            process_fut(ftype, fut3_2_1, fut3_2_2).await;
        }.await;

        let fut3_3 = async {
            sleep_with_time_print(20, "3_3").await;
        };

        process_fut(ftype, fut3_1, fut3_3).await;

        sleep_with_time_print( 1000, "3").await;
    };

    process_fut(ftype, fut1, fut3).await;
}


/* NOTE: didnt work. Need to implement traits to have both join and select.
The reason is beause the futures are "anonymous types". As we are asking to get
the concrete types for the arguements. Rust compiler will not be able to infer
this and hecne we decide to use traits

// here we will try to run select and trpl to see how we can use async to do something
fn multi_async<A,B>() 
where
    A: Future<Output = ()>,
    B: Future<Output = ()>,
{
    let mut get_time = Instant::now();
    trpl::block_on(
        nested_async( trpl_join::<A,B> )
    );
    println!( "1st time: {:?}", get_time.elapsed().as_millis()  );

    get_time = Instant::now();
    trpl::block_on(
        nested_async( trpl_join::<A,B> )
    );
    println!( "2st time: {:?}", get_time.elapsed().as_millis()  );
}

// both work
async fn sleep_with_time_print(t: u32, stmt: &str) {
// fn sleep_with_time_print(t: u32, stmt: &str) -> impl Future<Output = ()> {
    // async move {
    println!("working with fut: fut{}", stmt);
    trpl::sleep( std::time::Duration::from_millis(t as u64) ).await;
    // }
}

async fn trpl_join<A,B>( fut1: A, fut2: B) 
// fn trpl_join<A,B>( fut1: A, fut2: B) -> impl Future<Output = ()>
where 
    A: Future<Output = ()>,
    B: Future<Output = ()>,
{
    // async move {
    trpl::join!(fut1, fut2);
    // }
}

async fn tokio_select<A,B>( fut1: A, fut2: B) 
where 
    A: Future<Output = ()>,
    B: Future<Output = ()>,
{
    tokio::select!{
        _ = fut1 => {}
        _ = fut2 => {}
    }
}

async fn nested_async<A,B,C,F>( f: F ) 
// async fn nested_async<F>( f: F ) 
where 
    A: Future<Output = ()>,
    B: Future<Output = ()>,
    C: Future<Output = ()>,
    F: Fn( A, B ) -> C ,
{
    let fut1 = async {
        let fut1_1 = sleep_with_time_print( 100, "1_1");
        fut1_1.await;
        
        let fut1_2 = async {
            sleep_with_time_print(20, "1_2").await;

            let fut1_2_1 = sleep_with_time_print(120, "1_2_1");
            let fut1_2_2 = sleep_with_time_print(30, "1_2_2");

            // f(fut1_2_1, fut1_2_2).await;
            // f(fut1_2_1, fut1_2_2).await;
        };


        let fut1_3 = async {
            trpl::sleep( std::time::Duration::from_millis(20) );
        };

        trpl::join!(fut1_2, fut1_3);

        let fut1_4 = async {
            trpl::sleep( std::time::Duration::from_millis(20) );
        }.await;

    };

    let fut2 = async {
        trpl::sleep( std::time::Duration::from_millis(200) ).await
    };
}
*/



