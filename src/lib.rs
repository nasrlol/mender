//- lexer definitions

use std::collections::HashMap;

pub struct mender_token<'a> {
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

pub struct mender_lexer<'a> {
    pub position: u8,
    pub tokens:   Vec<token<'a>>,
    // the lifetime of the buffer lasts longer i think because we build the tokens a while later
    pub buffer:   &'a str,
}


struct mender_config_section<'a> {
    name:      &'a str,

    values:    HashMap<str, any>,
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
    let mut lexer = mender_lexer {
        buffer: buffer,
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

//- ast definitions
struct mender_ast<'a> {
    nodes:       Vec<mender_node>,
    root_idx:    usize,
    current_idx: usize,
}

// are the lifetimes correct
struct mender_node {

    // we store the indeces and we can act on them as the pointers to the global node struct
    // thne a mender node really is just a pointer and doesnt hold any values
    next_node:   Option<usize>,
    parent_node: Option<usize>,
    child_node:  Option<usize>,
}

//- Ast helpers
// a section starts with an open brace and ends with a closing brace
// all the values are children of the parent value

// the starting index can now be derivde
fn handle_tokens_section(section_name: &str, starting_index: i32, length: i32, tokens: token) {

    // TODO: how do you determine end the string in rust
    // TODO: find the first bracket

    let mut index: i32 = starting_index;

    while tokens[index] != length {

        if tokens[index] != (token_type::CloseBrace) {

            //- scan for the last section
            new_section : mender_config_section {

                name:      section_name,

		// TODO: fill in the values into a hashmap
            }
        }

        index += 1;
    }
}

fn build_ast_based_on_token_stream<'a>(lexer: mender_lexer) -> mender_ast<'a> {
    // TODO(nasr): find out how to fill in the null types
    const tree: mender_ast = mender_ast {
        root: mender_node {
            token:       mender_token {
                lexeme:     "",
                token_type: token_type::Undefined,
            },
            next_node:   Some(0),
            parent_node: Some(0),
        },

        current: mender_node {
            next_node:   Some(0),
            parent_node: Some(0),
	    child_node:  Some(0),
        },
    };

    for (index, current_token) in lexer.tokens {
        match token.token_type {
            token_type::Undefined => {
                // panic out
                eprintln("[ERROR] syntax error in acl. {}", token);
            }

            token_type::String => {
                if token.current.parent_node.token_type == token_type::Declaration {
                    let node: mender_node = mender_node {};

                    current_token.children.push(node);
                }
            }

            token_type::Integer => if root.current.token_type == token_type::Declaration {},
            token_type::Declaration => if lexer.tokens[idx + 1].token_type == mender::token_type::token_open_brace {},
            token_type::OpenBrace => {

                // check if the previous token is of the type token_type::Declaration
                if lexer.tokens[index - 1] == token_type::Declaration {

                    handle_token_section(lexer.tokens[index - 1], index, lexer.tokens.lenght(), token);

                }

            }

            token_type::CloseBrace => {
                // NOTE(nasr): this gets handlesd in the section handler
                continue;
            }

            // TODO(nasr): handle arrays
            token_type::OpenBracket => {}
            token_type::CloseBracket => {}

            token_type::Slash => {
                if token.next_token.token_type == token_type::Slash {
                    skip_until_reaching_slash(token_type::Slash);
                }
            }
        }
    }
}
