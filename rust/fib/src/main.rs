mod fib;

#[cfg(verifier="smack")]
#[ macro_use ]
#[cfg(verifier="smack")]
mod smack;
#[cfg(verifier="smack")]
use smack::*;

#[cfg(verifier="smack")]
fn main () {
    let n = 5usize.nondet ();
    assume!(n > 2);

    let cache = fib::cache_fib(n);

    // this is not always true - using this to test that boogie can detect bugs!
    assert!( cache[n] <= n as u64 );
}

#[cfg(not(verifier="smack"))]
fn main() {
    let limit: usize = 10;
    let cache = fib::cache_fib(limit);
    for i in 0..limit {
        println!("fib({}) = {}", i, cache[i]);
    }
}
