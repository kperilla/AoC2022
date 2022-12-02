// use error_chain::error_chain;
// use std::io::Read;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

pub fn part1(){
    println!("Part 1");
}

pub fn part2(){
    println!("Part 2");
}


// fn main() -> Result<()> {
//     let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);

//     Ok(())
// }