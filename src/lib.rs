mod interpreter;

pub struct Ducky {
    interpreter: interpreter::Interpreter,
}
impl Ducky {
    pub fn new() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut interpreter = interpreter::Interpreter::new().unwrap();

        // Add 1 and 2
        interpreter.interpret("1 + 2");
        println!("{:?} - value", interpreter.pop_i32());

        // Clear stack
        interpreter.pop();
        println!("{:?} - default value", interpreter.pop_i32());

        // More complex math
        interpreter.interpret("1 + 2 + 5 * 3");
        println!("{:?} - value", interpreter.pop_i32());

        todo!()
    }
}
