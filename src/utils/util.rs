pub fn extract_relevant_lines<'a>(
	mut stderr: &'a str,
	strip_start_tokens: &[&str],
	strip_end_tokens: &[&str],
) -> &'a str {
	// Find best matching start token
	if let Some(start_token_pos) = strip_start_tokens
		.iter()
		.filter_map(|t| stderr.rfind(t))
		.max()
	{
		// Keep only lines after that
		stderr = match stderr[start_token_pos..].find('\n') {
			Some(line_end) => &stderr[(line_end + start_token_pos + 1)..],
			None => "",
		};
	}

	// Find best matching end token
	if let Some(end_token_pos) = strip_end_tokens
		.iter()
		.filter_map(|t| stderr.rfind(t))
		.min()
	{
		// Keep only lines before that
		stderr = match stderr[..end_token_pos].rfind('\n') {
			Some(prev_line_end) => &stderr[..=prev_line_end],
			None => "",
		};
	}

	// Strip trailing or leading empty lines
	stderr = stderr.trim_start_matches('\n');
	while stderr.ends_with("\n\n") {
		stderr = &stderr[..(stderr.len() - 1)];
	}

	stderr
}