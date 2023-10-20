// 078.rs
// https://www.runoob.com/rust/rust-concurrency.html

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/* result:
main thread print 0
spawned thread print 0
main thread print 1
spawned thread print 1
spawned thread print 2
main thread print 2
spawned thread print 3
*/
