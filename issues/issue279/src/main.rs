use logos::Logos;

#[derive(Logos, Debug)]
pub enum Token {
    #[token(r"\")]
    Backslash,
    #[token(r"\\")]
    DoubleBackslash,
    #[token(r"\begin")]
    EnvironmentBegin,
    #[token(r"\end")]
    EnvironmentEnd,
    //#[token(r"\begin{document}")] // <- the part that creates problems
    DocumentBegin,                  // <-
    #[regex(r"\\[a-zA-Z]+")]
    MacroName,
    Error,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    macro_rules! assert_token_positions {
        ($source:expr, $token:pat, $($pos:expr),+ $(,)?) => {
            let source = $source;

            let positions: Vec<std::ops::Range<usize>> = vec![$($pos),*];
            let spanned_token: Vec<_> = Token::lexer(source)
                .spanned()
                .filter(|(token, _)| matches!(token, $token))
                .collect();


            let strs: Vec<_> = Token::lexer(source)
                .spanned()
                .map(|(token, span)| (token, source[span].to_string()))
                .collect();

            assert_eq!(
                spanned_token.len(), positions.len(),
                "The number of tokens found did not match the expected number of positions {strs:?}"
            );

            for (pos, (token, span)) in positions.into_iter().zip(spanned_token) {
                assert_eq!(
                    pos,
                    span,
                    "Token {token:#?} was found, but expected at {pos:?}"
                );
            }
        };
    }

    #[test]
    fn token_backslash() {
        assert_token_positions!(r"Should match \+, but not \\+", Token::Backslash, 13..14,);
    }
    #[test]
    fn token_double_backslash() {
        assert_token_positions!(
            r"Should match \\, but not \",
            Token::DoubleBackslash,
            13..15,
        );
    }
    #[test]
    fn token_environment_begin() {
        assert_token_positions!(r"\begin{equation}", Token::EnvironmentBegin, 0..6,);
    }
    #[test]
    fn token_environment_end() {
        assert_token_positions!(r"\end{equation}", Token::EnvironmentEnd, 0..4,);
    }
    #[test]
    fn token_macro_name() {
        assert_token_positions!(
            r"\sin\cos\text{some text}\alpha1234",
            Token::MacroName,
            0..4,
            4..8,
            8..13,
            24..30,
        );
    }
}