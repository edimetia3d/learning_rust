use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::{thread, io};
use std::io::Write;

enum JobMsg {
    JOB(Box<dyn FnOnce() + Send + 'static>),
    TERMINATE,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<JobMsg>>>) -> Worker {
        let thread = thread::spawn(move ||
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();


                match msg {
                    JobMsg::JOB(job) => {
                        println!("Worker {} got a job.", id);
                        job()
                    }
                    JobMsg::TERMINATE => {
                        println!("Worker {} got a terminate msg", id);
                        return;
                    }
                }
            });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<JobMsg>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(JobMsg::JOB(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.iter_mut() {
            println!("Sending Quit Msg to worker {}", worker.id);

            self.sender.send(JobMsg::TERMINATE).unwrap();
        }

        for worker in self.workers.drain(..) {
            worker.thread.join().unwrap();
            println!("Worker {} stopped", worker.id);
        }
        io::stdout().flush().unwrap();
    }
}
