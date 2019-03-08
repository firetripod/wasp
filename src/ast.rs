use wasmly::DataType;

#[derive(Debug)]
pub struct App {
    pub children: Vec<TopLevelOperation>,
}

#[derive(Debug, Clone)]
pub enum TopLevelOperation {
    Comment(String),
    DefineGlobal(Global),
    DefineFunction(FunctionDefinition),
    DefineWasmFunction(WasmFunctionDefinition),
    ExternalFunction(ExternalFunction),
}

#[derive(Debug, Clone)]
pub struct Global {
    pub name: String,
    pub value: GlobalValue,
}

#[derive(Debug, Clone)]
pub enum GlobalValue {
    Number(i32),
    Text(String),
    Data(Vec<GlobalValue>),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub name: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub external_name: Option<String>,
    pub params: Vec<String>,
    pub output: Option<String>,
    pub children: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct WasmFunctionDefinition {
    pub name: String,
    pub external_name: Option<String>,
    pub params: Vec<DataType>,
    pub outputs: Vec<DataType>,
    pub locals: Vec<DataType>,
    pub children: Vec<WasmOperation>,
}

#[derive(Debug, Clone)]
pub struct OperationFunctionCall {
    pub function_name: String,
    pub params: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct OperationLet {
    pub bindings: Vec<(String,Expression)>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    TextLiteral(String),
    Identifier(String),
    Comment(String),
    FunctionCall(OperationFunctionCall),
    Number(i32),
    EmptyList,
    Let(OperationLet)
}

#[derive(Debug, Clone)]
pub enum WasmOperation {
    Comment(String),
    Identifier(String),
    Number(i32),
}
