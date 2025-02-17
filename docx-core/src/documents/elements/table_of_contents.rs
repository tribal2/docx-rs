use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
// This struct is only used by writers
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContents {
    pub instr: InstrToC,
    pub items: Vec<TableOfContentsItem>,
    pub auto: bool,
    pub dirty: bool,
    pub alias: Option<String>,
    pub page_ref_placeholder: Option<String>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.instr = self.instr.heading_styles_range(start, end);
        self
    }

    pub fn add_style_with_level(mut self, s: StyleWithLevel) -> Self {
        self.instr = self.instr.add_style_with_level(s);
        self
    }

    pub fn hyperlink(mut self) -> Self {
        self.instr = self.instr.hyperlink();
        self
    }

    pub fn alias(mut self, a: impl Into<String>) -> Self {
        self.alias = Some(a.into());
        self
    }

    // pub fn tc_field_level_range(mut self, start: usize, end: usize) -> Self {
    //     self.instr = self.instr.tc_field_level_range(start, end);
    //     self
    // }

    pub fn add_item(mut self, t: TableOfContentsItem) -> Self {
        self.items.push(t);
        self
    }

    pub fn auto(mut self) -> Self {
        self.auto = true;
        self
    }

    pub fn dirty(mut self) -> Self {
        self.dirty = true;
        self
    }
}

impl BuildXML for TableOfContents {
    fn build(&self) -> Vec<u8> {
        let mut p = StructuredDataTagProperty::new();
        if let Some(ref alias) = self.alias {
            p = p.alias(alias);
        }
        if self.items.is_empty() {
            let p1 = Paragraph::new().add_run(
                Run::new()
                    .add_field_char(FieldCharType::Begin, true)
                    .add_instr_text(InstrText::TOC(self.instr.clone()))
                    .add_field_char(FieldCharType::Separate, false),
            );
            let p2 = Paragraph::new().add_run(Run::new().add_field_char(FieldCharType::End, false));

            XMLBuilder::new()
                .open_structured_tag()
                .add_child(&p)
                .open_structured_tag_content()
                .add_child(&p1)
                .add_child(&p2)
                .close()
                .close()
                .build()
        } else {
            let items: Vec<TableOfContentsItem> = self
                .items
                .iter()
                .map(|item| {
                    let mut item = item.clone();
                    item.instr = self.instr.clone();
                    item.dirty = self.dirty;
                    if item.page_ref.is_none() {
                        item.page_ref = self.page_ref_placeholder.clone();
                    }
                    item
                })
                .collect();
            XMLBuilder::new()
                .open_structured_tag()
                .add_child(&p)
                .open_structured_tag_content()
                .add_child(&items)
                .close()
                .close()
                .build()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_toc() {
        let b = TableOfContents::new().heading_styles_range(1, 3).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdt><w:sdtPr><w:rPr /></w:sdtPr><w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="true" /><w:instrText>TOC \o &quot;1-3&quot;</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /></w:r></w:p><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
</w:sdt>"#
        );
    }

    /*
        #[test]
        fn test_toc_with_items() {
            let b = TableOfContents::new()
                .heading_styles_range(1, 3)
                .add_items(Paragraph::new().add_run(Run::new().add_text("Hello")))
                .build();
            assert_eq!(
                str::from_utf8(&b).unwrap(),
                r#"<w:sdt>
      <w:sdtPr />
      <w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="false" /><w:instrText>TOC \o &quot;1-3&quot;</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /></w:r><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r><w:r><w:rPr /><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
    </w:sdt>"#
            );
        }
        */
}
