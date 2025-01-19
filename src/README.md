# Ross Ulbricht Freedom Watch Bot

## To build this project:

Add a `secret.rs` to `src` directory with contents:

>```
>pub const BECH32_NSEC: &str = "nsecXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
>```

where the `nsecXXX...` value is replaced by your bot's nostr private key

## Command Line Arguments

Arg 1: `true|false` (optional, indicates that the bot should actually post to nostr, default = `false`)

Arg 2: current date epoch timestamp (optional with presense of Arg 1, manually provides the timestamp for the current datetime, default = runtime `Utc.now()`)

### Examples: 
| Result      | CLI command      |
| ------------- | ------------- |
| Run with live posting to nostr, defaulting to runtime `UTC:now` | `cargo run -- true` |
| Run, defaulting to no posting and runtime `UTC::now` | `cargo run` |
| Run with live posting to nostr with system timestamp | `cargo run -- true $(date '+%s')` |
| Run without posting to nostr with specific epoch time | `cargo run -- false 1737521940` |



