// recursive descent parser
use lexer::*;

pub mod node {
    #[derive(Default, Debug)]
    pub struct Program {
        pub statements: Vec<Statement>
    }

    #[derive(Default, Debug)]
    pub struct Statement {
        pub expressions: Vec<Expression>
    }

    #[derive(Debug)]
    pub enum Expression {
        Scope(Scope),
        IntLit(isize),
        StringLit(String),
        Variable()
    }

    // scopes
    #[derive(Debug, Default)]
    pub enum ScopeType {
        Function,
        If,
        Loop,
        #[default]
        Local
    }

    #[derive(Debug, Default)]
    pub struct Scope {
        pub kind: ScopeType,
        pub signature: Option<Statement>,
        pub body: Program
    }
}

struct Parser<'a> {
    tokens: &'a mut Vec<Lexeme>
}

impl Parser<'_> {
    // used to evaluate integer literal expressions involving increment and decrement
    fn eval_lit(&mut self) -> isize {
        let mut value = Default::default();

        while let Some(lexeme) = self.tokens.pop() {
            let increment = lexeme == Lexeme::Token(Token::Increment);
            let decrement = lexeme == Lexeme::Token(Token::Decrement);

            if !increment && !decrement {
                break;
            }

            value += increment as isize;
            value -= decrement as isize;
        }

        value
    }


    fn parse_int(&mut self) -> Option<isize> {
        Some(self.eval_lit())
    }

    // TODO: return type Result with ScopeError type that can be error for condition, body, etc.
    fn parse_scope(&mut self) -> Option<node::Scope> {
        let mut scope: node::Scope = Default::default();

        scope.kind = match self.tokens.pop() {
            Some(Lexeme::Token(Token::Access)) => node::ScopeType::Function,
            Some(Lexeme::Token(Token::Repeat)) => node::ScopeType::Loop,
            // TODO: if statement
            _ => return None
        };

        scope.signature = Some(self.parse_line()?);

        scope.body = self.parse_body()?;

        Some(scope)
    }

    fn parse_quote(&mut self) -> Option<node::Expression> {
        let string = node::Expression::StringLit(self.parse_int()?.to_string());
        self.tokens.pop(); // pops the ending quote
        Some(string)
    }

    // TODO: this is duplicate code for parse()
    fn parse_body(&mut self) -> Option<node::Program> {
        let mut program: node::Program = Default::default();

        let mut line = self.parse_line();
        while !line.as_ref()?.expressions.is_empty() {
            program.statements.push(line.unwrap());

            line = self.parse_line();
        }

        Some(program)
    }

    // TODO: definantly not the best wasy to handle scope body parsing beign a bool
    fn parse_line(&mut self) -> Option<node::Statement> {
        use node::Expression::*;

        let mut statement: node::Statement = Default::default();

        // TODO: should node::Expressions be put here or should the parsing functions return them?
        // TODO: replace unwraps with proper error handling
        while let Some(lexeme) = self.tokens.pop() {
            statement.expressions.push(match lexeme {
                Lexeme::Token(Token::Zero)      => IntLit(self.parse_int()?),
                Lexeme::Token(Token::Increment) => unreachable!(),
                Lexeme::Token(Token::Decrement) => unreachable!(),
                Lexeme::Token(Token::Access)    => todo!(),
                Lexeme::Token(Token::Repeat)    => unreachable!(),
                Lexeme::Token(Token::Quote)     => self.parse_quote()?,
                Lexeme::Token(Token::Variable)  => unreachable!(),
                Lexeme::Token(Token::ScopeStart)=> Scope(self.parse_scope()?),
                Lexeme::Token(Token::ScopeEnd)  => unreachable!(),

                Lexeme::Identifier(id)          => todo!(),

                Lexeme::Token(Token::LineBreak) => return Some(statement)
            });
        }

        None
    }
}

pub fn parse(tokens: &mut Vec<Lexeme>) -> Result<node::Program, String> {
    tokens.reverse(); // TODO: is reversing first faster than pop_back()?
    let mut parser = Parser {
        tokens
    };
    let mut program: node::Program = Default::default();

    let mut line_numb = 1;
    let mut line = parser.parse_line();
    while !line.as_ref().ok_or(format!("invalid syntax at line {}", line_numb))?.expressions.is_empty() {
        program.statements.push(line.unwrap());

        line = parser.parse_line();
        line_numb += 1;
    }

    Ok(program)
}
