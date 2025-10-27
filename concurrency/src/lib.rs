use std::ops::Add;
use std::thread;

pub fn _concurrent_add<T>(items: Vec<T>, num: T) -> Vec<thread::JoinHandle<T>>
where
    T: Send + 'static,
    T: Add<Output = T> + Copy,
{
    let mut handles = Vec::new();
    for item in items {
        let handle = thread::spawn(move || item + num);
        handles.push(handle);
    }
    handles
}

pub fn concurrent_add<T>(items: Vec<T>, num: T) -> Vec<thread::JoinHandle<T>>
where
    T: Add<Output = T> + Copy,
    T: Send + 'static,
{
    items
        .into_iter()
        .map(|item| thread::spawn(move || item + num))
        .collect()
}

pub fn main() {
    {
        let mut list = vec![1, 2, 3, 4, 5];

        let handles = concurrent_add(list.clone(), 3);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 3);
        }
    }

    {
        let mut list = vec![10., 20., 30., 40., 50.];

        let handles = concurrent_add(list.clone(), 5.);

        for handle in handles {
            let result = handle.join().unwrap();
            let original = list.remove(0);

            assert_eq!(result, original + 5.);
        }
    }
}
