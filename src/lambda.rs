#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lambda {
    Variable(String),
    Function(String, Box<Lambda>),
    Application(Box<Lambda>, Box<Lambda>),
}
impl Lambda {
    pub fn variable(name: &str) -> Self {
        Self::Variable(name.to_owned())
    }
    pub fn function(args: Vec<&str>, body: Lambda) -> Self {
        args.into_iter().rev().fold(body, |acc, arg| {
            Lambda::Function(arg.to_owned(), Box::new(acc))
        })
    }
    pub fn application(func: Lambda, args: Vec<Lambda>) -> Self {
        args.into_iter().fold(func, |acc, arg| {
            Lambda::Application(Box::new(acc), Box::new(arg))
        })
    }

    fn substitute(&self, var: &str, value: &Lambda) -> Self {
        match self {
            Lambda::Variable(name) if name == var => value.clone(),
            Lambda::Variable(_) => self.clone(),
            Lambda::Function(param, _) if param == var => self.clone(),
            Lambda::Function(param, body) => {
                Lambda::Function(param.clone(), Box::new(body.substitute(var, value)))
            }
            Lambda::Application(f, arg) => Lambda::Application(
                Box::new(f.substitute(var, value)),
                Box::new(arg.substitute(var, value)),
            ),
        }
    }
    pub fn reduce(&self) -> Self {
        match self {
            Lambda::Variable(_) => self.clone(),

            Lambda::Function(param, body) => {
                Lambda::Function(param.clone(), Box::new(body.reduce()))
            }
            Lambda::Application(f, arg) => match f.reduce() {
                Lambda::Function(param, body) => body.substitute(&param, &arg.reduce()).reduce(),
                reduced => Lambda::Application(Box::new(reduced), Box::new(arg.reduce())),
            },
        }
    }
}
