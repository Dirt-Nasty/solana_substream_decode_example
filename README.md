
# Match And Decode In Solana Substreams

I wrote this with the intention of bringing the `match_and_decode` functionality that is generated with Abigen on .abis to Solana for IDLs.  This is all based on the very small amount of knowledge and experience I have working on Solana.

I used a modified version of [TopLedger's Filter Transactions Substream](https://github.com/Topledger/solana-programs/tree/main/filter-program-transactions). I modified it to use a module parameter to parse transactions for a given list of program addresses.  In their version they had a hard coded list.  It will only return transactions where the provided program addresses are the executing account in the outer or inner instructions.  You can find my modified version that supports params [here](https://github.com/Dirt-Nasty/solana_program_transactions).


## Usage/Examples

I'll list here what I've done and the steps you should take to replicate with your IDL of choice.

### cargo.toml
Besides the typical dependencies need when working with Solana substreams I added two lines here.  One for the repo of the IDL and one for `anchor-lang` which is used to grab the discriptors of each instruction in the IDL.  There was some version conflicts I had to navigate through to get the rust-analyzer to not throw an error.

```toml
message-transmitter = { git = "https://github.com/circlefin/solana-cctp-contracts", branch = "master" }
anchor-lang = { version = "=0.28.0", features = ["init-if-needed", "event-cpi"] }
```

### substreams.yaml
Here I'm importing my modified substream so it can be used as an input in my `map_events` module.  It's also important to note the binary type.  You NEED to specify this binary type for the IDL to work properly with substreams.  Other than that business as usual.

```yaml
imports:
  substreams_solana_transactions: https://github.com/Dirt-Nasty/solana_program_transactions/releases/download/v1.0.6/tl-solana-program-transactions-1-0-6-v1.0.6.spkg

binaries:
  default:
    type: wasm/rust-v1+wasm-bindgen-shims
    file: target/wasm32-unknown-unknown/release/substreams.wasm

networks:
  solana:
    params:
      substreams_solana_transactions:map_block: "circle-cctp:CCTPmbSD7gX1bxKPAmg77w8oFzNFpaQiQUWD43TKaecd"
```

### proto/program.proto
I'm just specify what the map_events module will output here.  It resembles what is generated with the `substreams init` command for ethereum substreams.  For this example I didn't replicate all instructions just a few.

### src/idl/*
Stepping into the src folder you'll notice a folder labeled idl.  This is where we are using the imported idl.  I decided to add a trait that resembles what is generated using `substreams_ethereum::Abigen`.  `match_log`, `decode`, and `match_and_decode`.  These will allow us to parse the bytes from the instruction data and check if it matches the Instruction and decode those bytes into the instruction params from the IDL.  You must imply this trait to each instruction you'd like to filter out in your map module.  This can be replicated for each instruction just swapping out the names.

`match_log` will check if the bytes passed in starts with the instruction discriminator using `starts_with` and return a bool.

`decode` will check the bytes passed in and will drop the first 8 bytes and then `try_from_slice` into the instruction params.

```rust
pub trait Instruction: Sized {
    fn match_log(data: &Vec<u8>) -> bool;
    fn decode(data: &Vec<u8>) -> Result<Self, String>;
    fn match_and_decode(data: &Vec<u8>) -> Option<Self> {
        if Self::match_log(data) {
            Self::decode(data).ok()
        } else {
            None
        }
    }
}

impl Instruction for message_transmitter::instruction::SendMessage {
    fn match_log(data: &Vec<u8>) -> bool {
        data.starts_with(&Self::DISCRIMINATOR)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, String> {

        if data.len() >= 8 {
            
            let decode = self::Instructions::SendMessageParams::try_from_slice(&data[8..]);

            match decode  {
                Ok(d) => {
                    return Ok(
                        Self{
                            params : d
                        }
                    )
                },
                _ => { 
                    Err("Unable to decode".to_string()) }
            }
        }
        else {
            Err("Invalid data length".to_string())
        }
    }
}
```

### modules/001_map_events.rs
Here is where we'll `match_and_decode` the data from the instructions.  We are already filtering out transactions from that modified substream that include the program(s) in question but you'll get ALL instructions.  We just want instructions where the `executing_account == program_address` on both the outer and inner instructions.  Once we filter out that condition then we'll take the `data` and pass it into our match_and_decode to see if it is the instruction we're looking for.  If it is then we'll decode it and append to our events for the module output.

```rust
if inst.executing_account == program_address {
    if let Some(event) = idl::ReceiveMessage::match_and_decode(&inst.data)
```

and with all that - you now have a similar output of what you'd get if you ran `substreams init`

```
"messageTransmitterReceiveMessages": [
    {
      "evtTxHash": "2cxFvcmrQbpp4ex5CfzmRhS2ndJzwjCyVA8bZkrjWrPUpNEUcCFSbxZShrNFsFfNpd1LsTaBNqYUtwsUu6LVeJzM",
      "evtIndex": 480,
      "evtBlockTime": "2024-08-04T17:27:08Z",
      "evtBlockNumber": "281550642",
      "txGasFee": "125000",
      "txCaller": "DdcBeTaeQwsxWF7B2yLfY3Hu5HwbeZXmSHL2unFd7KBf",
      "message":
"0x0000000000000000000000050000000000016901000000000000000000000000bd3fa81b58ba92a82136038b25adec7066af3155a65fc943419a5ad590042fd67c9791fd015acf53a54cc823edb8ff81b9ed722ee7a885e54d080de088b949c28db5e166ccd256
58e4752f5faed32c2ebc55dc4500000000000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48ccc17da2378a3dd885a3a46d21eaee35540503c3c316f80e57ea27a73a2895b000000000000000000000000000000000000000000000000
000000005d21dba00000000000000000000000000f18f923480dc144326e6c65d4f3d47aa459bb41c",
      "attestation":
"0xc91b99145fdc771e8e7fbc1f90da53d357ae40b0cd6b3bb0b5ffec39c52b6dd416098504f985f10d6534e48c804be8fb1d8ffe5a4513da1378ad7dddb93e22d31cbd8c91b143febc3933d07aba0ca4269ee65f834ca32714e00363083cee935cc63ce76fc50103
7f46448b303742ba7f2c9cb52d88c679c3d2ca62f755f49627051b"
    }
  ]
```