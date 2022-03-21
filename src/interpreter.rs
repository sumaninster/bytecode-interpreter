pub mod interpreter {
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;
    use std::sync::{mpsc};
    use std::sync::mpsc::{channel, Sender, Receiver};
    /*
    Enum for all bytecode instruction set
     */
    #[derive(Debug, PartialEq, Clone)]
    #[allow(dead_code)]
    pub enum ByteCode {
        LoadVar(i64),
        WriteVar(&'static str),
        ReadVar(&'static str),
        Add,
        Multiply,
        Subtract,
        Divide,
        LessThan,
        LessThanEqual,
        GreaterThan,
        GreaterThanEqual,
        Loop(Vec<ByteCode>, Vec<ByteCode>),
        FunctionCall(&'static str, Vec<&'static str>),
        Print(&'static str),
        PrintLn(&'static str),
        Sleep(u64),
        Spawn(Vec<ByteCode>, Vec<&'static str>),
        Mutex(&'static str),
        Channel(&'static str, &'static str),
        SendChannel(&'static str),
        ReceiveChannel(&'static str),
        ReturnValue,
        Return,
    }
    /*
    Byte code supported data types
     */
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[allow(dead_code)]
    pub enum ByteCodeDataTypes {
        Integer64(i64),
        Boolean(bool),
        None,
    }
    /*
    Byte code supported Channel specific data types
     */
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum ByteCodeMpscSyncTypes {
        SendChannel(Sender<i64>),
        ReceiveChannel(Receiver<i64>),
    }
    /*
    Enum for bytecode execution errors
     */
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum ByteCodeError {
        DivisionByZero,
        StackUnderflow,
        UnknownByteCode,
        NoReturnOpcode,
        ChannelNotFound,
    }
    /*
    Program structure to hold bytecode, stack, global variables, parameters and function bytecodes
     */
    #[allow(dead_code)]
    pub struct Program {
        code: Vec<ByteCode>,
        stack: Vec<ByteCodeDataTypes>,
        global_vars: HashMap<&'static str, ByteCodeDataTypes>,
        parameters: HashMap<&'static str, ByteCodeMpscSyncTypes>,
        #[allow(dead_code)]
        functions: HashMap<&'static str, Vec<ByteCode>>,
    }
    /*
    Macro to get value from enum type ByteCodeDataTypes
     */
    macro_rules! value {
        ($var:expr) => {
            match $var {
                ByteCodeDataTypes::Integer64(v) => v,
                ByteCodeDataTypes::Boolean(_) => todo!(),
                ByteCodeDataTypes::None => todo!(),
            }
        };
    }
    /*
    Macro to perform mathematics operation on values from stack - add, subtract, multiply and divide
     */
    macro_rules! operation {
    ($code:expr,$op:tt) => {
            if let (Some(a1), Some(b1)) = ($code.stack.pop(), $code.stack.pop()) {
                let a = value!(a1);
                let b = value!(b1);
                $code.stack.push(ByteCodeDataTypes::Integer64(b $op a));
                None
            } else {
                Some(ByteCodeError::StackUnderflow)
            }
        }
    }
    /*
    Macro to compare values from stack - less than, less than equal, greater than, greater than equal
     */
    macro_rules! compare {
    ($code:expr,$op:tt) => {
            if let (Some(a1), Some(b1)) = ($code.stack.pop(), $code.stack.pop()) {
                let a = value!(a1);
                let b = value!(b1);
                $code.stack.push(ByteCodeDataTypes::Boolean(b $op a));
                None
            } else {
                Some(ByteCodeError::StackUnderflow)
            }
        }
    }
    /*
    Macro to print output to terminal with color code (yellow)
     */
    #[macro_export]
    macro_rules! output {
        ($st:expr) => {
            print!("\x1b[93m{}\x1b[0m", $st)
        };
    }
    /*
    Macro to print output to terminal with color code (yellow) and new line
     */
    #[macro_export]
    macro_rules! output_ln {
        ($st:expr) => {
            println!("\x1b[93m{}\x1b[0m", $st)
        };
    }
    /*
    Move values from one hash map to another hash map
     */
    #[macro_export]
    macro_rules! move_parameters {
        ($mpsc_data:expr,$parameter_vars:expr) => {{
            let mut parameters = HashMap::new();
            for i in 0..$parameter_vars.len() {
                parameters.insert($parameter_vars[i], $mpsc_data.remove($parameter_vars[i]).unwrap());
            }
            parameters
        }};
    }
    /*
    Interpret and execute the byte code
     */
    #[allow(dead_code)]
    pub fn execute(code: Vec<ByteCode>, stack: Vec<ByteCodeDataTypes>,
                   global_vars: HashMap<&'static str, ByteCodeDataTypes>,
                   parameters: HashMap<&'static str, ByteCodeMpscSyncTypes>,
                   functions: HashMap<&'static str, Vec<ByteCode>>) -> (Result<ByteCodeDataTypes, ByteCodeError>, HashMap<&'static str, ByteCodeDataTypes>) {
        let mut program = Program {
            code,
            stack,
            global_vars,
            parameters,
            functions,
        };
        let mut mpsc_data:  HashMap<&'static str, ByteCodeMpscSyncTypes> = HashMap::new();
        for (_index, bc) in program.code.iter().enumerate() {
            let bcr = match bc {
                ByteCode::LoadVar(i) => {
                    program.stack.push(ByteCodeDataTypes::Integer64(*i));
                    None
                },
                ByteCode::WriteVar(var) => {
                    if program.stack.is_empty() {
                        return (Err(ByteCodeError::StackUnderflow), HashMap::new());
                    }
                    program.global_vars.insert(*var, program.stack.pop().unwrap());
                    None
                },
                ByteCode::ReadVar(var) => {
                    if program.global_vars.contains_key(var) {
                        program.stack.push(program.global_vars.get(var).unwrap().clone());
                        None
                    } else {
                        return (Err(ByteCodeError::StackUnderflow), HashMap::new());
                    }
                },
                ByteCode::Add => operation!(program, +),
                ByteCode::Subtract => operation!(program, -),
                ByteCode::Multiply => operation!(program, *),
                ByteCode::Divide => {
                    if program.stack[program.stack.len()-1] == ByteCodeDataTypes::Integer64(0) {
                        Some(ByteCodeError::DivisionByZero)
                    } else {
                        operation!(program, /)
                    }
                },
                ByteCode::LessThan => compare!(program, <),
                ByteCode::LessThanEqual => compare!(program, <=),
                ByteCode::GreaterThan => compare!(program, >),
                ByteCode::GreaterThanEqual => compare!(program, >=),
                ByteCode::Print(var) => {
                    output!(format!("{} = {:?}", var, *program.global_vars.get(var).unwrap()));
                    None
                },
                ByteCode::PrintLn(var) => {
                    output_ln!(format!("{} = {:?}", var, *program.global_vars.get(var).unwrap()));
                    None
                },
                ByteCode::ReturnValue => {
                    return match program.stack.pop() {
                        Some(res) => (Ok(res), HashMap::new()),
                        _ => (Err(ByteCodeError::UnknownByteCode), HashMap::new()),
                    }
                },
                ByteCode::Return => {
                    return (Ok(ByteCodeDataTypes::None), HashMap::new());
                }
                ByteCode::Loop(loop_condition, loop_code) => {
                    loop {
                        let (result, vars) = execute(loop_condition.to_vec(),
                                                     Vec::new(),
                                                     program.global_vars.clone(),
                                                     HashMap::new(),
                                                     program.functions.clone());
                        program.global_vars = vars;
                        match result {
                            Ok(r) => match r {
                                ByteCodeDataTypes::Boolean(b) => {
                                    if b {
                                        let (_result, vars) = execute(loop_code.to_vec(),
                                                                      Vec::new(),
                                                                      program.global_vars.clone(),
                                                                      HashMap::new(),
                                                                      program.functions.clone());
                                        program.global_vars = vars;
                                    } else {
                                        break;
                                    }
                                },
                                _ => {}
                            },
                            Err(e) => {
                                return (Err(e), HashMap::new());
                            }
                        }
                    }
                    None
                },
                ByteCode::FunctionCall(func_code, parameter_vars) => {
                    let parameters = move_parameters!(program.parameters, parameter_vars);
                    let (result, _vars) = execute((*program.functions.get(func_code).unwrap().to_vec()).to_owned(),
                                                  Vec::new(),
                                                  program.global_vars.clone(),
                                                  parameters,
                                                  program.functions.clone());
                    match result {
                        Ok(r) => program.stack.push(r),
                        Err(e) => {
                            return (Err(e), HashMap::new());
                        }
                    }
                    None
                },
                ByteCode::Sleep(duration) => {
                    thread::sleep(Duration::from_secs(*duration));
                    None
                },
                ByteCode::Spawn(spawn_code, parameter_vars) => {
                    let spawn_code_copy = spawn_code.to_vec();
                    let vars_copy = program.global_vars.clone();
                    let functions_copy = program.functions.clone();
                    let (tx, rx) = mpsc::sync_channel(1);
                    let parameters = move_parameters!(mpsc_data, parameter_vars);
                    thread::spawn(move || {
                        let result = execute(spawn_code_copy,
                                             Vec::new(),
                                             vars_copy,
                                             parameters,
                                             functions_copy);
                        tx.send(result).unwrap();
                    });
                    let (result, _vars) = rx.recv().unwrap();
                    match result {
                        Ok(r) => program.stack.push(r),
                        Err(e) => {
                            return (Err(e), HashMap::new());
                        }
                    }
                    None
                },
                ByteCode::Channel(ctx, crx) => {
                    let (tx, rx): (Sender<i64>, Receiver<i64>) = channel();
                    mpsc_data.insert(ctx, ByteCodeMpscSyncTypes::SendChannel(tx));
                    mpsc_data.insert(crx, ByteCodeMpscSyncTypes::ReceiveChannel(rx));
                    //output_ln!(format!("ByteCode::Channel- {:?}", mpsc_data));
                    None
                },
                ByteCode::SendChannel(ctx) => {
                    if program.parameters.contains_key(ctx) {
                        match program.parameters.get(ctx).unwrap() {
                            ByteCodeMpscSyncTypes::SendChannel(tx) => {
                                match program.stack.pop().unwrap() {
                                    ByteCodeDataTypes::Integer64(v) => {
                                        tx.send(v).unwrap()
                                    },
                                    _ => return (Err(ByteCodeError::UnknownByteCode), HashMap::new()),
                                }
                            },
                            _ => return (Err(ByteCodeError::ChannelNotFound), HashMap::new()),
                        }
                    } else {
                        return (Err(ByteCodeError::ChannelNotFound), HashMap::new());
                    }
                    None
                },
                ByteCode::ReceiveChannel(crx) => {
                    if program.parameters.contains_key(crx) {
                        match program.parameters.get(crx).unwrap() {
                            ByteCodeMpscSyncTypes::ReceiveChannel(rx) => {
                                let received = rx.recv().unwrap();
                                program.stack.push(ByteCodeDataTypes::Integer64(received));
                            },
                            _ => return (Err(ByteCodeError::ChannelNotFound), HashMap::new()),
                        }
                    } else {
                        return (Err(ByteCodeError::ChannelNotFound), HashMap::new());
                    }
                    None
                }
                _ => Some(ByteCodeError::StackUnderflow),
            };
            match bcr {
                Some(err) => output_ln!(format!("{:?}", err)),
                _ => {},
            }
        }
        return match program.stack.pop() {
            Some(res) => (Ok(res), program.global_vars),
            _ => (Err(ByteCodeError::UnknownByteCode), program.global_vars),
        };
    }
}
#[cfg(test)]
mod tests {
    use crate::{output_ln, interpreter::interpreter::{execute, ByteCode}};
    use std::collections::HashMap;
    use std::fs;
    use std::fs::metadata;
    use std::ffi::OsStr;
    use crate::parse_code::parse_code::parse_code;
    /*
    Macro to print result or error to terminal
     */
    macro_rules! result {
        ($func:expr, $result:expr) => {
            match $result {
                Ok(res) => {
                    output_ln!(format!("{}, Return Value: {:?}", $func, res));
                    true
                },
                Err(e) => {
                    output_ln!(format!("{} Error: {:?}", $func, e));
                    false
                },
            }
        };
    }
    /*
    Test arithmetic operation
     */
    fn execute_arithmetic_byte_code() -> bool {
        let (result, _) =
            execute(vec![ByteCode::LoadVar(1),
                         ByteCode::WriteVar("x"), ByteCode::LoadVar(2),
                         ByteCode::WriteVar("y"), ByteCode::ReadVar("x"),
                         ByteCode::LoadVar(1), ByteCode::Add, ByteCode::ReadVar("y"),
                         ByteCode::Multiply, ByteCode::ReturnValue],
                    Vec::new(),
                    HashMap::new(),
                    HashMap::new(),
                    HashMap::new());
        result!("execute_arithmetic_byte_code", result)
    }
    /*
    Test compare values from stack
     */
    fn execute_compare_byte_code() -> bool {
        let (result, _) =
            execute(vec![ByteCode::LoadVar(1),
                         ByteCode::WriteVar("x"), ByteCode::LoadVar(2),
                         ByteCode::WriteVar("y"),
                         ByteCode::ReadVar("x"), ByteCode::ReadVar("y"),
                         ByteCode::GreaterThan, ByteCode::ReturnValue],
                    Vec::new(),
                    HashMap::new(),
                    HashMap::new(),
                    HashMap::new());
        result!("execute_compare_byte_code", result)
    }
    /*
    Test function definition and call
     */
    fn execute_function_byte_code() -> bool {
        let mut functions = HashMap::new();
        functions.insert("add", vec![ByteCode::ReadVar("x"), ByteCode::ReadVar("y"), ByteCode::Add, ByteCode::ReturnValue]);
        let (result, _) =
            execute(vec![ByteCode::LoadVar(1), ByteCode::WriteVar("x"),
                         ByteCode::LoadVar(2), ByteCode::WriteVar("y"),
                         ByteCode::FunctionCall("add", vec![]), ByteCode::ReturnValue],
                    Vec::new(),
                    HashMap::new(),
                    HashMap::new(),
                    functions);
        result!("execute_function_byte_code", result)
    }
    /*
    Read the byte codes from code folder and execute one by one
     */
    #[allow(dead_code)]
    pub fn execute_byte_code_from_file(dir: &str, ext: &str) -> bool {
        let paths = fs::read_dir(dir).unwrap();
        let mut success = true;
        for path in paths {
            match path {
                Ok(path) => {
                    let md = metadata(path.path()).unwrap();
                    if md.is_dir() {
                        execute_byte_code_from_file(path.path().as_os_str().to_str().unwrap(), ext);
                    } else if md.is_file() {
                        let s = path.file_name();
                        let file_path = std::path::Path::new(s.as_os_str());
                        if file_path.extension().and_then(OsStr::to_str) == Some(ext) {
                            let file_name = format!("{}", path.path().display());
                            //output_ln!(format!("Content Of File: {}", file_name));
                            let (byte_code, functions) = parse_code(path.path().to_str().unwrap());
                            let (result, _) = execute(byte_code,
                                                      Vec::new(),
                                                      HashMap::new(),
                                                      HashMap::new(),
                                                      functions);
                            if !result!(format!("execute_byte_code_from_file({})", file_name), result) {
                                success = false;
                            }
                        }
                    }
                },
                Err(e) => output_ln!(format!("{:?}", e)),
            }
        }
        success
    }
    #[test]
    fn test_arithmetic() {
        assert_eq!(execute_arithmetic_byte_code(), true);
    }
    #[test]
    fn test_compare() {
        assert_eq!(execute_compare_byte_code(), true);
    }
    #[test]
    fn test_function() {
        assert_eq!(execute_function_byte_code(), true);
    }
    #[test]
    fn test_execute_files() {
        assert_eq!(execute_byte_code_from_file("./code", "bc"), true);
    }
}
