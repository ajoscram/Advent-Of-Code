use regex::Regex;
use std::sync::LazyLock;

pub const DAY: &str = "3";
const OP_REGEX: &str = r"(mul|don't|do)\((\d+,\d+|)\)";
const PARAMS_REGEX: &str = r"^(\d+),(\d+)$";

struct Op {
    source: String,
    opcode: String,
    first_param: Option<i32>,
    second_param: Option<i32>
}

pub fn solve(lines: impl Iterator<Item = String>) {
    let ops: Vec<Op> = lines
        .map(|line| get_ops(&line))
        .flatten()
        .collect();

    let mul_total: i32 = ops
        .iter()
        .filter(|op| op.opcode == "mul")
        .map(|mul| mul.first_param.unwrap() * mul.second_param.unwrap())
        .sum();

    println!("The result of adding all multiplications is {}", mul_total);
    
    // 2nd star
    
    let mut enabled = true;
    let mut enabled_total = 0;
    for op in ops.iter() {
        match op.opcode.as_str() {
            "mul" => enabled_total += if enabled { op.first_param.unwrap() * op.second_param.unwrap() } else { 0 },
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => panic!("The following op should not have matched: {}", op.source),
        }
    }
    
    println!("The result of adding all enabled multiplications is {}", enabled_total);
}

fn get_ops(line: &str) -> Vec<Op> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(OP_REGEX).unwrap());
    return RE
        .captures_iter(line)
        .map(|capture| capture.extract())
        .map(|(source, [opcode, params])| get_op(source, opcode, params))
        .collect();
}

fn get_op(source: &str, opcode: &str, params: &str) -> Op {
    let (first_param, second_param) = get_params(params);   
    return Op {
        source: source.to_string(),
        opcode: opcode.to_string(),
        first_param,
        second_param,
    };
}

fn get_params(params: &str) -> (Option<i32>, Option<i32>) {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(PARAMS_REGEX).unwrap());
    return RE
        .captures(params)
        .map(|capture| capture.extract())
        .map(|(_, [first, second])| (first.parse::<i32>().ok(), second.parse::<i32>().ok()))
        .unwrap_or((None, None));
}