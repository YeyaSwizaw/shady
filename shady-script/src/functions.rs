use instr::Type;

pub struct Function {
    pub name: &'static str,
    pub args: &'static [Type],
    pub ret: Type,
}

macro_rules! functions {
    ($($name:ident($($arg:ident),*) -> $ret:ident;)+) => {
        static FUNCTIONS: &'static [Function] = &[
            $(Function {
                name: stringify!($name),
                args: &[$(Type::$arg),*],
                ret: Type::$ret
            }),+
        ];
    };
}

functions! {
    sin(Float) -> Float;
    cos(Float) -> Float;
    tan(Float) -> Float;
}

pub fn find_function(name: &str, args: &[Type]) -> Option<Type> {
    for f in FUNCTIONS {
        if f.name == name && f.args == args {
            return Some(f.ret)
        }
    }

    None
}
