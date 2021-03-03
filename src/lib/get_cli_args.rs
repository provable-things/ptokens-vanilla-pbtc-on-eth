use crate::lib::{errors::AppError, types::Result, usage_info::USAGE_INFO};
use docopt::Docopt;
use std::{fs::read_to_string, path::Path};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub arg_vOut: u32,
    pub flag_fee: u64,
    pub flag_confs: u64,
    pub flag_nonce: u64,
    pub flag_chainId: u8, // NOTE: ETH network!
    pub flag_file: String,
    pub flag_gasPrice: u64,
    pub flag_network: String, // NOTE: BTC network!
    pub flag_version: bool,
    pub flag_difficulty: u64,
    pub flag_bytecode: String,
    pub flag_recipient: String,
    pub flag_ethNetwork: String,
    pub cmd_submitEthBlock: bool,
    pub arg_key: String,
    pub arg_txId: String,
    pub arg_path: String,
    pub arg_value: String,
    pub arg_numUtxos: usize,
    pub arg_message: String,
    pub arg_address: String,
    pub arg_blockJson: String,
    pub arg_utxosJson: String,
    pub cmd_initializeEth: bool,
    pub cmd_initializeBtc: bool,
    pub cmd_submitBtcBlock: bool,
    pub cmd_getEnclaveState: bool,
    pub cmd_getLatestBlockNumbers: bool,
    pub cmd_debugAddUtxos: bool,
    pub cmd_debugRemoveUtxo: bool,
    pub cmd_debugGetAllUtxos: bool,
    pub cmd_debugGetAllDbKeys: bool,
    pub cmd_debugGetKeyFromDb: bool,
    pub cmd_debugClearAllUtxos: bool,
    pub cmd_debugMaybeAddUtxoToDb: bool,
    pub cmd_debugConsolidateUtxos: bool,
    pub cmd_debugReprocessBtcBlock: bool,
    pub cmd_debugReprocessEthBlock: bool,
    pub cmd_debugSetKeyInDbToValue: bool,
    pub cmd_debugErc777ChangePNetwork: bool,
    pub cmd_debugGetChildPaysForParentTx: bool,
    pub cmd_debugErc777ProxyChangePNetwork: bool,
    pub cmd_debugErc777ProxyChangePNetworkByProxy: bool,
    pub cmd_signMessageWithEthKey: bool,
    pub cmd_signHexMsgWithEthKeyWithPrefix: bool,
    pub cmd_signAsciiMsgWithEthKeyWithNoPrefix: bool,
}

impl CliArgs {
    pub fn update_block_in_cli_args(mut self, block_json: String) -> Result<Self> {
        self.arg_blockJson = block_json;
        Ok(self)
    }

    pub fn update_utxos_json_in_cli_args(mut self, json: String) -> Result<Self> {
        self.arg_utxosJson = json;
        Ok(self)
    }

    pub fn update_path_in_cli_args(mut self, bytecode_path: String) -> Result<Self> {
        self.arg_path = bytecode_path;
        Ok(self)
    }
}

pub fn parse_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO).and_then(|d| d.deserialize()) {
        Ok(cli_args) => Ok(cli_args),
        Err(e) => Err(AppError::Custom(e.to_string())),
    }
}

pub fn maybe_read_block_json_from_file(cli_args: CliArgs) -> Result<CliArgs> {
    match Path::new(&cli_args.flag_file).exists() {
        true => {
            info!("✔ File exists @ path: {}, reading file...", cli_args.flag_file);
            match cli_args {
                CliArgs {
                    cmd_debugAddUtxos: true,
                    ..
                } => {
                    info!("✔ Updating UTXOS in CLI args...");
                    cli_args
                        .clone()
                        .update_utxos_json_in_cli_args(read_to_string(cli_args.flag_file)?)
                },
                _ => {
                    info!("✔ Updating block in CLI args...");
                    cli_args
                        .clone()
                        .update_block_in_cli_args(read_to_string(cli_args.flag_file)?)
                },
            }
        },
        false => {
            info!("✔ No file exists @ path: {}, not reading file...", cli_args.flag_file);
            Ok(cli_args)
        },
    }
}

pub fn set_path_from_bytecode_flag(cli_args: CliArgs) -> Result<CliArgs> {
    match Path::new(&cli_args.flag_bytecode).exists() {
        false => Ok(cli_args),
        true => {
            info!("✔ File exists @ path: {}, reading file...", cli_args.flag_bytecode);
            cli_args.clone().update_path_in_cli_args(cli_args.flag_bytecode)
        },
    }
}

pub fn get_cli_args() -> Result<CliArgs> {
    parse_cli_args()
        .and_then(maybe_read_block_json_from_file)
        .and_then(set_path_from_bytecode_flag)
}
