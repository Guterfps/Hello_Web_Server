
use std::{
    thread, 
    io, 
    sync::{mpsc::{self, Receiver}, Arc, Mutex},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
    /// The `build` function will return a PoolCreationError if the `size` is zero.
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
        
        Ok(ThreadPool { workers, sender })
    }
        
    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();   
    }
}
    
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, 
            receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, io::Error> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();    

            println!("Worker {id} got a job, executing..");

            job();
        })?;

        Ok(Worker { id, thread})
    }
}
