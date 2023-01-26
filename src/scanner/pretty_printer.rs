use crate::expr::Expr;
use crate::scanner::token_type::Literal;

pub struct PrettyPrinter {
    inner: String,
    indent_size: usize,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
            indent_size: 4,
        }
    }

    pub fn print_expr(mut self, expr: &Expr<'_>) {
        self.push_expr(expr);

        println!("{}", self.inner);
    }

    fn pop(&mut self) -> &mut Self {
        self.inner.pop();

        self
    }

    fn push<S>(&mut self, s: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.inner.push_str(s.as_ref());

        self
    }

    fn push_char(&mut self, c: char) -> &mut Self {
        self.inner.push(c);

        self
    }

    fn push_expr(&mut self, expr: &Expr) -> &mut Self {
        match expr {
            Expr::Assign(assign) => self
                .push(assign.name.build_string())
                .push(" = ")
                .push_expr(assign.value.as_ref()),

            Expr::Binary(bin) => self
                .push_expr(&bin.left)
                .push_char(' ')
                .push(bin.operator.build_string())
                .push_char(' ')
                .push_expr(&bin.right),

            Expr::Call(call) => {
                self.push_expr(call.callee.as_ref()).push_char(')');

                for arg in &call.arguments {
                    self.push_expr(arg).push_char(',');
                }

                self.pop()
            }

            Expr::Get(get) => self
                .push_expr(get.object.as_ref())
                .push_char('.')
                .push(get.name.build_string()),

            Expr::Grouping(grouping) => self
                .push_char('(')
                .push_expr(grouping.expression.as_ref())
                .push_char(')'),

            Expr::Literal(literal) => match &literal.value {
                Literal::String(..) => self
                    .push_char('"')
                    .push(literal.value.build_string())
                    .push_char('"'),
                _ => self.push(literal.value.build_string()),
            },

            Expr::Set(set) => self
                .push_expr(set.object.as_ref())
                .push_char('.')
                .push(set.name.build_string())
                .push(" = ")
                .push_expr(set.value.as_ref())
                .push_char(';'),

            Expr::Super(s) => self
                .push("super")
                .push_char('.')
                .push(s.method.build_string()),

            Expr::This(..) => self.push("this"),

            Expr::Unary(unary) => self
                .push(unary.operator.build_string())
                .push_expr(unary.right.as_ref()),

            Expr::Variable(var) => self.push(var.name.build_string()),
        }
    }
}
