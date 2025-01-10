use console::{style, StyledObject, Term};

pub fn stdout(title: &str) -> Term {
    let term = Term::stdout();
    term.set_title(title);
    term
}

pub fn info<D>(msg: D) -> StyledObject<D> {
    style(msg).cyan()
}

pub fn success<D>(msg: D) -> StyledObject<D> {
    style(msg).green()
}

pub fn warning<D>(msg: D) -> StyledObject<D> {
    style(msg).yellow()
}

pub fn error<D>(msg: D) -> StyledObject<D> {
    style(msg).red()
}
