use core::hash::Hash;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hasher;

use rig_bytecode::Instruction;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Array(Rc<RefCell<Vec<Value>>>),
    Function(usize), // Index of function in the program
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Null, Value::Null) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            (Value::Array(a), Value::Array(b)) => Rc::ptr_eq(a, b),
            (Value::Function(a), Value::Function(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
            (Value::Boolean(a), Value::Boolean(b)) => Some(a.cmp(b)),
            (Value::Function(a), Value::Function(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Value::Boolean(b) => b.hash(state),
            Value::Number(n) => n.to_bits().hash(state),
            Value::String(s) => s.hash(state),
            Value::Object(o) => Rc::as_ptr(o).hash(state),
            Value::Array(a) => Rc::as_ptr(a).hash(state),
            Value::Function(f) => f.hash(state),
            _ => {}
        }
    }
}

pub struct VM {
    registers: Vec<Value>,
    constants: Vec<Value>,
    program: Vec<Instruction>,
    pc: usize,
    call_stack: Vec<usize>,
    scopes: Vec<Rc<RefCell<HashMap<String, Value>>>>,
    strict_mode: bool,
}

impl VM {
    pub fn new(program: Vec<Instruction>, constants: Vec<Value>) -> Self {
        VM {
            registers: vec![Value::Undefined; 256], // 256 registers
            constants,
            program,
            pc: 0,
            call_stack: Vec::new(),
            scopes: vec![Rc::new(RefCell::new(HashMap::new()))], // Global scope
            strict_mode: false,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            let instruction = self.program[self.pc].clone();
            self.execute(instruction);
            self.pc += 1;
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LoadConst { reg, const_idx } => {
                self.registers[reg as usize] = self.constants[const_idx as usize].clone();
            }
            Instruction::LoadUndefined { reg } => {
                self.registers[reg as usize] = Value::Undefined;
            }
            Instruction::LoadNull { reg } => {
                self.registers[reg as usize] = Value::Null;
            }
            Instruction::LoadBool { reg, value } => {
                self.registers[reg as usize] = Value::Boolean(value);
            }
            Instruction::Move { dst, src } => {
                self.registers[dst as usize] = self.registers[src as usize].clone();
            }
            Instruction::Add { dst, a, b } => {
                let result = self.binary_op(a, b, |x, y| x + y);
                self.registers[dst as usize] = result;
            }
            Instruction::Sub { dst, a, b } => {
                let result = self.binary_op(a, b, |x, y| x - y);
                self.registers[dst as usize] = result;
            }
            Instruction::Mul { dst, a, b } => {
                let result = self.binary_op(a, b, |x, y| x * y);
                self.registers[dst as usize] = result;
            }
            Instruction::Div { dst, a, b } => {
                let result = self.binary_op(a, b, |x, y| x / y);
                self.registers[dst as usize] = result;
            }
            Instruction::Mod { dst, a, b } => {
                let result = self.binary_op(a, b, |x, y| x % y);
                self.registers[dst as usize] = result;
            }
            Instruction::Pow { dst, a, b } => {
                let result = self.binary_op(a, b, |x, y| x.powf(y));
                self.registers[dst as usize] = result;
            }
            Instruction::Neg { dst, a } => {
                if let Value::Number(x) = self.registers[a as usize] {
                    self.registers[dst as usize] = Value::Number(-x);
                } else {
                    panic!("Invalid type for negation");
                }
            }
            Instruction::Eq { a, b } => {
                let result = self.compare(a, b, |x, y| x == y);
                self.registers[0] = Value::Boolean(result); // Store result in register 0
            }
            Instruction::Lt { a, b } => {
                let result = self.compare(a, b, |x, y| x < y);
                self.registers[0] = Value::Boolean(result);
            }
            Instruction::Le { a, b } => {
                let result = self.compare(a, b, |x, y| x <= y);
                self.registers[0] = Value::Boolean(result);
            }
            Instruction::Jmp { offset } => {
                self.pc = (self.pc as i32 + offset) as usize;
            }
            Instruction::JmpIf { cond, offset } => {
                if let Value::Boolean(true) = self.registers[cond as usize] {
                    self.pc = (self.pc as i32 + offset) as usize;
                }
            }
            Instruction::Call {
                func_reg,
                arg_count,
            } => {
                if let Value::Function(func_idx) = self.registers[func_reg as usize] {
                    self.call_stack.push(self.pc);
                    self.pc = func_idx;
                    // Create new scope for function
                    self.scopes.push(Rc::new(RefCell::new(HashMap::new())));
                } else {
                    panic!("Invalid function call");
                }
            }
            Instruction::Return { start_reg, count } => {
                if let Some(return_addr) = self.call_stack.pop() {
                    self.pc = return_addr;
                    self.scopes.pop(); // Remove function scope
                } else {
                    panic!("Return without call");
                }
            }
            Instruction::NewObject { reg } => {
                self.registers[reg as usize] = Value::Object(Rc::new(RefCell::new(HashMap::new())));
            }
            Instruction::GetProp { dst, obj, key } => {
                if let (Value::Object(obj), Value::String(key)) =
                    (&self.registers[obj as usize], &self.registers[key as usize])
                {
                    let obj_ref = obj.borrow();
                    let value = obj_ref.get(key).unwrap_or(&Value::Undefined).clone();
                    drop(obj_ref);
                    self.registers[dst as usize] = value;
                } else {
                    panic!("Invalid GetProp operation");
                }
            }
            Instruction::SetProp { obj, key, value } => {
                if let (Value::Object(obj), Value::String(key)) =
                    (&self.registers[obj as usize], &self.registers[key as usize])
                {
                    let mut obj_ref = obj.borrow_mut();
                    obj_ref.insert(key.clone(), self.registers[value as usize].clone());
                } else {
                    panic!("Invalid SetProp operation");
                }
            }
            Instruction::Closure { reg, func_idx } => {
                self.registers[reg as usize] = Value::Function(func_idx as usize);
            }
            Instruction::GetScope { dst, var_idx } => {
                // Simplified scope handling
                if let Some(scope) = self.scopes.last() {
                    let scope_ref = scope.borrow();
                    if let Some(value) = scope_ref.get(&format!("var_{}", var_idx)) {
                        self.registers[dst as usize] = value.clone();
                    } else {
                        self.registers[dst as usize] = Value::Undefined;
                    }
                } else {
                    panic!("No active scope");
                }
            }
            Instruction::SetScope { var_idx, src } => {
                if let Some(scope) = self.scopes.last() {
                    let mut scope_ref = scope.borrow_mut();
                    scope_ref.insert(
                        format!("var_{}", var_idx),
                        self.registers[src as usize].clone(),
                    );
                } else {
                    panic!("No active scope");
                }
            }
            Instruction::NewArray { reg } => {
                self.registers[reg as usize] = Value::Array(Rc::new(RefCell::new(Vec::new())));
            }
            Instruction::GetElem { dst, array, index } => {
                if let (Value::Array(arr), Value::Number(fidx)) = (
                    &self.registers[array as usize],
                    &self.registers[index as usize],
                ) {
                    let idx = fidx.floor() as usize;
                    let arr_ref = arr.borrow();
                    let value = arr_ref.get(idx).unwrap_or(&Value::Undefined).clone();
                    drop(arr_ref);
                    self.registers[dst as usize] = value
                } else {
                    panic!("Invalid GetElem operation");
                }
            }
            Instruction::SetElem {
                array,
                index,
                value,
            } => {
                if let (Value::Array(arr), Value::Number(fidx)) = (
                    &self.registers[array as usize],
                    &self.registers[index as usize],
                ) {
                    let idx = fidx.floor() as usize;
                    let mut arr_ref = arr.borrow_mut();
                    if (idx) >= arr_ref.len() {
                        arr_ref.resize(idx + 1, Value::Undefined);
                    }
                    arr_ref[idx] = self.registers[value as usize].clone();
                } else {
                    panic!("Invalid SetElem operation");
                }
            }
            Instruction::TypeOf { dst, src } => {
                self.registers[dst as usize] = Value::String(match self.registers[src as usize] {
                    Value::Undefined => "undefined".to_string(),
                    Value::Null => "object".to_string(),
                    Value::Boolean(_) => "boolean".to_string(),
                    Value::Number(_) => "number".to_string(),
                    Value::String(_) => "string".to_string(),
                    Value::Object(_) => "object".to_string(),
                    Value::Array(_) => "object".to_string(),
                    Value::Function(_) => "function".to_string(),
                });
            }
            Instruction::InstanceOf { dst, obj, ctor } => {
                // Simplified instanceof (just checks if obj is of type ctor)
                self.registers[dst as usize] = Value::Boolean(matches!(
                    (
                        self.registers[obj as usize].clone(),
                        self.registers[ctor as usize].clone()
                    ),
                    (Value::Object(_), Value::Function(_)) | (Value::Array(_), Value::Function(_))
                ));
            }
            Instruction::DeclareFunc {
                reg,
                name_idx,
                param_count,
            } => {
                // For simplicity, we're just storing the function in a register
                self.registers[reg as usize] = Value::Function(self.pc);
                // The actual function body would follow this instruction
            }
            Instruction::DeclareVar { name_idx } => {
                if let Some(scope) = self.scopes.last() {
                    let mut scope_ref = scope.borrow_mut();
                    scope_ref.insert(format!("var_{}", name_idx), Value::Undefined);
                } else {
                    panic!("No active scope");
                }
            }
            Instruction::UseStrict => {
                self.strict_mode = true;
            }
        }
    }

    fn binary_op<F>(&self, a: u8, b: u8, op: F) -> Value
    where
        F: Fn(f64, f64) -> f64,
    {
        match (&self.registers[a as usize], &self.registers[b as usize]) {
            (Value::Number(x), Value::Number(y)) => Value::Number(op(*x, *y)),
            _ => panic!("Invalid types for binary operation"),
        }
    }

    fn compare<F>(&self, a: u8, b: u8, op: F) -> bool
    where
        F: Fn(&Value, &Value) -> bool,
    {
        op(&self.registers[a as usize], &self.registers[b as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::Move { dst: 1, src: 0 },
        ];
        let constants = vec![Value::Number(10.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[1], Value::Number(10.0));
    }

    #[test]
    fn test_add() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Add { dst: 2, a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(5.0), Value::Number(7.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[2], Value::Number(12.0));
    }

    #[test]
    fn test_sub() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Sub { dst: 2, a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(10.0), Value::Number(3.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[2], Value::Number(7.0));
    }

    #[test]
    fn test_mul() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Mul { dst: 2, a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(4.0), Value::Number(3.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[2], Value::Number(12.0));
    }

    #[test]
    fn test_div() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Div { dst: 2, a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(8.0), Value::Number(2.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[2], Value::Number(4.0));
    }

    #[test]
    fn test_mod() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Mod { dst: 2, a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(10.0), Value::Number(3.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[2], Value::Number(1.0));
    }

    #[test]
    fn test_neg() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::Neg { dst: 1, a: 0 },
        ];
        let constants = vec![Value::Number(5.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[1], Value::Number(-5.0));
    }

    #[test]
    fn test_eq() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Eq { a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(5.0), Value::Number(5.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[0], Value::Boolean(true));
    }

    #[test]
    fn test_lt() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Lt { a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(3.0), Value::Number(5.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[0], Value::Boolean(true));
    }

    #[test]
    fn test_le() {
        let program = vec![
            Instruction::LoadConst {
                reg: 0,
                const_idx: 0,
            },
            Instruction::LoadConst {
                reg: 1,
                const_idx: 1,
            },
            Instruction::Le { a: 0, b: 1 },
        ];
        let constants = vec![Value::Number(5.0), Value::Number(5.0)];

        let mut vm = VM::new(program, constants);
        vm.run();

        assert_eq!(vm.registers[0], Value::Boolean(true));
    }
}
