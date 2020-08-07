use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

//
struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue); // Drop the guard to unlock the object before notifying other threads to do sth.
        self.inner.available.notify_one(); // Trigger the flag
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => queue = self.inner.available.wait(queue).unwrap(), // Wait until the available flag is triggered.
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(), // Create default value by its type.
        available: Condvar::default(),
    };
    let inner = Arc::new(inner);

    (
        Sender {
            inner: inner.clone(), // Sender and Receiver are communicated by the same object.
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
