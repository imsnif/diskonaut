use std::iter::FromIterator;
use unicode_width::UnicodeWidthChar;

fn truncate_iter_to_unicode_width<Input, Collect>(iter: Input, width: usize) -> Collect
where
    Input: Iterator<Item = char>,
    Collect: FromIterator<char>,
{
    let mut chunk_width = 0;
    iter
        .take_while(|ch| {
            chunk_width += ch.width().unwrap_or(0);
            chunk_width <= width
        })
        .collect()
}

pub fn truncate_middle(row: &str, max_length: u16) -> String {
    if max_length < 6 {
        truncate_iter_to_unicode_width(row.chars(), max_length as usize)
    } else if row.len() as u16 > max_length {
        let split_point = (max_length as usize / 2) - 2;
        let first_slice = truncate_iter_to_unicode_width::<_, String>(row.chars(), split_point);
        let second_slice =
            truncate_iter_to_unicode_width::<_, Vec<_>>(row.chars().rev(), split_point)
                .into_iter()
                .rev()
                .collect::<String>();

        if max_length % 2 == 0 {
            format!("{}[...]{}", first_slice, second_slice)
        } else {
            format!("{}[..]{}", first_slice, second_slice)
        }
    } else {
        row.to_string()
    }
}

pub fn truncate_end(row: &str, max_len: u16) -> String {
    if row.chars().count() > max_len as usize {
        let mut truncated = String::from(row);
        truncated.truncate(max_len as usize - 3);
        format!("{}...", truncated)
    } else {
        row.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_middle_char_boundary() {
        assert_eq!(
            truncate_middle("굿걸 - 누가 방송국을 털었나 E06.mp4", 44),
            "굿걸 - 누가 방송국을[...]국을 털었나 E06.mp4",
        );
    }
}
