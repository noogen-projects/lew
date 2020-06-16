use std::ops::{Add, AddAssign, Range, Sub, SubAssign};

use derive_more::{Add, AddAssign, Sub, SubAssign};
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::{html, utils, Html};

use crate::Widget;

pub mod tool;

pub struct SimpleToolbar {
    pub id: String,
    pub class: String,
    pub tools: Vec<Box<dyn Widget>>,
}

impl Default for SimpleToolbar {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            class: "lew-simple__toolbar".to_string(),
            tools: vec![
                Box::new(tool::Header::new()),
                Box::new(tool::Bold::new()),
                Box::new(tool::Italic::new()),
                Box::new(tool::Quote::new()),
            ],
        }
    }
}

impl SimpleToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}

impl Widget for SimpleToolbar {
    fn build(&self) -> Html {
        let item_class = format!("{}_item", self.class);
        html! {
            <ul id = &self.id class = &self.class>
                {
                    self.tools
                        .iter()
                        .map(|tool| html! { <li class = &item_class>{ tool.build() }</li> })
                        .collect::<Html>()
                }
            </ul>
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Add, Sub, AddAssign, SubAssign)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl From<Range<usize>> for Selection {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Add<usize> for Selection {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<usize> for Selection {
    fn add_assign(&mut self, rhs: usize) {
        self.start += rhs;
        self.end += rhs;
    }
}

impl Sub<usize> for Selection {
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<usize> for Selection {
    fn sub_assign(&mut self, rhs: usize) {
        self.start -= rhs;
        self.end -= rhs;
    }
}

pub fn replace_selected_in_textarea(
    textarea_selector: impl AsRef<str>, fmt: impl Into<ReplaceFmt>, mode: UnselectedApplyMode,
) {
    if let Ok(Some(element)) = utils::document().query_selector(textarea_selector.as_ref()) {
        if let Ok(textarea) = element.dyn_into::<HtmlTextAreaElement>() {
            let text = textarea.value();
            let start_char = textarea
                .selection_start()
                .map(|start| start.unwrap_or(0) as usize)
                .unwrap_or(0);
            let mut end_char = textarea
                .selection_end()
                .map(|end| end.unwrap_or(0) as usize)
                .unwrap_or(0);
            if end_char < start_char {
                end_char = start_char;
            }

            let (text, selection) = fmt.into().layout(text, start_char..end_char, mode);

            textarea.set_value(&text);
            textarea.focus().ok();
            textarea.set_selection_start(Some(selection.start as u32)).ok();
            textarea.set_selection_end(Some(selection.end as u32)).ok();
        }
    }
}

pub enum ReplaceFmt {
    Around(String, String),
    StartLine(String),
}

impl ReplaceFmt {
    pub fn layout(
        &self, text: String, selection: impl Into<Selection>, mode: UnselectedApplyMode,
    ) -> (String, Selection) {
        let text: Vec<_> = text.chars().collect();
        let selection = selection.into();
        let replacement = if selection.is_empty() {
            Selection {
                start: text[0..selection.start]
                    .iter()
                    .rposition(|&ch| mode.is_start_boundary(ch))
                    .map(|pos| pos + 1)
                    .unwrap_or(0),
                end: text[selection.end..]
                    .iter()
                    .position(|&ch| mode.is_end_boundary(ch))
                    .map(|pos| selection.start + pos)
                    .unwrap_or_else(|| text.len()),
            }
        } else {
            selection
        };

        match self {
            ReplaceFmt::Around(prefix, suffix) => Self::around_layout(text, prefix, suffix, selection, replacement),
            ReplaceFmt::StartLine(prefix) => Self::start_line_layout(text, prefix, selection, replacement),
        }
    }

    fn around_layout(
        text: Vec<char>, prefix: &str, suffix: &str, selection: Selection, replacement: Selection,
    ) -> (String, Selection) {
        let before: String = text[..replacement.start].iter().collect();
        let source: String = text[replacement.start..replacement.end].iter().collect();
        let after: String = text[replacement.end..].iter().collect();

        let prefix_chars = prefix.chars().count();
        let result_text;
        let result_selection;

        if before.ends_with(prefix) && after.starts_with(suffix) {
            result_text = before.trim_end_matches(prefix).to_string() + &source + after.trim_start_matches(suffix);
            result_selection = selection - prefix_chars;
        } else if source.starts_with(prefix) && source.ends_with(suffix) && source.len() >= prefix.len() + suffix.len()
        {
            let before_chars = before.chars().count();
            result_text = before + source.trim_start_matches(prefix).trim_end_matches(suffix) + &after;
            result_selection = if selection.is_empty() {
                let pos = before_chars.max(selection.start - prefix_chars);
                Selection { start: pos, end: pos }
            } else {
                Selection {
                    start: selection.start,
                    end: selection.end - prefix_chars - suffix.chars().count(),
                }
            };
        } else {
            result_text = before + prefix + &source + suffix + &after;
            result_selection = selection + prefix_chars;
        }

        (result_text, result_selection)
    }

    fn start_line_layout(
        text: Vec<char>, prefix: &str, selection: Selection, replacement: Selection,
    ) -> (String, Selection) {
        let before: String = text[..replacement.start].iter().collect();
        let source: String = text[replacement.start..replacement.end].iter().collect();
        let after: String = text[replacement.end..].iter().collect();

        let lines: Vec<&str> = source.lines().collect();
        let is_prefixed_lines = lines.iter().skip(1).all(|line| line.starts_with(prefix));
        let block_prefix = format!("\n{}", prefix);
        let prefix_chars = prefix.chars().count();
        let result_text;
        let result_selection;

        if is_prefixed_lines && (before.ends_with(&block_prefix) || &before == prefix) {
            let mut target = String::new();
            for (idx, line) in lines.iter().enumerate() {
                if idx > 0 {
                    target.push('\n');
                    target.push_str(line.trim_start_matches(prefix));
                } else {
                    target.push_str(line);
                }
            }

            result_text = before.trim_end_matches(prefix).to_string() + &target + &after;
            result_selection = Selection {
                start: selection.start - prefix_chars,
                end: selection.end - prefix_chars * lines.len(),
            }
        } else if is_prefixed_lines && (before.is_empty() || before.ends_with('\n')) && source.starts_with(prefix) {
            let mut target = String::new();
            for (idx, line) in lines.iter().enumerate() {
                if idx > 0 {
                    target.push('\n');
                }
                target.push_str(line.trim_start_matches(prefix));
            }

            result_text = before + &target + &after;
            result_selection = if selection.is_empty() {
                selection - prefix_chars.min(selection.start - replacement.start)
            } else {
                Selection {
                    start: selection.start,
                    end: selection.end - prefix_chars * lines.len(),
                }
            };
        } else if is_prefixed_lines && source.starts_with(&block_prefix) {
            let mut target = String::new();
            for (idx, line) in lines.iter().enumerate() {
                target.push('\n');
                if idx > 0 {
                    target.push_str(line.trim_start_matches(prefix));
                } else {
                    target.push_str(line.trim_start_matches(&block_prefix));
                }
            }

            result_text = before + &target + &after;
            result_selection = Selection {
                start: selection.start - prefix_chars,
                end: selection.end - prefix_chars * lines.len(),
            };
        } else {
            let before_endline = before.is_empty() || before.ends_with('\n');
            let mut target = String::new();
            if !before_endline {
                target.push('\n');
            }
            target.push_str(&prefix);
            for (idx, line) in lines.iter().enumerate() {
                if idx > 0 {
                    target.push_str(&block_prefix);
                }
                target.push_str(line);
            }

            result_text = before + &target + &after;
            result_selection = if selection.is_empty() {
                selection + prefix_chars
            } else {
                Selection {
                    start: selection.start + if before_endline { 0 } else { 1 } + prefix_chars,
                    end: selection.end + if before_endline { 0 } else { 1 } + prefix_chars * lines.len(),
                }
            };
        }

        (result_text, result_selection)
    }
}

impl<Before: Into<String>, After: Into<String>> From<(Before, After)> for ReplaceFmt {
    fn from((before, after): (Before, After)) -> Self {
        Self::Around(before.into(), after.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnselectedApplyMode {
    Word,
    Line,
    FromWordToEndLine,
}

impl UnselectedApplyMode {
    pub fn is_start_boundary(&self, ch: char) -> bool {
        match self {
            UnselectedApplyMode::Word | UnselectedApplyMode::FromWordToEndLine => ch.is_whitespace(),
            UnselectedApplyMode::Line => ch == '\n',
        }
    }

    pub fn is_end_boundary(&self, ch: char) -> bool {
        match self {
            UnselectedApplyMode::Word => ch.is_whitespace(),
            UnselectedApplyMode::Line | UnselectedApplyMode::FromWordToEndLine => ch == '\n',
        }
    }
}

impl Default for UnselectedApplyMode {
    fn default() -> Self {
        Self::Word
    }
}
