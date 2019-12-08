use std::thread;
use std::sync::mpsc;

pub trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct Worker {
  pub id: usize,
  _t: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let _t = thread::spawn(move || {
      loop {
        let job = receiver.lock().unwrap().recv().unwrap();

        println!("Worker {} got a job; executing.", id);

        job.call_box();
      }
    });

    Worker { id, _t }
  }
}

use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
  _team: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut _team = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for i in 0..size {
      _team.push(Worker::new(i, Arc::clone(&receiver)))
    }

    ThreadPool{ _team, sender }
  }

  pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }

}