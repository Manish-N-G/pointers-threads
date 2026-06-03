trait MyAs {
    fn get(&self, id: &str) 
        -> std::pin::Pin<Box<dyn std::future::Future< Output = Option<String>> + Send + Sync + '_ >>;
}


fn process(val: std::sync::Arc<dyn MyAs + Send + Sync >)
    -> tokio::task::JoinHandle<Option<String>>
{ 
    tokio::spawn( async move{ 
        val.get("testing").await
    })
}

#[derive(Debug)]
struct St<'a> {
    a: String,
    b: &'a str,
}

impl MyAs for St<'_> {
    fn get(&self, id: &str) 
        -> std::pin::Pin<Box<dyn std::future::Future< Output = Option<String>> + Send + Sync + '_ >>
    {
        let x = match (self, id) {
            (_, y) if y.len() > 7 => None,
            (x, y) if y.len() > x.a.len() => Some(y.to_owned()),
            (x, _) => Some(x.a.clone()),
        };
        Box::pin( async { x })
    }
}

pub fn thread3b_async_runner() {
    let x = St { a: "Yo".to_string(), b: "Mama"};
    let rt = tokio::runtime::Runtime::new().expect("runtime didnt work");
    let y = rt.block_on(
        x.get("random")
    );
    println!("y1 is {:?}", y);
    let y = rt.block_on(
        x.get("said")
    ); // this thread is awiated here
    println!("y2 is {:?}", y);
    let y = rt.block_on(
        x.get("hahhhahahah")
    ); // this thread is awiated here
    println!("y3 is {:?}", y);

    let x = std::sync::Arc::new(x);
    let z = rt.block_on( async{
        let x = process(x).await;
        println!("x is {:?}", x);
        x
    });
}
