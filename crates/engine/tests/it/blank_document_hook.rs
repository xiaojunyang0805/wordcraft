//! An app built on WordCraft adjusts new blank documents for its region (its Normal defaults).
//! The hook is process-wide, so no other test here makes a blank document.

use wordcraft_engine::{Region, blank_document, set_blank_document_hook};

fn chinese_defaults(region: &Region, d: &mut wordcraft_doc::Document) {
    if region.language == "zh" {
        d.styles.default_chr.font_ea = Some("Hook EA Font".into());
        d.styles.default_chr.size = Some(10.5);
    }
}

#[test]
fn the_hook_adjusts_blank_documents_after_the_paper_is_set() {
    assert!(set_blank_document_hook(chinese_defaults));
    assert!(!set_blank_document_hook(chinese_defaults), "only the first hook counts");
    let zh = blank_document(&Region::from_tag("zh-CN"));
    assert_eq!(zh.styles.default_chr.font_ea.as_deref(), Some("Hook EA Font"));
    assert_eq!(zh.styles.default_chr.size, Some(10.5));
    // The region's paper is set first and kept: A4 in China.
    assert!((zh.last_section.page_w - 595.3).abs() < 1.0, "{}", zh.last_section.page_w);
    // The hook decides what changes: a US blank document stays WordCraft's.
    let us = blank_document(&Region::from_tag("en-US"));
    assert_eq!(us.styles.default_chr.font_ea, None);
    assert_eq!(us.styles.default_chr.size, Some(12.0));
}
