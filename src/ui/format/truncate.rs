pub fn truncate_middle(row: &str, max_length: u16) -> String {
    if max_length < 6 {
        let mut res = String::from(row);
        res.truncate(max_length as usize);
        res
    } else if row.len() as u16 > max_length {
        let first_slice = &row[0..(max_length as usize / 2) - 2];
        let second_slice = &row[(row.len() - (max_length / 2) as usize + 2)..row.len()];
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
