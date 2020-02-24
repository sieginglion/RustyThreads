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

```Rust
mod ParMap;

fn main() {
    let a = vec![0, 1, 2, 3];
    let b = ParMap::par_map(a, |x| x + 1, 2);
}
```