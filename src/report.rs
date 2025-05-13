pub fn report_error_multiline(
    start_line: usize,
    context_lines: &[String],
    start_column: usize,
    message: &str,
) {
    eprintln!("Error: {}", message);
    eprintln!();

    // 打印所有相关行
    for (i, line) in context_lines.iter().enumerate() {
        eprintln!("{:>4} | {}", start_line + i, line);

        // 只在第一行（字符串开始的地方）显示错误指示符
        if i == 0 {
            eprintln!(
                "     | {}{}",
                " ".repeat(start_column - 1),
                "^-- String starts here"
            );
        }
    }
    // 在最后一行后面显示 "never closes" 提示
    eprintln!("     | {}", "... string never closes");
}

pub fn report_error(line: usize, source_lines: &[String], column: usize, message: &str) {
    if let Some(line_content) = source_lines.get(line - 1) {
        eprintln!("Error: {}", message);
        eprintln!();
        eprintln!("{:>4} | {}", line, line_content);
        eprintln!("     | {}{}", " ".repeat(column - 1), "^-- Here.");
    }
}
