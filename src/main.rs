use hyper::{Body, Request, Response, Server};
// use tokio::main;
// use std::io;


//#[tokio::main]
fn main() {
    let x = 5;
    let x = x + 1;
    let y: u32 = 0xff;
    let asd ="asd";
    let spaces: usize = asd.len();
    let z: u8 = 0b1111_1111;
    let c = 'z';
    let d = 'ℤ';
    let heart_eyed_cat = '😻';
    let a = "asd";
    let b = "❤";
    let e = '🎂';
    let f = '❤';

    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {} {} {} {}", x, y, spaces,z);
        println!(" {} {} {} {} {} {} {} {}", a,b,c,d,e,f,heart_eyed_cat,z)
    }

    {
        println!("{:?}", getWallpaperSites());
    }
    println!("Hello, world! {}",x);


    // Still inside `async fn main`...
   // let client = Client::new();
//
   // // Parse an `http::Uri`...
   // let uri = "http://httpbin.org/ip".parse().unwrap();
//
   // // Await the response...
   // let resp = client.get(uri).awaiting();
//
   // println!("Response: {}", resp.status());
}

// fn getWallpaperSites() {
//     let sites = ["http://interfacelift.com"];
//     // println!("{}", sites);
// }


fn getWallpaperSites() -> Vec<String> {
    let sites = ["http://interfacelift.com"];
    sites.iter().map(|x| x.to_string()).collect()
}