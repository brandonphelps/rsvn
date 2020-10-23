

mod svn_source_funcs;

use std::fs::File;
use std::io::prelude::*;

trait Show {
    fn show(&self) -> String;
}

trait SVNSource {
    fn cat(&self, a: String) -> String;
    fn cat_r(&self, path: String, rev: u64) -> Result<String, &'static str>;
    fn cat_s(&self) -> &'static str;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

struct Duck();
struct Parrot();

impl Show for Duck {
    fn show(&self) -> String {
        format!("Duck quack")
    }
}

impl Show for Parrot {
    fn show(&self) -> String {
        format!("squak squak")
    }
}

struct MockSVNServer();
struct LocalFileSVNServer {
    // make this constant after initialization
    root_path: String
}

impl LocalFileSVNServer {
    fn new(root_path: &str) -> LocalFileSVNServer {
        LocalFileSVNServer {
            root_path: root_path.to_string()
        }
    }
}

impl SVNSource for MockSVNServer {
    // returns a new instance? 
    fn cat(&self, a: String ) -> String {
        let s = a + &" Hello World".to_string();
        s
    }
    fn cat_r(&self, path: String, rev: u64) -> Result<String, &'static str> {
        Err("Hello world")
    }

    // Static return
    fn cat_s(&self) -> &'static str {
        "Hello World"
    }
}

impl SVNSource for LocalFileSVNServer {
    // returns a new instance? 
    fn cat(&self, a: String ) -> String {
        "hello world".to_string()
    }

    fn cat_r(&self, path: String, rev: u64) -> Result<String, &'static str> {
        
        Ok("Hello world".to_string())
    }

    // Static return
    fn cat_s(&self) -> &'static str {
        "Hello World"
    }
}


fn main() {
    println!("Hello, world!");
    let answer = 42;
    let maybe_pi = 3.14;
    let q = Duck();
    let p = Parrot();
    svn_source_funcs::add_to_waitlist();
    println!("Answer: {}", answer.show());
    println!("Duck: {}", q.show());

    let ducks: Vec<&Duck> = vec![&q];

    for d in &ducks {
        println!("Duck: {}", d.show());
    }

    let m = MockSVNServer();
    let l = LocalFileSVNServer::new(r"C:\Users\Brandon\Desktop\rust");
    println!("cat: {}", m.cat("hello world".to_string()));
    println!("cat: {}", m.cat_s());

    let par =  m.cat_r("testing".to_string(), 1);


    match par {
        Ok(v) => println!("Working with blah {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    }
}
