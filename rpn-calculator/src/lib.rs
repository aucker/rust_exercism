
#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    /*unimplemented!(
		"Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
		inputs,
	);*/
    use crate::CalculatorInput::{Value, Add, Subtract, Multiply, Divide};

    let v:Vec<i32> = vec![];
    let mut result = inputs.iter().fold(v, |mut stack, input| {
        if let Some(new) = match input {
            Add => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b + a))),
            Subtract => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b - a))),
            Multiply => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b * a))),
            Divide => stack
                .pop()
                .and_then(|a| stack.pop().and_then(|b| Some(b / a))),
            Value(value) => Some(*value),
        } {
            stack.push(new);
        }
        stack
    });
    result
        .pop()
        .and_then(|x| if result.is_empty() {Some(x)} else { None })

}
