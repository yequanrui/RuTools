use console::{style, StyledObject, Term};

/// 初始化控制台实例，并设置标题
pub fn stdout(title: &str) -> Term {
    let term = Term::stdout();
    term.set_title(title);
    term
}

/// 创建带有信息样式的文本（青色）
pub fn info<D>(msg: D) -> StyledObject<D> {
    style(msg).cyan()
}

/// 创建带有成功样式的文本（绿色）
pub fn success<D>(msg: D) -> StyledObject<D> {
    style(msg).green()
}

/// 创建带有警告样式的文本（黄色）
pub fn warning<D>(msg: D) -> StyledObject<D> {
    style(msg).yellow()
}

/// 创建带有错误样式的文本（红色）
pub fn error<D>(msg: D) -> StyledObject<D> {
    style(msg).red()
}
