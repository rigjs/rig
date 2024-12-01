/// Represents a set of instructions for a virtual machine.
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Loads a constant into a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    /// - `const_idx`: The constant index (32 bits).
    LoadConst { reg: u8, const_idx: u32 },

    /// Loads an undefined value into a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    LoadUndefined { reg: u8 },

    /// Loads a null value into a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    LoadNull { reg: u8 },

    /// Loads a boolean value into a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    /// - `value`: The boolean value (1 bit).
    LoadBool { reg: u8, value: bool },

    /// Moves a value between registers.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `src`: The source register index (8 bits).
    Move { dst: u8, src: u8 },

    /// Performs addition on two registers and stores the result.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Add { dst: u8, a: u8, b: u8 },

    /// Performs subtraction on two registers and stores the result.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Sub { dst: u8, a: u8, b: u8 },

    /// Performs multiplication on two registers and stores the result.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Mul { dst: u8, a: u8, b: u8 },

    /// Performs division on two registers and stores the result.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Div { dst: u8, a: u8, b: u8 },

    /// Performs modulus operation on two registers and stores the result.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Mod { dst: u8, a: u8, b: u8 },

    /// Performs exponentiation on two registers and stores the result.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The base operand register index (8 bits).
    /// - `b`: The exponent operand register index (8 bits).
    Pow { dst: u8, a: u8, b: u8 },

    /// Negates a value in a register.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `a`: The operand register index (8 bits).
    Neg { dst: u8, a: u8 },

    /// Compares two registers for equality.
    ///
    /// # Parameters
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Eq { a: u8, b: u8 },

    /// Compares if the value in the first register is less than the second.
    ///
    /// # Parameters
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Lt { a: u8, b: u8 },

    /// Compares if the value in the first register is less than or equal to the second.
    ///
    /// # Parameters
    /// - `a`: The first operand register index (8 bits).
    /// - `b`: The second operand register index (8 bits).
    Le { a: u8, b: u8 },

    /// Performs an unconditional jump.
    ///
    /// # Parameters
    /// - `offset`: The jump offset (32 bits).
    Jmp { offset: i32 },

    /// Performs a conditional jump based on a register's value.
    ///
    /// # Parameters
    /// - `cond`: The condition register index (8 bits).
    /// - `offset`: The jump offset (32 bits).
    JmpIf { cond: u8, offset: i32 },

    /// Calls a function with a specified number of arguments.
    ///
    /// # Parameters
    /// - `func_reg`: The function register index (8 bits).
    /// - `arg_count`: The number of arguments (8 bits).
    Call { func_reg: u8, arg_count: u8 },

    /// Returns from a function with specified values.
    ///
    /// # Parameters
    /// - `start_reg`: The start register index for return values (8 bits).
    /// - `count`: The number of return values (8 bits).
    Return { start_reg: u8, count: u8 },

    /// Creates a new object in a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    NewObject { reg: u8 },

    /// Gets a property from an object and stores it in a register.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `obj`: The object register index (8 bits).
    /// - `key`: The key register index (8 bits).
    GetProp { dst: u8, obj: u8, key: u8 },

    /// Sets a property on an object.
    ///
    /// # Parameters
    /// - `obj`: The object register index (8 bits).
    /// - `key`: The key register index (8 bits).
    /// - `value`: The value register index (8 bits).
    SetProp { obj: u8, key: u8, value: u8 },

    /// Creates a closure from a function index and stores it in a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    /// - `func_idx`: The function index (32 bits).
    Closure { reg: u8, func_idx: u32 },

    /// Retrieves a variable from the scope and stores it in a register.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `var_idx`: The variable index (32 bits).
    GetScope { dst: u8, var_idx: u32 },

    /// Sets a variable in the scope from a register.
    ///
    /// # Parameters
    /// - `var_idx`: The variable index (32 bits).
    /// - `src`: The source register index (8 bits).
    SetScope { var_idx: u32, src: u8 },

    /// Creates a new array in a register.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    NewArray { reg: u8 },

    /// Gets an element from an array and stores it in a register.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `array`: The array register index (8 bits).
    /// - `index`: The index register index (8 bits).
    GetElem { dst: u8, array: u8, index: u8 },

    /// Sets an element in an array.
    ///
    /// # Parameters
    /// - `array`: The array register index (8 bits).
    /// - `index`: The index register index (8 bits).
    /// - `value`: The value register index (8 bits).
    SetElem { array: u8, index: u8, value: u8 },

    /// Determines the type of a value in a register.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `src`: The source register index (8 bits).
    TypeOf { dst: u8, src: u8 },

    /// Checks if an object is an instance of a constructor.
    ///
    /// # Parameters
    /// - `dst`: The destination register index (8 bits).
    /// - `obj`: The object register index (8 bits).
    /// - `ctor`: The constructor register index (8 bits).
    InstanceOf { dst: u8, obj: u8, ctor: u8 },

    /// Declares a function with a specified number of parameters.
    ///
    /// # Parameters
    /// - `reg`: The register index (8 bits).
    /// - `name_idx`: The function name index (32 bits).
    /// - `param_count`: The number of parameters (8 bits).
    DeclareFunc { reg: u8, name_idx: u32, param_count: u8 },

    /// Declares a variable by its name index.
    ///
    /// # Parameters
    /// - `name_idx`: The variable name index (32 bits).
    DeclareVar { name_idx: u32 },

    /// Enables strict mode.
    UseStrict,
}
