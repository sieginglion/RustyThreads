fn p_map<A: Send + Sync + 'static, B: Send + 'static>(X: Vec<A>, f: fn(&A) -> B, workers: usize) -> Vec<B> {
    let X = std::sync::Arc::new(X);
    let mut handles = Vec::with_capacity(workers);
    for n in 0..workers {
        let X = X.clone();
        let handle = std::thread::spawn(move || {
            let mut Y_ = Vec::with_capacity(X.len() / workers + 1);
            for i in (n..X.len()).step_by(workers) {
                Y_.push(f(&X[i]));
            }
            return Y_;
        });
        handles.push(handle);
    }
    let mut R = Vec::with_capacity(workers);
    for handle in handles {
        R.push(handle.join().unwrap());
    }
    let mut Y = Vec::with_capacity(X.len());
    for i in 0..X.len() {
        Y.push(R[i % workers].pop().unwrap());
    }
    Y.reverse();
    return Y;
}