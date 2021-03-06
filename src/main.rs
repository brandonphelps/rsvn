

mod svn_source_funcs;

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
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

    fn construct_path(path: String, rev: u64) -> PathBuf {
        PathBuf::new()
    }

    fn cat_k(&self, path: String, rev: u64) -> Result<String, String> {
        let p = PathBuf::from(path);
        // have to clone because String doesn't implement copy so rust would try to move it.
        // so make full new string and move that in. 

        let mut contents = String::new();

        let mut k = PathBuf::from(self.root_path.clone());
        k.push(rev.to_string());
        k.push(p);


        println!("Looking at: {}", k.display());
        let r_file = File::open(k);
        let mut file = match r_file {
            Ok(k)  => {
                println!("Succesfully opened file fadfaf");
                k
            }
            Err(_) => return Err("FileRead error: ".to_string()),
        };

        println!("Looking at file");
        match file.read_to_string(&mut contents) {
            Ok(j) => println!("Successfully read the file"),
            Err(_) => {
                println!("Failed to read from file");
                return Err("Failed to read from file".to_string())
            }
        }

        Ok(contents)
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
        let p = PathBuf::from(path);
        // have to clone because String doesn't implement copy so rust would try to move it.
        // so make full new string and move that in. 

        let mut contents = String::new();

        let mut k = PathBuf::from(self.root_path.clone());
        k.push(p);
        k.push(rev.to_string());

        println!("Looking at: {}", k.display());
        let r_file = File::open(k);
        let mut file = match r_file {
            Ok(k)  => {
                println!("Succesfully opened file fadfaf");
                k
            }
            Err(_) => return Err("FileRead error: "),
        };

        println!("Looking at file");
        match file.read_to_string(&mut contents) {
            Ok(j) => println!("Successfully read the file"),
            Err(_) => {
                println!("Failed to read from file");
                return Err("Failed to read from file")
            }
        }

        Ok(contents)
    }

    // Static return
    fn cat_s(&self) -> &'static str {
        "Hello World"
    }

}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // pulls in names from outer
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/resources/dev_repo");

        assert!(d.exists());
    }

    fn test_svn_cat() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/resources/dev_repo");
        let s_path = match d.to_str() {
            Some(T) => T,
            // want to have a assert false here. 
            None => "",
        };
        // this seems stupid
        assert_ne!(s_path, "");
        let local_svn = LocalFileSVNServer::new(s_path);
        let rest = match local_svn.cat_k("hello/world.txt".to_string(), 1) {
            Ok(k) => k,
            Err(_) => "".to_string(),
        };
        assert_eq!(rest, "hello world");
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
    let l = LocalFileSVNServer::new(r"C:\Users\Brandon\Desktop\rust\testing");
    println!("cat: {}", m.cat("hello world".to_string()));
    println!("cat: {}", m.cat_s());

    println!("Hell owrold rfjeklqjreklqjr");

    match l.cat_k("hello.txt".to_string(), 1) {
        Ok(f) => println!("{}", f),
        Err(e) => println!("Reading file error: {}", e),
    }

    // match l.root_path.into_os_string() {
    //     Ok(k) => println!(k.into_string()),
    //     Err(_) => println!("Error printing string fkcsl"),
    // }
    // println!("ROOT PATH: {}", l.root_path.into_os_string().into_string());

    // match par {
    //     Ok(v) => println!("Working with blah {:?}", v),
    //     Err(e) => println!("Error: {:?}", e),
    // }
}
