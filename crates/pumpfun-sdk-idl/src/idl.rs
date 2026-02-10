use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Idl {
    pub version: String,
    pub name: String,
    pub instructions: Vec<IdlInstruction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdlInstruction {
    pub name: String,
    pub accounts: Vec<IdlAccount>,
    pub args: Vec<IdlArg>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdlAccount {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdlArg {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}
