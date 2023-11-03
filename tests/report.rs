#![feature(error_generic_member_access)]

use expect_test::expect;
use thiserror::Error;
use thiserror_ext::AsReport;

#[derive(Error, Debug)]
#[error("inner error")]
struct Inner {}

#[derive(Error, Debug)]
#[error("middle error: {source}")] // Some error may include source message in its message
struct Middle {
    #[from]
    source: Inner,
    #[backtrace]
    backtrace: std::backtrace::Backtrace,
}

#[derive(Error, Debug)]
#[error("outer error")] // but the best practice is to not include
struct Outer {
    #[from]
    #[backtrace]
    source: Middle,
}

fn inner() -> Result<(), Inner> {
    Err(Inner {})
}

fn middle() -> Result<(), Middle> {
    inner()?;
    Ok(())
}

fn outer() -> Result<(), Outer> {
    middle()?;
    Ok(())
}

#[test]
fn test_report_display() {
    let expect = expect![[r#"
        outer error: middle error: inner error
    "#]];
    expect.assert_eq(&format!("{}", outer().unwrap_err().as_report()));
}

#[test]
fn test_report_display_alternate() {
    let expect = expect![[r#"
        outer error

        Caused by these errors (recent errors listed first):
          1: middle error*
          2: inner error
    "#]];
    expect.assert_eq(&format!("{:#}", outer().unwrap_err().as_report()));
}

#[test]
fn test_report_display_alternate_single_source() {
    let expect = expect![[r#"
        middle error*

        Caused by this error:
          1: inner error
    "#]];
    expect.assert_eq(&format!("{:#}", middle().unwrap_err().as_report()));
}

// Show that there's extra backtrace information compared to `Display`.
// Backtrace is intentionally disabled to make the test deterministic.
#[test]
fn test_report_debug() {
    let expect = expect![[r#"
        outer error: middle error: inner error

        Backtrace:
        disabled backtrace
    "#]];
    expect.assert_eq(&format!("{:?}", outer().unwrap_err().as_report()));
}

// Show that there's extra backtrace information compared to `Display`.
// Backtrace is intentionally disabled to make the test deterministic.
#[test]
fn test_report_debug_alternate() {
    let expect = expect![[r#"
        outer error

        Caused by these errors (recent errors listed first):
          1: middle error*
          2: inner error

        Backtrace:
        disabled backtrace
    "#]];
    expect.assert_eq(&format!("{:#?}", outer().unwrap_err().as_report()));
}
