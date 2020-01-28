# RustyThreads
Concise thread pool for Rust

```Rust
mod threadpool;

fn main() {
    let pool = threadpool::ThreadPool::new(4);
    pool.assign(|| {
        return true;
    });
    let results = pool.join();
    assert_eq!(results, vec![true]);
}
```