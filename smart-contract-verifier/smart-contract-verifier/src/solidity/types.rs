use crate::{
    compiler,
    verifier::{self, LocalBytecodeParts},
    MatchType,
};
use blockscout_display_bytes::Bytes as DisplayBytes;
use ethers_solc::CompilerOutput;

#[derive(Clone, Debug)]
pub struct Success {
    pub compiler_input: ethers_solc::CompilerInput,
    pub compiler_output: CompilerOutput,
    pub compiler_version: compiler::Version,
    pub file_path: String,
    pub contract_name: String,
    pub abi: Option<serde_json::Value>,
    pub constructor_args: Option<DisplayBytes>,
    pub local_bytecode_parts: LocalBytecodeParts,
    pub match_type: MatchType,
}

impl From<(ethers_solc::CompilerInput, verifier::Success)> for Success {
    fn from((compiler_input, success): (ethers_solc::CompilerInput, verifier::Success)) -> Self {
        Self {
            compiler_input,
            compiler_output: success.compiler_output,
            compiler_version: success.compiler_version,
            file_path: success.file_path,
            contract_name: success.contract_name,
            abi: success.abi,
            constructor_args: success.constructor_args,
            local_bytecode_parts: success.local_bytecode_parts,
            match_type: success.match_type,
        }
    }
}
