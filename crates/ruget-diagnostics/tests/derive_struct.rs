use ruget_diagnostics::Diagnostic;
use ruget_diagnostics::DiagnosticCategory;
use ruget_diagnostics::Explain;
use thiserror::Error;

#[derive(Diagnostic, Debug, Eq, PartialEq, Error)]
#[error("Colored struct.")]
#[label("color::struct")]
#[advice("Color.")]
#[category(Misc)]
pub struct Color {
    input: Option<String>,
    field: i32,
}

impl Explain for Color {}

#[test]
fn it_works() {
    let clr = Color {
        field: 1,
        input: Some("lol".into()),
    };
    assert_eq!("color::struct", clr.label());
    assert_eq!("Color.", clr.advice().unwrap());
}
