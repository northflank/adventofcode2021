use std::fmt::{Debug, Formatter};
use std::fs;
use std::str::FromStr;

fn main() {
    let program = fs::read_to_string("input-day-24.txt").expect("Inputs not found");
    let program = program.parse::<Program>().expect("Invalid program: ");

    let mut alu_symbolic = SymbolicALU::new();

    let relations = alu_symbolic.simulate(
        &program.instructions,
        // All `eql _ w` comparisons must evaluate to 0 in order for z to remain zero
        &vec![1, 1, 1, 1, 1, 1, 1],
        &vec![],
    );

    assert_eq!(alu_symbolic.z.val, 0);
    assert_eq!(alu_symbolic.z.inp.iter().sum::<i64>(), 0);

    println!("Model No Requirements:");
    for (p, q) in &relations {
        println!("{:?} == {:?}", p, q);
    }

    println!();

    let mut min = vec![-1; 14];
    let mut max = vec![-1; 14];
    for (p, q) in relations {
        let pi = p.inp.iter().position(|a| *a != 0).unwrap();
        let qi = q.inp.iter().position(|a| *a != 0).unwrap();

        if p.val < 0 {
            max[pi] = 9;
            max[qi] = 9 + p.val;

            min[pi] = 1 - p.val;
            min[qi] = 1;
        } else {
            max[pi] = 9 - p.val;
            max[qi] = 9;

            min[pi] = 1;
            min[qi] = 1 + p.val;
        }
    }

    // Sanity checks
    assert_eq!(ALU::new().execute(&program.instructions, &max).z, 0);
    assert_eq!(ALU::new().execute(&program.instructions, &min).z, 0);

    println!("Task 1:");
    println!(
        "{}",
        max.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("")
    );

    println!();
    println!("Task 2:");
    println!(
        "{}",
        min.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic ALU
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone, Debug)]
pub enum Variable {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug)]
pub enum Argument {
    Var(Variable),
    Val(i64),
}

#[derive(Debug)]
pub enum Instruction {
    Inp(Variable),
    Add(Variable, Argument),
    Mul(Variable, Argument),
    Div(Variable, Argument),
    Mod(Variable, Argument),
    Eql(Variable, Argument),
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl ALU {
    fn new() -> ALU {
        ALU {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    fn execute<'a, P, I>(&mut self, program: P, input: I) -> &Self
    where
        P: IntoIterator<Item = &'a Instruction>,
        I: IntoIterator<Item = &'a i64>,
    {
        let mut input = input.into_iter();
        for x in program.into_iter() {
            match x {
                Instruction::Inp(a) => self.set(a, *input.next().expect("input exhausted")),
                Instruction::Add(a, b) => self.exec_op(a, b, |a, b| a + b),
                Instruction::Mul(a, b) => self.exec_op(a, b, |a, b| a * b),
                Instruction::Div(a, b) => self.exec_op(a, b, |a, b| a / b),
                Instruction::Mod(a, b) => self.exec_op(a, b, |a, b| a % b),
                Instruction::Eql(a, b) => self.exec_op(a, b, |a, b| if a == b { 1 } else { 0 }),
            }
        }
        self
    }

    fn exec_op<F: FnOnce(i64, i64) -> i64>(&mut self, a: &Variable, b: &Argument, f: F) {
        self.set(a, f(self.get_val(&Argument::Var(*a)), self.get_val(b)))
    }

    fn get_val(&self, a: &Argument) -> i64 {
        match a {
            Argument::Val(a) => *a,
            Argument::Var(Variable::X) => self.x,
            Argument::Var(Variable::Y) => self.y,
            Argument::Var(Variable::Z) => self.z,
            Argument::Var(Variable::W) => self.w,
        }
    }

    fn set(&mut self, a: &Variable, val: i64) {
        match a {
            Variable::X => self.x = val,
            Variable::Y => self.y = val,
            Variable::Z => self.z = val,
            Variable::W => self.w = val,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Minimal Symbolic ALU for Analysis of MONAD
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct SymbolicVar {
    // assumes linear relations between digits
    pub inp: Vec<i64>,
    pub val: i64,
}

impl SymbolicVar {
    fn zero() -> SymbolicVar {
        SymbolicVar {
            inp: Vec::new(),
            val: 0,
        }
    }

    fn val(val: i64) -> SymbolicVar {
        SymbolicVar {
            inp: Vec::new(),
            val,
        }
    }

    fn inp(inp: usize) -> SymbolicVar {
        SymbolicVar {
            inp: vec![0; inp].drain(..).chain(std::iter::once(1)).collect(),
            val: 0,
        }
    }

    // Assumes inputs from 1 to 9
    fn min(&self) -> i64 {
        self.inp.iter().map(|a| a * 1).sum::<i64>() + self.val
    }

    fn max(&self) -> i64 {
        self.inp.iter().map(|a| a * 9).sum::<i64>() + self.val
    }
}

impl Debug for SymbolicVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let terms: Vec<_> = self
            .inp
            .iter()
            .enumerate()
            .filter(|(_, b)| **b != 0)
            .map(|(i, a)| {
                if *a == 1 {
                    format!("#{}", i)
                } else {
                    format!("({} * #{})", a, i)
                }
            })
            .chain(std::iter::once(self.val.to_string()))
            .collect();

        write!(f, "{}", terms.join(" + "))
    }
}

impl std::ops::Add for SymbolicVar {
    type Output = SymbolicVar;

    fn add(self, rhs: Self) -> Self::Output {
        SymbolicVar {
            inp: (0..(self.inp.len().max(rhs.inp.len())))
                .map(|i| {
                    self.inp.get(i).copied().unwrap_or(0) + rhs.inp.get(i).copied().unwrap_or(0)
                })
                .collect(),
            val: self.val + rhs.val,
        }
    }
}

impl std::ops::Mul for SymbolicVar {
    type Output = SymbolicVar;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(rhs.inp.iter().sum::<i64>(), 0);
        SymbolicVar {
            inp: self.inp.iter().map(|a| a * rhs.val).collect(),
            val: self.val * rhs.val,
        }
    }
}

impl std::ops::Div for SymbolicVar {
    type Output = SymbolicVar;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.inp.is_empty());
        SymbolicVar {
            inp: self.inp.iter().map(|a| a / rhs.val).collect(),
            val: self.val / rhs.val,
        }
    }
}

impl std::ops::Rem for SymbolicVar {
    type Output = SymbolicVar;

    fn rem(self, rhs: Self) -> Self::Output {
        assert!(rhs.inp.is_empty());
        let r = SymbolicVar {
            inp: self.inp.iter().map(|a| a % rhs.val).collect(),
            val: self.val % rhs.val,
        };
        // println!("MOD ({:?}) % {} = {:?}", self, rhs.val, r);
        r
    }
}

#[derive(Debug)]
pub struct SymbolicALU {
    pub x: SymbolicVar,
    pub y: SymbolicVar,
    pub z: SymbolicVar,
    pub w: SymbolicVar,
}

impl SymbolicALU {
    pub fn new() -> SymbolicALU {
        SymbolicALU {
            x: SymbolicVar::zero(),
            y: SymbolicVar::zero(),
            z: SymbolicVar::zero(),
            w: SymbolicVar::zero(),
        }
    }

    pub fn simulate<'a, P, I, E>(
        &mut self,
        program: P,
        eql_hints: E,
        parial_input: I,
    ) -> Vec<(SymbolicVar, SymbolicVar)>
    where
        P: IntoIterator<Item = &'a Instruction>,
        I: IntoIterator<Item = &'a i64>,
        E: IntoIterator<Item = &'a i64>,
    {
        let mut relations = Vec::new();
        let mut input = parial_input
            .into_iter()
            .map(|a| SymbolicVar::val(*a))
            .chain(
                std::iter::successors(Some(0usize), |a| Some(a + 1))
                    .into_iter()
                    .map(|i| SymbolicVar::inp(i)),
            );

        let mut eq = eql_hints.into_iter();
        for x in program.into_iter() {
            match x {
                Instruction::Inp(a) => self.set(a, input.next().unwrap()),
                Instruction::Add(a, b) => self.set(a, self.get_var(a) + self.get_arg(b)),
                Instruction::Mul(a, b) => self.set(a, self.get_var(a) * self.get_arg(b)),
                Instruction::Div(a, b) => self.set(a, self.get_var(a) / self.get_arg(b)),
                Instruction::Mod(a, b) => self.set(a, self.get_var(a) % self.get_arg(b)),
                Instruction::Eql(a, b) => {
                    let p = self.get_var(a);
                    let q = self.get_arg(b);

                    let pq_range = p.min().max(q.min())..=p.max().min(q.max());
                    let overlap = pq_range.end() - pq_range.start() + 1;

                    // Equality checks which involve w and aren't unsatisfiable
                    if q.inp.iter().sum::<i64>() > 0 && overlap > 0 {
                        relations.push((p, q));
                    }

                    let c = if overlap <= 0 {
                        0
                    } else if overlap == 1 {
                        1
                    } else {
                        *eq.next().unwrap()
                    };

                    self.set(a, SymbolicVar::val(c))
                }
                _ => unreachable!(),
            }
        }

        relations
    }

    fn get_var(&self, a: &Variable) -> SymbolicVar {
        match a {
            Variable::X => self.x.clone(),
            Variable::Y => self.y.clone(),
            Variable::Z => self.z.clone(),
            Variable::W => self.w.clone(),
        }
    }

    fn get_arg(&self, a: &Argument) -> SymbolicVar {
        match a {
            Argument::Val(a) => SymbolicVar::val(*a),
            Argument::Var(v) => self.get_var(v),
        }
    }

    fn set(&mut self, a: &Variable, val: SymbolicVar) {
        match a {
            Variable::X => self.x = val,
            Variable::Y => self.y = val,
            Variable::Z => self.z = val,
            Variable::W => self.w = val,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Parser Implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FromStr for Variable {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            "w" => Variable::W,
            other => Err(format!("Invalid variable {}", other))?,
        };

        Ok(result)
    }
}

impl FromStr for Argument {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(var) = s.parse::<Variable>() {
            return Ok(Argument::Var(var));
        }

        match s.parse::<i64>() {
            Ok(val) => Ok(Argument::Val(val)),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(" ").collect();

        match tokens[0] {
            "inp" => Ok(Instruction::Inp(tokens[1].parse()?)),
            "add" => Ok(Instruction::Add(tokens[1].parse()?, tokens[2].parse()?)),
            "mul" => Ok(Instruction::Mul(tokens[1].parse()?, tokens[2].parse()?)),
            "div" => Ok(Instruction::Div(tokens[1].parse()?, tokens[2].parse()?)),
            "mod" => Ok(Instruction::Mod(tokens[1].parse()?, tokens[2].parse()?)),
            "eql" => Ok(Instruction::Eql(tokens[1].parse()?, tokens[2].parse()?)),
            _ => Err(format!("Unknown instruction {}", tokens[0])),
        }
    }
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();

        for l in s.lines() {
            if l.trim().is_empty() {
                continue;
            }

            if l.starts_with("EXIT") {
                break;
            }

            instructions.push(l.parse()?)
        }

        Ok(Program { instructions })
    }
}
