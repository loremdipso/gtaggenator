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
    - [x] Implement dump_tags
    - [x] Implement dump
    - [x] Implement Duplicate checking
    - [x] Implement renaming
    - [x] Get rid of unique name requirement?
    - [ ] Test renaming code/deduplication work
    - [x] Implement batching in Async SQL queries
    - [ ] Add dates
    - [ ] Flush before reads? Yay or nay?

2. GUI

    - [x] POC
    - [ ] Figure out plugin architecture
        - Load how?
        - Can they be dynamic?
    - [x] How do we load files?
    - [x] How do we load files?
        - [x] Make sure we aren't leaking memory/crashing
    - [ ] Am I re-requesting an image anytime I change anything?
    - [x] Add toasts
    - [ ] Add scss

3. Backburner
    - [ ] Figure out migrations
        - [ ] Maybe query schema, figure out if anything's missing?
    - [ ] How do we ensure we don't leave the DB in a funky state?
    - [ ] Package thread-safe SQLite with the project instead of using whatever's installed
    - [ ] Swap font to roboto-mono, maybe
