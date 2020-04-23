#[cfg(verifier="smack")]
#[ macro_use ]
#[cfg(verifier="smack")]
mod smack;
#[cfg(verifier="smack")]
use smack::*;

pub fn cache_fib(n: usize) -> Vec<u64> {
    let mut cache = vec![0; (n+1) as u64];
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


#[cfg(verifier="smack")]
fn main () {
    let n = 5usize.nondet ();
    assume!(n > 2);

    let cache = cache_fib(n);

    // this is not always true - using this to test that boogie can detect bugs!
    assert!( cache[n] <= n as u64 );
}

#[cfg(not(verifier="smack"))]
fn main() {
    let limit: usize = 10;
    let cache = cache_fib(limit);
    for i in 0..limit {
        println!("fib({}) = {}", i, cache[i]);
    }
}
