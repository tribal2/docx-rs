extern crate docx_core;

use docx_core::*;

pub const DUMMY: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

#[test]
pub fn indent() -> Result<(), DocxError> {
  let path = std::path::Path::new("./tests/output/indent.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new(DUMMY)).indent(840, None))
    .add_paragraph(Paragraph::new())
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new(DUMMY))
        .indent(840, Some(SpecialIndentType::FirstLine(720))),
    )
    .add_paragraph(Paragraph::new())
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new(DUMMY))
        .indent(1560, Some(SpecialIndentType::Hanging(720))),
    )
    .build()
    .pack(file)?;
  Ok(())
}

#[test]
pub fn size() -> Result<(), DocxError> {
  let path = std::path::Path::new("./tests/output/size.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new("Hello")).size(60))
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new(" Wor").size(50))
        .add_run(Run::new("ld")),
    )
    .build()
    .pack(file)?;
  Ok(())
}

#[test]
pub fn alignment() -> Result<(), DocxError> {
  let path = std::path::Path::new("./tests/output/alignment.docx");
  let file = std::fs::File::create(&path).unwrap();
  Docx::new()
    .add_paragraph(Paragraph::new().add_run(Run::new("Hello")))
    .add_paragraph(
      Paragraph::new()
        .add_run(Run::new(" World"))
        .align(AlignmentType::Right),
    )
    .build()
    .pack(file)?;
  Ok(())
}
