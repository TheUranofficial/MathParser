use std::collections::HashMap;

struct MathParser;
struct MathExpression {
    expression: String,
    vars: HashMap<String, f64>,
}

impl MathExpression {
    fn new_with_vars(expression: String, vars: HashMap<String, f64>) -> Self {
        Self { expression, vars }
    }
}

impl MathParser {
    fn parse(expression: MathExpression) -> f64 {
        let mut result = 0.0;
        let mut operator = ' ';
        let mut last_number = String::new();

        for (_index, symbol) in expression.expression.as_bytes().iter().enumerate() {
            match *symbol as char {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => last_number.push(*symbol as char),
                '+' | '-' | '*' | '/' => {
                    result = Self::process_operation(
                        &operator,
                        &result,
                        &Self::parse_number(&expression.vars, &last_number),
                    );
                    last_number.clear();
                    operator = *symbol as char;
                }
                '.' => {
                    if !last_number.contains(*symbol as char) {
                        last_number.push(*symbol as char);
                    }
                }
                _ => continue,
            }
        }

        if !last_number.is_empty() {
            result = Self::process_operation(
                &operator,
                &result,
                &Self::parse_number(&expression.vars, &last_number),
            );
        }

        result
    }

    fn process_operation(operator: &char, first_value: &f64, second_value: &f64) -> f64 {
        match operator {
            '-' => first_value - second_value,
            '/' => first_value / second_value,
            '*' => first_value * second_value,
            _ => first_value + second_value,
        }
    }

    fn parse_number(vars: &HashMap<String, f64>, number: &String) -> f64 {
        if let Some(value) = vars.get(number) {
            return *value;
        }

        number.parse::<f64>().unwrap()
    }
}

fn main() {
    loop {
        let mut expression = String::new();
        let mut vars = HashMap::new();

        vars.insert(String::from("koch"), 1488.0);
        vars.insert(String::from("golda"), 67.0);

        std::io::stdin().read_line(&mut expression).unwrap();

        println!(
            "{}",
            MathParser::parse(MathExpression::new_with_vars(expression, vars))
        );
    }
}
