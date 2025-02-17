use std::{rc::Rc, sync::Mutex, time::Duration};

#[tokio::main]
async fn main() {
    let _ = garb().await;
    // garb2().await;
    garb3().await;
    // garb4().await;
    // tokio::spawn(garb2()).await;
}

async fn garb() {
    let x = vec!["hello"];
    let mut rx = Rc::new(x);

    tokio::time::sleep(Duration::from_millis(100)).await;
    // garb3().await;
    let test = Rc::get_mut(&mut rx).unwrap();
    test.push("world");
    println!("{:?}", test);
}

async fn garb2() {
    garb().await;
}

async fn garb3() {
    let _ = tokio::spawn(async move {
        let x = vec!["garbo"];
        let mx = Mutex::new(x);
        {
            let mut mxg = mx.lock().unwrap();
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    })
    .await;
}

async fn garb4() {
    let x = vec!["garbo"];
    let mx = Mutex::new(x);
    let mut mxg = mx.lock().unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    garb3().await;
    mxg.push("value");
}
