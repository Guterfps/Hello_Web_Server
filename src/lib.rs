
use std::{
    thread, 
    io, 
    sync::{mpsc::{self}, Arc, Mutex},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreads(&'static str),
    OsError(String),
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The `size` is the number of threads in the pool.
    /// 
    /// # Errors
    /// 
    /// The `build` function will return a PoolCreationError if the `size` is zero,
    /// or if the OS fails to create the threads.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
        return Err(PoolCreationError::
            ZeroThreads("Cant initialize ThreadPool with zero threads"));
        }
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            let worker = match Worker::new(id, Arc::clone(&receiver)) {
                Ok(w) => w,
                Err(e) => {
                    return Err(PoolCreationError::
                                OsError(format!("Os Error: {:#?}", e)))
                }
            };
            
            workers.push(worker);
        }
        
        Ok(ThreadPool { workers, sender: Some(sender) })
    }
    
    /// adds a job to the ThreadPool.
    /// 
    /// The `f` is the job to add.
    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();   
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shuting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
    
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) 
        -> Result<Worker, io::Error> {
        let builder = thread::Builder::new();
        let thread = Some(builder.spawn(move || loop {
            let msg = receiver.lock().unwrap().recv();    

            match msg {
                Ok(job) => {
                    println!("Worker {id} got a job, executing..");
        
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected, shuting down.");
                    break;
                }
            }
        })?);

        Ok(Worker { id, thread})
    }
}
