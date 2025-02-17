use std::rc::Rc;

// # following https://ruslanspivak.com/lsbasi-part7/...
pub struct LarskoAst();

enum Token {
    Num(u32),
    UnaryOp(OpType, Rc<Token>), // i'm already lost too, dw about it
    BinOp(Rc<Token>, OpType, Rc<Token>),
}

enum OpType {}

struct Parser {
    
}

// class Parser:
//     def __init__(self, lexer):
//         self.lexer = lexer
//         self.current_token = self.lexer.get_next_token()

//     def error(self):
//         raise Exception("Invalid Lasko Syntax:TM:")

//     def eat(self, token_type):
//         # compare the current token type with the passed token
//         # type and if they match then "eat" the current token
//         # and assign the next token to the self.current_token,
//         # otherwise raise an exception.
//         if self.current_token.type == token_type:
//             self.current_token = self.lexer.get_next_token()
//         else:
//             self.error()

//     def factor(self):
//         """factor : INTEGER | LPAREN expr RPAREN"""
//         token = self.current_token
//         if token.type == INTEGER:
//             self.eat(INTEGER)
//             return Num(token)
//         elif token.type == LPAREN:
//             self.eat(LPAREN)
//             node = self.expr()
//             self.eat(RPAREN)
//             return node

//     def term(self):
//         """term : factor ((MUL | DIV) factor)*"""
//         node = self.factor()

//         while self.current_token.type in (MUL, DIV):
//             token = self.current_token
//             if token.type == MUL:
//                 self.eat(MUL)
//             elif token.type == DIV:
//                 self.eat(DIV)

//             node = BinOp(left=node, op=token, right=self.factor())

//         return node

//     def expr(self):
//         """
//         expr   : term ((PLUS | MINUS) term)*
//         term   : factor ((MUL | DIV) factor)*
//         factor : INTEGER | LPAREN expr RPAREN
//         """
//         node = self.term()

//         while self.current_token.type in (PLUS, MINUS):
//             token = self.current_token
//             if token.type == PLUS:
//                 self.eat(PLUS)
//             elif token.type == MINUS:
//                 self.eat(MINUS)

//             node = BinOp(left=node, op=token, right=self.term())

//         return node

//     def parse(self):
//         return self.expr()
