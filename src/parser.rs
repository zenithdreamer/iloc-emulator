pub fn parse_iloc(program: &str) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    let mut in_block_comment = false;

    for line in program.lines() {
        let mut trimmed_line = line.trim().to_string();

        if in_block_comment {
            if let Some(end_pos) = trimmed_line.find("*/") {
                trimmed_line = trimmed_line[end_pos + 2..].to_string(); // Skip past the block comment
                in_block_comment = false;
            } else {
                continue; // Ignore lines within block comments
            }
        }

        if let Some(start_pos) = trimmed_line.find("/*") {
            if let Some(end_pos) = trimmed_line[start_pos..].find("*/") {
                let formatted_line = format!(
                    "{}{}",
                    &trimmed_line[..start_pos],
                    &trimmed_line[start_pos + end_pos + 2..]
                );
                let formatted_line = formatted_line.trim().to_string();
                trimmed_line = formatted_line;
            } else {
                trimmed_line = trimmed_line[..start_pos].to_string();
                in_block_comment = true;
            }
        }

        let without_comments = trimmed_line
            .split('#')
            .next()
            .unwrap_or("")
            .split("//")
            .next()
            .unwrap_or("")
            .trim();

        if !without_comments.is_empty() {
            let normalized_line = without_comments
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(" ")
                .replace(", ", ",");
            lines.push(normalized_line);
        }
    }

    if lines.is_empty() {
        return Err("Program is empty".to_string());
    }

    Ok(lines)
}
