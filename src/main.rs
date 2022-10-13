extern crate reqwest;
// extern crate select;

//use select::document::Document;
//use select::predicate::Name;
// use winit::monitor::MonitorHandle; get resolution of machine somehow.
// use hyper::{Body, Request, Response, Server};
// use tokio::main;
// use std::io;
mod urk;

//#[tokio::main]
fn main() {
    let x_resolution = 3840;
    let y_resolution = 2160;
    let client = reqwest::blocking::Client::new();
    let url = format!("https://interfacelift.com/wallpaper/downloads/downloads/wide_16:9/{}x{}/", x_resolution, y_resolution);
    // let url = "https://vg.no";
    let interfacelift_startpage = client.get(&url)
     .header("USER_AGENT","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36").send().unwrap().text().unwrap();
    //println!("{}",interfaceliftStartpage); 
    let document = scraper::Html::parse_document(&interfacelift_startpage);
    let selector_jeder = scraper::Selector::parse("div.jeder").unwrap();
    let selector_b = scraper::Selector::parse("b.").unwrap();
    let one = document.select(&selector_jeder).map(|x| x.inner_html());
    let jeder = document.select(&selector_jeder).next().unwrap();
    for element in jeder.select(&selector_b) {
        println!("{}",element.value().name());
    }
//
    //println!("{}",url);
    //println!("{:#?}",jeder_b);
    // one.zip(1..101).for_each(|(item, number)|println!("{}. {}", number, item));
    //let wrapper_vec = WrapperVec {
    //    b: current_page_number,
    //};
    // print!("{}",current_page_number);
    // print!("{}",current_page_number);
    
}

// header! {(UserAgent, "User-Agent") => [String]}
// "https://interfacelift.com/wallpaper/downloads/downloads/wide_16:9/3840x2160/"
// fn main() {
// 
//     
//     // let request = client.get("https");
//     // headers.set(UserAgent("wallshifter/0.0.1".to_owned()));
//     // let response = client.get(
//     //     "https://www.vg.no"
//     // )
//     // .header("USER_AGENT","FOO")
//     // .send();
//    let client = reqwest::blocking::Client::new();
//    let response = client.get("https://vg.no",)
//     .header("USER_AGENT","Foo")
//     .send();
// 
//    let unwrapped_response = response.unwrap().text().unwrap();
//    let document = scraper::Html::parse_document(&unwrapped_response);
// 
//    let title_selector = scraper::Selector::parse("div").unwrap();
// 
//    let titles = document.select(&title_selector).map(|x| x.inner_html());
// 
//    println!("{:#?}",titles);
// 
//    titles
//       .zip(1..101)
//       .for_each(|(item, number)| println!("{}. {}", number, item));
//         
//    //urk::imdb();
// }
// 

    // let x = 5;
    // let x = x + 5;
    // let y: u32 = 0xff;
    // let asd ="asd";
    // let spaces: usize = asd.len();
    // let z: u8 = 0b1111_1111;
    // let c = 'z';
    // let d = 'ℤ';
    // let heart_eyed_cat = '😻';
    // let a = "asd";
    // let b = "❤";
    // let e = '🎂';
    // let f = '❤';
    
    
    
    
    
    

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {} {} {} {}", x, y, spaces,z);
    //     println!(" {} {} {} {} {} {} {} {}", a,b,c,d,e,f,heart_eyed_cat,z)
    // }

    // {
    //     println!("{:?}", getWallpaperSites());
    // }
    // println!("Hello, world! {}",x);

    //getInterfacelift("https://www.interfacelift.com")
    

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
//}

// fn getWallpaperSites() {
//     let sites = ["http://interfacelift.com"];
//     // println!("{}", sites);
// }
// fn getInterfacelift(url: &str) {
//     let mut resp = reqwest::get(url).unwrap();
//     assert!(resp.status().is_success());
//     Document::from_read(resp)
//         .unwrap()
//         .find(Name("a"))
//         .filter_map(|link| link.attr("href"))
//         .for_each(|ahrefTarget| println!("{}", ahrefTarget));
// }
// 
// fn getWallpaperSites() -> Vec<String> {
//     let sites = ["http://interfacelift.com"];
//     sites.iter().map(|x| x.to_string()).collect()
// }