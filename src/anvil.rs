///! lexer and parser for asl
///! loads file from config
///! single pass

//- definitions
pub struct  Ast {
	root: &node,
}

pub struct Node {
    pub _type : node_type,
	pub literal: &str,
	pub lexeme: &str,
	pub line: i32,
}


enum NodeType {
    SectionName,
    OpenBrace,
    CloseBrace,
    Field,
    Value,

}
enum TokenType {
    String,
    Integer,
    Declaration,
    OpenBrace,
    CloseBrace,
}

struct Lexer {
	pub buffer : &str,
    pub position: &char
	pub tokens : Vec<token>,
}

struct Token {
   pub lexeme: &str,
   pub token: TokenType,
}
//-

//- Helper
#[inline]
fn is_digit(character: u8)
{
    b32 Result = (Char >= '0' && Char <= '9');
    return Result;
}

#[inline]
fn is_rune_alpha(character: u8) {
     return ((Char >= 'A' && Char <= 'Z') || (Char >= 'a' && Char <= 'z'));
}


#[inline]
fn is_digit(character: u8) {
    return (Char >= '0' && Char <= '9');
}

#[inline]
fn is_alpha(character: &char) {
     return ((Char >= 'A' && Char <= 'Z') || (Char >= 'a' && Char <= 'z'));
}

//-


fn is_alpha_numeric(character: &char) -> bool  {

	if character == 
	return false
}

/// lexer
fn scan_tokens(file_path: &str) {

    let buffer = fs::read_to_string(file_path)
        .expect("[ERROR] Failed to read file");

	for (index, character) in buffer.chars() {
		// NOTE(nasr): because we enforce unix style file stuff.
		// There is no need to have windows new line characters 
		match character {
			//" " => continue,
			"\n" => continue,
			" " => continue,
		}
		

let word_end = 0;
let word_start = 0;

					token::new();

if is_numeric(character) {

			token.type = TokenType.Integer;
			token.lexeme = character;

			continue;
		} else if is_alpha(character) {

			
//- build the lexeme
{
word_start = idx;
for buffer[idx] != " " || buffer[idx] != "\n" {
idx++
}
                token.lexeme = &full_string[word_start..word_end];
}

 word_end = idx;

token.type = TokenType.String ;
		} else if character == "\"" {

for buffer[idx] != " " || buffer[idx] != "\n {
idx++;
}
token.lexeme = character;

}
}
}


/// parser
fn build_ast(lexer: Lexer) -> ast {

	let tree = ast::new();

}

fn load_config_file() {


}