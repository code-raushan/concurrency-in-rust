    use std::thread;

fn main() {
    let mut v: Vec<_> = Vec::new();
    for i in 1..10 {
        v.push(thread::spawn(move|| {
            println!("hi from {} thread!", i);
        }));
        println!("v: {:?}", i);
    }
    for handle in v {
        match handle.join() {
            Ok(_) => println!("thread finished"),
            Err(e) => println!("thread panicked: {:?}", e),
        }
    }
}
