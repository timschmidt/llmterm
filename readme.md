## llmterm keeps a history of your shell usage and offers suggestions

# cuda
llmterm uses kalosm to interface with the LLM.  Cuda support is enabled by default.  To choose other inference methods, edit the Cargo.toml file and rebuild.

# to build
```
cargo build
```

# to run
```
cargo run
```

# models
```
cargo run -- --model llama_3_1_8b_chat
```
- llama_3_1_8b_chat
- mistral_7b_instruct_2
- phi_3_5_mini_4k_instruct

# to exit
type Ctrl-C, or exit, or quit and press enter/return.
