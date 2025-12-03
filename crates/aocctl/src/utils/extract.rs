//! Simple HTML extraction helpers used by the submission logic.

/// Strip angle bracket tags from a small HTML snippet.
///
/// This is not a general purpose HTML parser, but for the small snippets
/// returned by Advent of Code it is sufficient.
pub fn strip_tags(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }

    out
}

/// Extract the text content of the first `<article>...</article>` block.
///
/// Returns `None` if no such block is found.
pub fn extract_article_text(html: &str) -> Option<String> {
    let start = html.find("<article")?;
    let end_rel = html[start..].find("</article>")?;
    let slice = &html[start..start + end_rel];
    Some(strip_tags(slice))
}
