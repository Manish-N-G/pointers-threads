// Async Select

pub fn async_select() {
    let rt = tokio::runtime::Runtime::new().expect("fail");
    rt.block_on( async {
        let fut1 = async {
            println!("start fut1");
            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
            println!("sleep 50 fut1, finieshed");
            for x in 0..1000000000{
                let y = (x+10000)*2 % 10;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
            println!("sleep 250 fut1, finieshed");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("sleep sec 10 fut1, finieshed");
        };

        let fut2 = async {
            println!("start fut2");
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
            println!("sleep 5 fut2, finieshed");
            tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
            println!("sleep 250 fut2, finieshed");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("sleep sec 10 fut2, finieshed");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("sleep sec 10 fut2, finieshed");
        };

        let v = tokio::select! {
            val1 = fut1 /* we send futures here */ => 8,
            val2 = fut2 /* we send futures here */ => 10,
        };

        println!("v is {v}");
    });
}
