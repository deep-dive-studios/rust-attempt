#![feature(test)]

extern crate test;

#[macro_use]
extern crate attempt;

use test::Bencher;

#[inline(never)]
fn gives_throw_ok() -> Result<&'static str, attempt::Error<&'static str>> {
    test::black_box(Ok("ok"))
}

#[inline(never)]
fn gives_ok() -> Result<&'static str, &'static str> {
    test::black_box(Ok("ok"))
}

#[inline(never)]
fn throws_up_ok() -> Result<&'static str, attempt::Error<&'static str>> {
    let ok_msg = test::black_box(pass!(gives_throw_ok()));
    Ok(ok_msg)
}

#[inline(never)]
fn throws_throw_ok() -> Result<&'static str, attempt::Error<&'static str>> {
    let ok_msg = test::black_box(attempt!(gives_ok()));
    Ok(ok_msg)
}

#[inline(never)]
fn throws_try_ok() -> Result<&'static str, &'static str> {
    let ok_msg = test::black_box(try!(gives_ok()));
    Ok(ok_msg)
}

#[bench]
fn bench_throw_ok_return(bench: &mut Bencher) {
    bench.iter(|| {
        throws_throw_ok()
    })
}

#[bench]
fn bench_up_ok_return(bench: &mut Bencher) {
    bench.iter(|| {
        throws_up_ok()
    })
}

#[bench]
fn bench_try_ok_return(bench: &mut Bencher) {
    bench.iter(|| {
        throws_try_ok()
    })
}
