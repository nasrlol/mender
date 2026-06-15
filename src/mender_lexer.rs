//- lexer definitions

pub struct token<'a> {
    pub lexeme:     &'a str,
    pub token_type: token_type,
}

pub enum token_type {
    Undefined,
    String,
    Declaration,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Slash,
}

pub struct lexer<'a, 'b> {
    pub position: u8,
    pub tokens:   Vec<token<'b>>,
    // the lifetime of the buffer lasts longer i think because we build the tokens a while later
    pub buffer:   &'a str,
}

//- lexical helpers
#[inline]
fn is_numeric(character: u8) -> bool {
    return (character >= b'0') && (character <= b'9');
}

#[inline]
fn is_rune_alpha(character: u8) -> bool {
    return (character >= b'A' && character <= b'Z') || (character >= b'a' && character <= b'z');
}

#[inline]
fn is_alpha(character: u8) -> bool {
    return (character >= b'A' && character <= b'Z') || (character >= b'a' && character <= b'z');
}

#[inline]
fn is_whitespace(character: u8) -> bool {
    return (character == b'\n') && (character == b' ');
}

//- lexical implementatoin
// TODO(nasr): rewrite this garbage
pub fn scan_tokens<'a>(buffer: &'a str) -> lexer<'a, 'a> {
    let mut lexer = lexer {
        buffer,
        position: 0,
        tokens: Vec::new(),
    };

    let mut idx = 0;
    let bytes = buffer.as_bytes();

    while idx < bytes.len() {
        let character = bytes[idx];

        if is_whitespace(character) {
            idx += 1;
            continue;
        }

        if is_numeric(character) {
            idx += 1;
            continue;
        }

        if is_alpha(character) {
            let word_start = idx;
            while idx < bytes.len() && !is_whitespace(bytes[idx]) {
                idx += 1;
            }
            let word_end = idx;

            let token = token {
                lexeme:     &buffer[word_start..word_end],
                token_type: token_type::String,
            };
            lexer.tokens.push(token);
            continue;
        }

        if character == b'"' {
            idx += 1;
            let string_start = idx;
            while idx < bytes.len() && bytes[idx] != b'"' {
                idx += 1;
            }
            let string_end = idx;
            if idx < bytes.len() {
                idx += 1;
            }

            let token = token {
                lexeme:     &buffer[string_start..string_end],
                token_type: token_type::String,
            };
            lexer.tokens.push(token);
            continue;
        }

        let token_type = match character {
            b'{' => token_type::OpenBrace,
            b'}' => token_type::CloseBrace,
            b'[' => token_type::OpenBracket,
            b']' => token_type::CloseBracket,
            b'/' => token_type::Slash,
            _ => {
                idx += 1;
                continue;
            }
        };

        let token = token {
            lexeme: &buffer[idx..idx + 1],
            token_type,
        };
        lexer.tokens.push(token);
        idx += 1;
    }

    return lexer;
}
