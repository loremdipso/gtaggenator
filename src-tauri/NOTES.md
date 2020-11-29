# Roadmap

1. Cli
    - [x] POC
    - [x] Separate from GUI so we could also release a less bloated standalone executable
    - [x] Fast file system traversal
    - [x] Settings
        - [x] Load
        - [x] Modify
        - [x] Save
    - [x] Efficient, persistent DB
        - In-memory?
        - SQL or KVS?
    - [x] Make writes async and batch them together if possible
    - [x] Create schema
    - [ ] Implement dump_tags
2. GUI

    - [x] POC
    - [ ] Figure out plugin architecture
        - Load how?
        - Can they be dynamic?

3. Backburner
    - [ ] Figure out migrations
    - [ ] How do we ensure we don't leave the DB in a funky state?
    - [ ] Package thread-safe SQLite with the project instead of using whatever's installed
