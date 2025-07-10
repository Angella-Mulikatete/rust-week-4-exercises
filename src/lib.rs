

// Custom errors for Bitcoin operations
#[derive(Debug)]
pub enum BitcoinError {
   InvalidTransaction,
    InsufficientFunds,
    InvalidAddress,
    ParseError(String),
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

// Generic Point struct for Bitcoin addresses or coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        // TODO: Implement constructor for Point
        Point { x, y }
    }
}
 impl std::error::Error for BitcoinError {}
     


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
        // TODO: Return a new builder for constructing a transaction
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
        // TODO: Implement default values
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
        // TODO: Initialize new builder by calling default
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

pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    // TODO: Match args to "send" or "balance" commands and parse required arguments
    if args.is_empty() {
        return Err(BitcoinError::ParseError("No command provided".to_string()));
    }
    match args[0].as_str(){
        "send" => {
            if args.len() < 3 {
                return Err(BitcoinError::ParseError("send requires amount and address".to_string()));
            }
            let amount = args[1].parse::<u64>().map_err(|_| {
                BitcoinError::ParseError("Invalid amount for send".to_string())
            })?;
            let address = args[2].clone();
            Ok(CliCommand::Send { amount, address })
        },
        "balance" => Ok(CliCommand::Balance),
        _ => Err(BitcoinError::ParseError("Unknown command".to_string())),
        }
    }


// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // TODO: Parse binary data into a LegacyTransaction
        // Minimum length is 10 bytes (4 version + 4 inputs count + 4 lock_time)
        if data.len() < 10 {
            return Err(BitcoinError::InvalidTransaction);
        }
        let version = i32::from_le_bytes(data[0..4].try_into().map_err(|_| BitcoinError::InvalidTransaction)?);
        let inputs_count = u32::from_le_bytes(data[4..8].try_into().map_err(|_| BitcoinError::InvalidTransaction)?);
        let lock_time = u32::from_le_bytes(data[8..12].try_into().map_err(|_| BitcoinError::InvalidTransaction)?);
        Ok(LegacyTransaction {
            version,
            inputs: Vec::with_capacity(inputs_count as usize),
            outputs: Vec::new(),
            lock_time,
        })
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        // TODO: Serialize only version and lock_time (simplified)
         let mut result = Vec::with_capacity(8);
        result.extend_from_slice(&self.version.to_le_bytes());
        result.extend_from_slice(&self.lock_time.to_le_bytes());
        result
    }
}   
