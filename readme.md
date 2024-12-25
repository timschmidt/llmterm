## llmterm keeps a history of your shell usage and offers suggestions
![llmterm screenshot](doc/screenshot.png)

# cuda
llmterm uses kalosm to interface with the LLM.  Cuda support is enabled by default.  To choose other inference methods, edit the Cargo.toml file and rebuild.

# to build
```
cargo build --release
```

# to run
```
cargo run --release
```

# models
```
cargo run --release -- --model llama_3_1_8b_chat
```
- llama_3_1_8b_chat
- mistral_7b_instruct_2
- phi_3_5_mini_4k_instruct

# to exit
Use Ctrl-C, or type exit or quit.

# todo
- prune history to stay within the model's context window
- tab completion
- support remote LLMs via API: https://github.com/floneum/floneum/tree/main/interfaces/kalosm
- process bash_history for context
