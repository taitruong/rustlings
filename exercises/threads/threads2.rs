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

    // solution 1 using locked status in a while do loop
    // using lock in loop and implicitly unlocking it allows next sub thread taking control of status.
    // for i in 0..10 {
    //     let status_shared = status.clone();
    //     thread::spawn(move || {
    //         // use i so that each job starts 250ms later...
    //         thread::sleep(Duration::from_millis(i * 250 + 250));
    //         // TODO: You must take an action before you update a shared value
    //         status_shared.lock().unwrap().jobs_completed += 1;
    //     });
    // }
    // while status.lock().unwrap().jobs_completed < 10 {
    //     println!("Waiting for job {}", status.to_owned().lock().unwrap().jobs_completed);
    //     thread::sleep(Duration::from_millis(250));
    // }
    // println!("Jobs completed: {}", status.to_owned().lock().unwrap().jobs_completed);

    // solution 2 using handles
    let mut handles = vec![];
    for i in 0..10 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 250 + 250));
            // TODO: You must take an action before you update a shared value
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
