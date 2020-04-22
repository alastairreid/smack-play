pub fn cache_fib(n: usize) -> Vec<u64> {
    let mut cache = vec![0; (n+1) as usize];
    cache [0] = 0;
    cache [1] = 1;
    fib(n, &mut cache);

    // this is always true
    assert!( cache[n] >= n as u64 );
    return cache;
}

fn fib(x: usize, cache: &mut Vec<u64>) {
    for i in 2..x+1 as usize {
        cache[i] = cache[i-1] + cache[i-2];
    }
}
