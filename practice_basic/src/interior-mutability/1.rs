use std::sync::RwLock;

fn main() {
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");
    drop(read1);
    // drop(read2);
    // let mut write1 = my_rwlock.write().unwrap();
    // *write1 = 6;
    // drop(write1);
    // println!("{:?}", my_rwlock);
    if let Ok(mut write1) = my_rwlock.try_write() {
        *write1 += 6;
        println!("{write1:?}");
        drop(write1);
        // println!("{write1:?}");
    } else {
        println!("Couldn't get write access")
    };
}
