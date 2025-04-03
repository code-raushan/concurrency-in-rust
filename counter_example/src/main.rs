use std::thread;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            let mut counter = 0;
            for _ in 0..1_000_000 {
                counter += 1;
            }
            counter
        }));
    }

    let final_counter = handles.into_iter().map(|h| h.join().unwrap()).sum::<i32>();
    println!("Final counter: {}", final_counter);
}
