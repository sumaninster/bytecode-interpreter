pub mod parse_code {
    use crate::interpreter::interpreter::ByteCode;
    use std::collections::HashMap;
    use std::fs;
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    enum CodeType {
        Program,
        LoopCondition,
        LoopCode,
        Function,
        Spawn,
    }
    /*
    Function to parse the byte code from file and convert it to vector
     */
    macro_rules! get_var {
        ($var2:expr) => {
            {
                let var1 = String::from($var2);
                let var: &'static str = Box::leak(var1.into_boxed_str());
                var
            }
        };
    }
    macro_rules! copy_vars {
        ($token:expr, $index:expr) => {
            {
                let mut parameters = vec![];
                for i in $index..$token.len() {
                    parameters.push(get_var!($token[i]));
                }
                parameters
            }
        };
    }
    #[allow(dead_code)]
    pub fn parse_code(path: &str) -> (Vec<ByteCode>, HashMap<&'static str, Vec<ByteCode>>) {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let mut byte_code = vec![];
        let mut functions: HashMap<&'static str, Vec<ByteCode>> = HashMap::new();
        let mut loop_condition = vec![];
        let mut loop_code = vec![];
        let mut function_code = vec![];
        let mut function_name: &'static str = "";
        let mut spawn_code = vec![];
        let mut spawn_vars = vec![];
        let mut tmp_code = vec![];
        let mut code_type = CodeType::Program;
        let mut code_type_stack: Vec<CodeType> = vec![];
        let lines: Vec<String> = data.lines().map(String::from).collect();
        let mut token: Vec<&str>;
        let mut code_split;
        for code in lines {
            //println!("{}", code);
            code_split = Box::leak(code.into_boxed_str());
            token = code_split.split_whitespace().collect();
            match token[0] {
                "LOAD_VAL" => tmp_code.push(ByteCode::LoadVar(token[1].parse::<i64>().unwrap())),
                "WRITE_VAR" => tmp_code.push(ByteCode::WriteVar(get_var!(token[1]))),
                "READ_VAR" => tmp_code.push(ByteCode::ReadVar(get_var!(token[1]))),
                "ADD" => tmp_code.push(ByteCode::Add),
                "SUBTRACT" => tmp_code.push(ByteCode::Subtract),
                "MULTIPLY" => tmp_code.push(ByteCode::Multiply),
                "DIVIDE" => tmp_code.push(ByteCode::Divide),
                "LESS_THAN" => tmp_code.push(ByteCode::LessThan),
                "LESS_THAN_EQUAL" => tmp_code.push(ByteCode::LessThanEqual),
                "GREATER_THAN" => tmp_code.push(ByteCode::GreaterThan),
                "GREATER_THAN_EQUAL" => tmp_code.push(ByteCode::GreaterThanEqual),
                "RETURN" =>tmp_code.push(ByteCode::Return),
                "RETURN_VALUE"=> tmp_code.push(ByteCode::ReturnValue),
                "PRINT" => tmp_code.push(ByteCode::Print(get_var!(token[1]))),
                "PRINT_LN" => tmp_code.push(ByteCode::PrintLn(get_var!(token[1]))),
                "SLEEP" => tmp_code.push(ByteCode::Sleep(token[1].parse::<u64>().unwrap())),
                "LOOP" => {
                    code_type_stack.push(code_type.clone());
                    match code_type {
                        CodeType::Program => byte_code.append(&mut tmp_code.to_vec()),
                        CodeType::LoopCondition => loop_condition.append(&mut tmp_code.to_vec()),
                        CodeType::LoopCode => loop_code.append(&mut tmp_code.to_vec()),
                        CodeType::Function => function_code.append(&mut tmp_code.to_vec()),
                        CodeType::Spawn => spawn_code.append(&mut tmp_code.to_vec()),
                    }
                    tmp_code.clear();
                    code_type = CodeType::LoopCondition;
                }
                "LOOP_START" => {
                    loop_condition.append(&mut tmp_code.to_vec());
                    tmp_code.clear();
                    code_type = CodeType::LoopCode;
                }
                "LOOP_END" => {
                    loop_code.append(&mut tmp_code.to_vec());
                    tmp_code.clear();
                    code_type = code_type_stack.pop().unwrap();
                    tmp_code.push(ByteCode::Loop(loop_condition.to_vec(), loop_code.to_vec()));
                    loop_condition.clear();
                    loop_code.clear();
                }
                "FUNC" => {
                    code_type_stack.push(code_type.clone());
                    code_type = CodeType::Function;
                    byte_code.append(&mut tmp_code.to_vec());
                    tmp_code.clear();
                    function_code.clear();
                    function_name = get_var!(token[1]);
                }
                "FUNC_END" => {
                    code_type = code_type_stack.pop().unwrap();
                    function_code.append(&mut tmp_code.to_vec());
                    tmp_code.clear();
                    functions.insert(function_name, function_code.to_vec());
                    function_code.clear();
                    function_name = "";
                }
                "FUNC_CALL" =>
                    tmp_code.push(ByteCode::FunctionCall(get_var!(token[1]), copy_vars!(token, 2))),
                "SPAWN" => {
                    code_type_stack.push(code_type.clone());
                    code_type = CodeType::Spawn;
                    byte_code.append(&mut tmp_code.to_vec());
                    tmp_code.clear();
                    spawn_code.clear();
                    spawn_vars.append(&mut copy_vars!(token, 1));
                },
                "SPAWN_END" => {
                    code_type = code_type_stack.pop().unwrap();
                    spawn_code.append(&mut tmp_code.to_vec());
                    tmp_code.clear();
                    tmp_code.push(ByteCode::Spawn(spawn_code.to_vec(), spawn_vars.to_vec()));
                    spawn_code.clear();
                    spawn_vars.clear();
                },
                "CHANNEL" => tmp_code.push(ByteCode::Channel(get_var!(token[1]), get_var!(token[2]))),
                "SEND_CHANNEL" => tmp_code.push(ByteCode::SendChannel(get_var!(token[1]))),
                "RECEIVE_CHANNEL" => tmp_code.push(ByteCode::ReceiveChannel(get_var!(token[1]))),
                _ => {}
            }
        }
        byte_code.append(&mut tmp_code.to_vec());
        tmp_code.clear();
        (byte_code, functions)
    }
}