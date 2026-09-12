use trpl::*;
use tokio::*;
// here we will try to run select and trpl to see how we can use async to do something
fn multi_async<A,B>() 
where
    A: Future<Output = ()>,
    B: Future<Output = ()>,
{
    trpl::block_on(
        nested_async( trpl_join::<A,B> )
    );
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
    A: Future,
    B: Future,
{
    // tokio::select!(fut1, fut2).await;
}

async fn nested_async<A,B,C,F>( f: F ) 
where 
    A: Future,
    B: Future,
    C: Future,
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

// f1 --
//   f1_1 --
//   await
//   f1_2
//   f1_3
//   join()


