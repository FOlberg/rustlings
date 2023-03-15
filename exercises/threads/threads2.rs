// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value -> Taking the lock and giving it back after value change
            // let mut mutex = status_shared.lock().unwrap();
            // let mut mutex = status_shared.lock().unwrap();
            // (*mutex).jobs_completed += 1;

            (*status_shared.lock().unwrap()).jobs_completed += 1;
            // drop(mutex);
            // status_shared.lock().unwrap().jobs_completed = counter;
            // Mutex::unlock(mutex);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed );
    }
}
