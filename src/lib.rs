// Custom errors for Bitcoin operations
#[derive(Debug)]
pub enum BitcoinError {
    ParseError(String),
    InvalidTransaction,
    InsufficientFunds,
    InvalidAddress,
}
impl std::fmt::Display for BitcoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitcoinError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            BitcoinError::InvalidTransaction => write!(f, "Invalid Transaction"),
            BitcoinError::InsufficientFunds => write!(f, "Insufficient Funds"),
            BitcoinError::InvalidAddress => write!(f, "Invalid Address"),
        }
    }
}
impl std::error::Error for BitcoinError {}

// Generic Point struct for Bitcoin addresses or coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Custom serialization for Bitcoin transaction
pub trait BitcoinSerialize {
    fn serialize(&self) -> Vec<u8>;
}

// Legacy Bitcoin transaction
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl LegacyTransaction {
    pub fn builder() -> LegacyTransactionBuilder {
        LegacyTransactionBuilder::new()
    }
}

// Transaction builder
pub struct LegacyTransactionBuilder {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Default for LegacyTransactionBuilder {
    fn default() -> Self {
        LegacyTransactionBuilder {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }
}

impl LegacyTransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version(mut self, version: i32) -> Self {
        self.version = version;
        self
    }

    pub fn add_input(mut self, input: TxInput) -> Self {
        self.inputs.push(input);
        self
    }

    pub fn add_output(mut self, output: TxOutput) -> Self {
        self.outputs.push(output);
        self
    }

    pub fn lock_time(mut self, lock_time: u32) -> Self {
        self.lock_time = lock_time;
        self
    }

    pub fn build(self) -> LegacyTransaction {
        LegacyTransaction {
            version: self.version,
            inputs: self.inputs,
            outputs: self.outputs,
            lock_time: self.lock_time,
        }
    }
}

// Transaction components
#[derive(Debug, Clone)]
pub struct TxInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // in satoshis
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}

// CLI command enum
pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    if args.is_empty() {
        return Err(BitcoinError::ParseError(
            "No arguments provided".to_string(),
        ));
    }

    match args[0].as_str() {
        "send" => {
            if args.len() < 3 {
                return Err(BitcoinError::ParseError(
                    "Send command requires amount and address".to_string(),
                ));
            }

            let amount = args[1]
                .parse::<u64>()
                .map_err(|_| BitcoinError::ParseError("Invalid amount".to_string()))?;
            let address = args[2].clone();

            Ok(CliCommand::Send { amount, address })
        }
        "balance" => Ok(CliCommand::Balance),
        _ => Err(BitcoinError::ParseError("Invalid command".to_string())),
    }
}

// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 16 {
            return Err(BitcoinError::InvalidTransaction);
        }

        // Parse version (4 bytes)
        let version = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        // Parse inputs count (4 bytes)
        let inputs_count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

        // Parse outputs count (4 bytes)
        let outputs_count = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);

        // Parse lock_time (4 bytes)
        let lock_time = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);

        // Create vectors with appropriate capacity
        let inputs = Vec::with_capacity(inputs_count as usize);
        let outputs = Vec::with_capacity(outputs_count as usize);

        // For simplicity, we're not actually parsing the input/output data
        // Just creating empty vectors with the right capacity

        Ok(LegacyTransaction {
            version,
            inputs,
            outputs,
            lock_time,
        })
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Serialize version (4 bytes)
        result.extend_from_slice(&self.version.to_le_bytes());

        // Serialize lock_time (4 bytes)
        result.extend_from_slice(&self.lock_time.to_le_bytes());

        result
    }
}
