use std::sync::{Arc, Mutex};

//
struct Inner<T> {
    queue: Mutex<Vec<T>>,
}

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(), // Create default value by its type.
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
