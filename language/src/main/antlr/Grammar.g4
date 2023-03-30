grammar GrammarParser;

@header {
package io.github.jamalam360.modifier.language;
}

@lexer::members {
	private int INDENT_TOKEN = GrammarParser.INDENT;
	private int DEDENT_TOKEN = GrammarParser.DEDENT;
}

import DentLexer;

program : ( NEWLINE | statement )* EOF ;

statement
	:	simpleStatement
	|	blockStatement
	;

simpleStatement : 'a' NEWLINE? ;

blockStatement : 'b' NEWLINE INDENT statement+ DEDENT ;

COMMENT : '#' ~[\r\n]* -> channel(HIDDEN) ;
