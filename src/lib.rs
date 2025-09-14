use std::{
    sync::{mpsc,Arc,Mutex},
    thread
};

type Job=Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    sender:Option<mpsc::Sender<Job>>,
    workers:Vec<Worker>,
}

impl ThreadPool{
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(n:usize)->Self{
        assert!(n>0);
        let mut workers=Vec::with_capacity(n);
        let (sender,receiver)=mpsc::channel();
        let receiver=Arc::new(Mutex::new(receiver));
        for i in 0..n{
            workers.push(Worker::new(i,Arc::clone(&receiver)));
        }
        Self{
            workers,
            sender:Some(sender),
        }
    }

    pub fn execute<F>(&self,f:F)
    where
        F: FnOnce()+Send+'static,
    {
        let job=Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){

        drop(self.sender.take());
        println!("Shutiing down");

        for worker in &mut self.workers{
            println!("Shutting down {} worker",worker.id);
            if let Some(thread)=worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>)->Self{
        let thread=thread::spawn(move ||{
            loop{
                if let Ok(job)=receiver.lock().unwrap().recv(){
                    println!("Worker with {id} got a job execting it");
                    job();
                }
                else{
                    println!("Worker with id={id} dropped");
                    break;
                }
            };
        });
        Self{
            id,
            thread:Some(thread),
        }
    }
}