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

pub enum Message {
  NewJob(Job),
  Terminate,
}

pub struct Worker {
  pub id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread: thread::JoinHandle<()> = thread::spawn(move || {
      loop {
        let msg = receiver.lock().unwrap().recv().unwrap();

        println!("Worker {} got a job; executing.", id);

        match msg {
          Message::NewJob(job) => {
            println!("Worker {} got a job; executing.", id);

            job.call_box();
          },
          Message::Terminate => {
            println!("Worker {} was told to terminate.", id);

            break;
          },
        }
      }
    });

    Worker { id, thread: Some(thread) }
  }
}

use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
  team: Vec<Worker>,
  sender: mpsc::Sender<Message>,
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

    let mut team = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for i in 0..size {
      team.push(Worker::new(i, Arc::clone(&receiver)))
    }

    ThreadPool{ team, sender }
  }

  pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.team {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    for worker in &mut self.team {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}