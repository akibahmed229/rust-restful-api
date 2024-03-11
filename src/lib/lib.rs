/*
This code creates a thread pool with a specified number of worker threads. 
Jobs can be sent to the thread pool for execution, and each job is executed by an available worker thread. 
This implementation ensures thread safety using mutexes and message passing for communication between threads.
*/

use std::{ thread, sync::mpsc }; // mpsc: multiple producer, single consumer
use std::sync::{ Arc, Mutex };

// This struct represents a thread pool, which contains a vector of workers and a sender for sending jobs to the workers using Message passing.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// Job is a type alias for a boxed closure that takes no arguments and returns nothing (FnOnce()) and can be sent between threads (Send) with a 'static lifetime.
type Job = Box<dyn FnOnce() + Send + 'static>;


// Message is an enum that represents the different types of messages that can be sent between threads.
enum Message {
   NewJob(Job), 
   Terminate,
}

impl ThreadPool {
   // create a new ThreadPool 
   // The size is the number of threads in the pool
   // # Panics
   // The `new` function will panic if the size is zero
  pub fn new(size: usize) -> ThreadPool {
    // assertion to ensure that the pool size is greater than zero
     assert!(size > 0);

     // Create a channel for communication between threads
     let (sender, reciver) = mpsc::channel();

     // Wrap the receiver in an Arc and a Mutex for thread safety
     let reciver = Arc::new(Mutex::new(reciver));

     // Initialize the vector of workers with the specified size
     let mut workers = Vec::with_capacity(size);

     // Create worker threads and store them in the vector
     for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&reciver)));
     }

     // Return the ThreadPool with the vector of workers and sender
     ThreadPool { workers, sender }
  }

  // Method to execute a job in the thread pool
  pub fn execute<F>(&self, f: F) 
  where 
       F: FnOnce() + Send + 'static
   {
      // Wrap the closure in a boxed Job and send it to a worker
      let job = Box::new(f); 
      self.sender.send(Message::NewJob(job)).unwrap(); // send the job to the worker
   }
}

// Implement the Drop trait for ThreadPool to ensure that all workers are terminated when the thread pool goes out of scope.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending therminate message to all worker."); 

        for _ in &self.workers  {
           self.sender.send(Message::Terminate).unwrap(); 
        }

        println!("Shutting down all workers.");

       for worker in &mut self.workers  {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
               thread.join().unwrap();
            }
       } 
    } 
}

// This struct represents a worker in the thread pool, identified by its id, and contains a thread::JoinHandle which represents the thread executing the worker's task.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

// Within the constructor (new()), a new thread is spawned where the worker continuously loops to receive and execute jobs from the channel.
impl Worker {
  fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
     // Spawn a new thread for the worker
     let thread = thread::spawn(move || loop {
           // Lock the receiver to receive a job
           let message = reciver.lock().unwrap().recv().unwrap();

           // Print a message indicating the worker is executing the job
           println!("Worker {} got a job; executing.", id);
            
          match message {
             Message::NewJob(job) => {
                println!("Worker {} got a job; executing.", id);
                job();
             }
             Message::Terminate => {
                println!("Worker {} was told to terminate.", id);
                break;
             }
          }
       }); 
     
     // Return the Worker with its id and thread handle
     Worker { id, thread: Some(thread) } 
  } 
}
