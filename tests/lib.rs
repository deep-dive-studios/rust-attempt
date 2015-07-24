#[macro_use]
extern crate attempt;


fn throw_static_message() -> Result<(), attempt::Error<&'static str>> {
    pass_new!("hi");
}

fn throw1() -> Result<(), attempt::Error<()>> {
    pass_new!(());
}

fn throw2() -> Result<(), attempt::Error<()>> {
    pass!(throw1());
    Ok(())
}

fn throw3() -> Result<(), attempt::Error<()>> {
    pass!(throw2());
    Ok(())
}

fn gives_ok() -> Result<&'static str, attempt::Error<&'static str>> {
    Ok("ok")
}

fn throws_ok() -> Result<&'static str, attempt::Error<&'static str>> {
    let ok_msg = pass!(gives_ok());
    Ok(ok_msg)
}

mod mod_test {
    use attempt;

    pub fn throws() -> Result<(), attempt::Error<&'static str>> {
        pass_new!("ahhhh");
    }
}

fn throws_into() -> Result<(), attempt::Error<String>> {
    attempt!(Err("some static string"));
    Ok(())
}

#[test]
fn test_static_message() {
    let error = throw_static_message().unwrap_err();
    assert_eq!(*error.original_error(), "hi");
    assert_eq!(error.to_string(), "Error: hi\n\tat 6:4 in lib (tests/lib.rs)");
}

#[test]
fn test_multiple_throws() {
    let error = throw3().unwrap_err();
    assert_eq!(error.original_error(), &());
    assert_eq!(format!("{:?}", error), "Error: ()\
    \n\tat 19:4 in lib (tests/lib.rs)\
    \n\tat 14:4 in lib (tests/lib.rs)\
    \n\tat 10:4 in lib (tests/lib.rs)");
}

#[test]
fn test_returns_ok() {
    let ok = throws_ok().unwrap();
    assert_eq!(ok, "ok");
}

#[test]
fn test_mod_throw() {
    let error = mod_test::throws().unwrap_err();
    assert_eq!(error.to_string(), "Error: ahhhh\
    \n\tat 36:8 in lib::mod_test (tests/lib.rs)");
}

#[test]
fn test_throws_into() {
    let error = throws_into().unwrap_err();
    assert_eq!(error.to_string(), "Error: some static string\
    \n\tat 41:4 in lib (tests/lib.rs)")
}
