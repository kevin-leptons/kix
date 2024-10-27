use std::thread::{self};

use kix::{Error, Result};

#[test]
fn to_std_error_box_by_question_mark() {
    assert!(to_std_error_box_fn().is_err());
}

#[test]
fn from_std_error_by_question_mark() {
    assert!(from_std_error_fn().is_err());
}

#[test]
fn as_std_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::AddrInUse, "An error happened");
    let error = Error::from(io_error);
    use_as_std_error_fn(error.as_std_error());
}

#[test]
fn from_str() {
    Error::new("An error happened");
}

#[test]
fn send_result_between_threads() {
    let handle = thread::spawn(|| kix::Result::Ok("foo"));
    let thread_result = match handle.join() {
        Ok(v) => v,
        Err(e) => panic!("Thread panic {:?}", e),
    };
    match thread_result {
        Ok(v) => assert_eq!(v, "foo"),
        Err(e) => panic!("Thread retur error {e}"),
    }
}

fn to_std_error_box_fn() -> std::result::Result<(), Box<dyn std::error::Error>> {
    Err(make_kix_error())?
}

fn from_std_error_fn() -> Result<()> {
    std::fs::File::open("/tmp/not_existed_file")?;
    Ok(())
}

fn make_kix_error() -> Error {
    let io_error = std::io::Error::new(std::io::ErrorKind::AddrInUse, "An error happened");
    Error::from(io_error)
}

fn use_as_std_error_fn(_: impl std::error::Error) {}
