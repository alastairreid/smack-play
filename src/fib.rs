#[ macro_use ]
mod smack;
use smack ::*;

fn fib(x: usize, cache: &mut Vec<u64>) {
    for i in 2..x+1 as usize {
        cache[i] = cache[i-1] + cache[i-2];
    }
}

fn main () {
    let n = 5usize.nondet ();
    assume !(n > 2);

    let mut cache = vec![0; (n+1) as u64];
    cache [0] = 0;
    cache [1] = 1;
    fib(n, &mut cache);

    // this is always true
    assert!( cache[n] >= n as u64 );

    // this is not always true - using this to test that boogie can detect bugs!
    assert!( cache[n] < 0 as u64 );
}
