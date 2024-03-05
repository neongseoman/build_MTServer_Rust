use std::{io, thread};

// struct Worker {
//     id : usize,
//     thread : thread::JoinHandle<()>
// }

pub struct ThreadPool{
    threads : Vec<thread::JoinHandle<()>>
}

impl ThreadPool{
    pub fn new(size:usize) -> ThreadPool{
        assert!(size>0);
        // 패닉을 일으키는게 좋은 걸까? 에러 처리를 하는게 더 낫지 않을까?
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size{
            threads.push(thread::spawn(||{}))
            // 여기에는 thread spawn이 생기는데 이전에는 무슨 일을 할지 클로저로 념겨줬었다.
            // 하지만 클로저에 뭘 넣어야할까? 이후에 넘길 수는 없을까?
            // 왜나하면 request가 오면 thread가 그 request를 처리하게 만들고 싶기 때문이다.
        }

        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f:F)
    where
        F:FnOnce() + Send + 'static
    {

    }
}