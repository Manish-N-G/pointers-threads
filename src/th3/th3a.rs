// we will try some async stuff in this to get a good idea how they work.
// This will be interestting,to find all the different querks that exists in
// the code.

pub async fn thread3a_normal_async() {
    make_coffee().await;
}

pub async fn thread3a_multi_async() {
    make_coffee_multi().await;
}

async fn make_coffee() {
    boil_water().await;
    grind_beans().await;
    brew_coffee().await;
}

async fn make_coffee_multi() {
    let f1 = boil_water();
    let f2 = grind_beans();
    tokio::join!(f1, f2);
    brew_coffee().await;
}

async fn boil_water() -> u8 {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("boiling the water for coffee");
    3
}

async fn grind_beans() -> u8 {
    println!("grinding the beans for coffee");
    8
}

async fn brew_coffee() -> u8 {
    println!("brewing the coffee with after water and coffee are done");
    44
}
