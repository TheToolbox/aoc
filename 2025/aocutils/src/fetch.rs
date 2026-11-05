//! Helpers for the `aoc fetch` command: finding the session cookie and
//! pulling example inputs out of a puzzle page.
//!
//! The parsing lives here, outside the binary, so it can be unit tested
//! without network access.

use std::env;
use std::fs;
use std::path::PathBuf;

/// Read the adventofcode.com session cookie.
///
/// Looks, in order, at the `AOC_SESSION` environment variable,
/// `~/.config/aoc/session`, and `.aoc-session` in the current directory.
/// The value is the `session` cookie from a logged-in browser.
pub fn find_session() -> Option<String> {
    if let Ok(s) = env::var("AOC_SESSION") {
        let s = s.trim().to_string();
        if !s.is_empty() {
            return Some(s);
        }
    }
    let mut candidates = Vec::new();
    if let Some(home) = env::var_os("HOME") {
        candidates.push(PathBuf::from(home).join(".config/aoc/session"));
    }
    candidates.push(PathBuf::from(".aoc-session"));
    for path in candidates {
        if let Ok(s) = fs::read_to_string(&path) {
            let s = s.trim().to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

/// Split a puzzle page into its part descriptions (`<article class="day-desc">`).
/// Part 2's article only appears once part 1 is solved.
pub fn split_parts(html: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find("<article class=\"day-desc\"") {
        let after = &rest[start..];
        let end = after.find("</article>").map(|e| e + "</article>".len()).unwrap_or(after.len());
        parts.push(&after[..end]);
        rest = &after[end..];
    }
    parts
}

/// Find the example input in one part's description.
///
/// Picks the first `<pre><code>` block whose paragraph just before it
/// mentions "example" (puzzles almost always say "For example:"), falling
/// back to the first `<pre><code>` block at all.
pub fn extract_example(part_html: &str) -> Option<String> {
    let mut blocks = Vec::new();
    let mut rest = part_html;
    while let Some(start) = rest.find("<pre><code>") {
        let before = &rest[..start];
        let last_paragraph = before.rfind("<p>").map_or(before, |i| &before[i..]);
        let body_start = start + "<pre><code>".len();
        let Some(len) = rest[body_start..].find("</code></pre>") else { break };
        let body = &rest[body_start..body_start + len];
        blocks.push((last_paragraph.to_lowercase().contains("example"), html_to_text(body)));
        rest = &rest[body_start + len + "</code></pre>".len()..];
    }
    blocks
        .iter()
        .find(|(after_example, _)| *after_example)
        .or_else(|| blocks.first())
        .map(|(_, text)| text.clone())
}

/// Strip tags (like `<em>`) and decode the HTML entities AoC uses.
pub fn html_to_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&amp;", "&")
}

/// Work out the puzzle year from a folder name such as `2025`.
pub fn year_from_dir_name(name: &str) -> Option<u32> {
    let y: u32 = name.parse().ok()?;
    (2015..=2100).contains(&y).then_some(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"<main>
<article class="day-desc"><h2>--- Day 1: Test ---</h2>
<p>For example, the elves like lists.</p><p>Here is a list:</p><pre><code>not the example</code></pre>
<p>For example:</p>
<pre><code>L68
L30
R&lt;<em>48</em>&gt;
</code></pre>
<p>The answer is <code><em>3</em></code>.</p>
</article>
<p>Your puzzle answer was <code>1</code>.</p>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2>
<p>Now try this example:</p>
<pre><code>a &amp; b
</code></pre>
</article>
</main>"#;

    #[test]
    fn finds_both_parts() {
        assert_eq!(split_parts(PAGE).len(), 2);
    }

    #[test]
    fn picks_block_after_for_example() {
        let parts = split_parts(PAGE);
        assert_eq!(extract_example(parts[0]).unwrap(), "L68\nL30\nR<48>\n");
        assert_eq!(extract_example(parts[1]).unwrap(), "a & b\n");
    }

    #[test]
    fn falls_back_to_first_block() {
        let html = "<article class=\"day-desc\"><pre><code>1 2 3\n</code></pre></article>";
        assert_eq!(extract_example(html).unwrap(), "1 2 3\n");
    }

    #[test]
    fn no_blocks_means_no_example() {
        assert_eq!(extract_example("<article class=\"day-desc\"><p>hi</p></article>"), None);
    }

    #[test]
    fn year_from_folder() {
        assert_eq!(year_from_dir_name("2025"), Some(2025));
        assert_eq!(year_from_dir_name("aoc"), None);
        assert_eq!(year_from_dir_name("1999"), None);
    }
}
