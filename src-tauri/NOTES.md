# Roadmap

1. Cli
    - [x] POC
    - [x] Separate from GUI so we could also release a less bloated standalone executable
    - [x] Fast file system traversal
    - [ ] Settings
        - [x] Load
        - [x] Modify
        - [x] Save
    - [x] Efficient, persistent DB
        - In-memory?
        - SQL or KVS?
    - [ ] Make writes async and batch them together if possible
2. GUI
    - [x] POC
    - [ ] Figure out plugin architecture
        - Load how?
        - Can they be dynamic?
