/*
 * src/threadpool.rs
 * author: ian brault <ian.brault@engineering.ucla.edu>
 */

use std::sync::Arc;
use std::thread;

use crossbeam::channel::{Receiver, Sender, unbounded};

use crate::semaphore::Semaphore;

enum PoolMessage<T> {
    Compute(T),
    Terminate,
}

pub struct ThreadPool<T,R> {
    n_threads: u32,
    tx: Sender<PoolMessage<T>>,
    rx: Receiver<R>,
    sem: Arc<Semaphore>,
}

impl<T,R> ThreadPool<T,R> where T: Send + 'static, R: Send + 'static {
    pub fn new(n_threads: u32, f: Box<Fn(T) -> R + Send + Sync>) -> ThreadPool<T,R> {
        let (master_send, worker_recv) = unbounded();
        let (worker_send, master_recv) = unbounded();

        let sem = Arc::new(Semaphore::new(0));
        let func = Arc::new(f);

        for _ in 0..n_threads {
            let (tx, rx) = (worker_send.clone(), worker_recv.clone());
            let tsem = sem.clone();
            let tfunc = func.clone();

            thread::spawn(move || {
                loop {
                    // wait for a message to be sent
                    tsem.acquire();
                    // compute on the message and return the result
                    // or terminate workers when pool is dropped
                    match rx.recv().unwrap() {
                        PoolMessage::Compute(val) => tx.send(tfunc(val)).unwrap(),
                        PoolMessage::Terminate => break,
                    }
                }
            });
        }

        ThreadPool { n_threads, tx: master_send, rx: master_recv, sem }
    }

    pub fn send(&self, n: T) {
        self.tx.send(PoolMessage::Compute(n)).unwrap();
        self.sem.release();
    }

    pub fn recv(&self) -> R {
        self.rx.recv().unwrap()
    }
}

impl<T,R> Drop for ThreadPool<T,R> {
    fn drop(&mut self) {
        // terminate worker threads when pool is dropped
        for _ in 0..self.n_threads {
            self.tx.send(PoolMessage::Terminate).unwrap();
            self.sem.release();
        }
    }
}
