/// ! June 01 21:59 - nasr
/// ! build tool
/// ! Inspired by nob.h ( by Mr. Zozin )
use std::env;
use std::process::Command;
use std::fs;

mod mender_lexer;
mod mender_ast;
//- lexer definitions

use std::collections::HashMap;

static COMMON_FLAGS: Vec<&str> = ["-g"];

pub struct mender_token<'a> {
    pub lexeme:     &'a str,
    pub token_type: token_type,
}

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



const fields: &str = ["paths", "grub", "build", "vm", "envirnoment"];

//- general config
struct target<'a> {
compiler: &'str,
    timeout:  u8,
    _default: u8,
    entry:    String,
    dir:      &'a str,
    file:     &'a str,
project:  &'a str
}
s
fn is_debug_build() -> bool {
    cfg!(debug_assertions)
}

fn command() -> Command {
    let mut command = Command::new("rustup");
    command.args(["run", "nightly", "cargo"]);
    return command;
}

fn build(cfg: mender_config) {
    if is_debug_build() {
        println!("INFO: Starting kernel build process...");
        let build_status = cargo_command()
            .args(&["build", "-Z ", "json-target-spec", "-Z ", "build-std=core,alloc", "--target-dir ", "{}", cfg.dir])
            .status();

        match build_status {
            Ok(status) if status.success() => {
                println!("INFO: Cargo build completed successfully.");
            }
            other => panic!("[ERROR] Build failed or cargo crashed: {:?}", other),
        }
        println!("INFO: Running validation tests...");
        let test_status = cargo_command().args(["test"]).status();

        if test_status.is_err() || !test_status.unwrap().success() {
            let _ = cargo_command().args(["clean"]).status();
            panic!("[ERROR] Tests failed! Cleaned target directory.");
        }

        println!("INFO: Generating ISO...");

        //- TODO(nasr): fill in the acl values
        build_iso("", "");
    } else {
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let default_location: &str = "./mender.acl";

    let cfg: &mender_config;
    //- command line arguments handling

    if args.len() < 2 {
        eprintln!("Usage: mender run [run|build|test|setup]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "--config" => {
            //- load config -- refactor, inlined from the previous function
            // call the lexer on the buffer, parse the tokens, and build the config
            {
                let file_path: &str = &args[2];

                if &args[2] == "" {
                    panic!("[ERROR] invalid path");
                }

                let buffer = fs::read_to_string(file_path).expect("[ERROR] Failed to read file");

                let lexer: mender_lexer::lexer = mender_lexer::scan_tokens(&buffer);
            }
        }

        "build" => {
            println!("INFO: Building kernel...");

            {
                // TODO(nasr): fill in the appropriate values
                // TODO(nasrr): get the arguments from the ast
                // build();
            }
        }

        "run" => {
            println!("INFO: Running VM...");
            run_vm();
        }

        "test" => {
            println!("INFO: Running tests...");
            // TODO: Implement test command
            let test_status = cargo_command().args(["test"]).status();

            match test_status {
                Ok(status) if status.success() => println!("INFO: Tests passed."),
                _ => panic!("[ERROR] Tests failed"),
            }
        }

        "setup" => {
            setup_dev_env();
        }

        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: build, run, test, setup.");
        }
    }
}
