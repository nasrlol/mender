///! lexer and parser for asl
///! loads file from config
///! single pass

//- lexer definitions
pub struct  Ast {
	root: &node,
}

pub struct Node {
     pub token: Token,
pub(crate) next_node: &node,
pub(crate)
}

//- ast definitions 

pub(crate) struct Lexer {
	pub buffer : &str,
    pub position: &char,
	pub tokens : Vec<token>,
}

struct Token {
	pub lexeme: &str,
	pub token_type: TokenType,

}

enum TokenType {
    String,
    Integer,
    Declaration,
    OpenBrace,
    CloseBrace,
OpenBracket,
CloseBracket,
Slash,
}

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
fn scan_tokens(buffer : &str) -> Lexer {
	
	lexer : Lexer = Lexer::new();
	
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
			
				word_start = idx;
				for buffer[idx] != " " || buffer[idx] != "\n" {
					idx++
				}
                token.lexeme = &full_string[word_start..word_end];
			
			word_end = idx;
			token.token_type = TokenType.String ;

		} else if character == "\"" {
			
			for buffer[idx] != " " || buffer[idx] != "\n {
				idx++;

		} else if buffer[idx] == "{" {

token.token_type = OpenBrace;
token.Lexeme = buffer[idx];

} else if buffer[idx] == "}" {

token.token_type = CloseBrace;
token.Lexeme = buffer[idx];

} else if buffer[idx] == "[" {

token.token_type = OpenBracket;
token.lexeme = buffer[idx];

} else if buffer[idx] == "]" {

token.token_type = CloseBracket;
token.lexeme = buffer[idx];

} else if buffer[idx] == "/" {

token.token_type = Slash;
 token.lexeme = buffer[idx];
}

//-

		token.lexeme = character;
		lexer.tokens.append(Token);
	}
	return Lexer;
}

/// parser
fn build_ast(lexer: Lexer) -> Ast  {
	
	let tree = ast::new();
tree.root = node::new();

for token in &lexer.tokens. {
if token.token-
root.next = token;
}


return tree;
}