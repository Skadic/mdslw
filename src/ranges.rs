/* An opinionated line wrapper for markdown files.
Copyright (C) 2023  Torsten Long

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::parse::CharRange;

#[derive(Debug)]
/// TextRange describes a range of characters in a document including whether they shall be
/// repeated verbatim or not. It also contains the number of spaces of indent to use when wrapping
/// the contained text.
pub struct TextRange {
    pub indent_spaces: usize,
    pub range: CharRange,
    pub verbatim: bool,
}

/// The first arguments contains those ranges in the document that shall be wrapped. Every
/// character in the document that is not inside such a range will be taken verbatim. This also
/// determines the starting indent in spaces for every range that shall be wrapped.
pub fn fill_markdown_ranges(wrap_ranges: Vec<CharRange>, text: &String) -> Vec<TextRange> {
    let mut last_end = 0;

    let lines = line_ranges(text);

    wrap_ranges
        .into_iter()
        // Append an element that points at the end of the document to ensure that we always add
        // the last ranges in the document because we always add a verbatim range before the
        // non-verbatim range.
        .chain([CharRange {
            start: text.len(),
            end: text.len(),
        }])
        .flat_map(|el| {
            let verbatim_line_start = find_line_start(last_end, &lines).unwrap_or(last_end);
            let verbatim = TextRange {
                verbatim: true,
                // This can never panic because, if a range contains the point, the difference to
                // its start will never be smaller than 0.
                indent_spaces: last_end - verbatim_line_start,
                range: CharRange {
                    start: last_end,
                    end: el.start,
                },
            };
            last_end = el.end;

            let wrap_line_start = find_line_start(el.start, &lines).unwrap_or(el.start);
            let wrap = TextRange {
                verbatim: false,
                indent_spaces: el.start - wrap_line_start,
                range: el,
            };
            [verbatim, wrap]
        })
        .filter(|el| el.range.len() > 1)
        .collect::<Vec<_>>()
}

/// Determine character ranges for each line in the document.
fn line_ranges(text: &String) -> Vec<CharRange> {
    let mut start = 0;

    text.split_inclusive("\n")
        .map(|el| {
            let end = start + el.len();
            let range = CharRange { start, end };
            start = end;
            range
        })
        .collect::<Vec<_>>()
}

/// Find the start of the line that "point" is in.
fn find_line_start(point: usize, line_ranges: &Vec<CharRange>) -> Option<usize> {
    line_ranges
        .into_iter()
        .find(|el| el.contains(&point))
        .map(|el| el.start)
}
