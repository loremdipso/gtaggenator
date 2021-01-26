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
    - [x] Implement batching in Async SQL queries
    - [x] Add dates
    - [x] Flush before reads? Yay or nay? Maybe with reload?
    - [ ] Test renaming code/deduplication work
    - [ ] Support rename

2. GUI

    - [x] POC
    - [x] How do we load files?
    - [x] How do we load files?
        - [x] Make sure we aren't leaking memory/crashing
    - [ ] Am I re-requesting an image anytime I change anything?
    - [x] Add toasts
    - [ ] Add scss
    - [x] Make handle prettier
    - [ ] Add keymap, ideally an editable one
        - Save config to tsettings
    - [ ] Make play into tabs (with configurable filters?
    - [x] Preload images in browser
    - [x] Dynamically bind filesystem to port
    - [ ] Add settings edit screen
    - [ ] Add tag edit screen
    - [ ] Exit on ctrl+q

3. Backburner
    - [ ] Figure out plugin architecture
        - Load how?
        - Can they be dynamic?
        - Do we even want to go to the trouble?
    - [ ] Cache Zip/pages on server
    - [ ] Figure out migrations
        - [ ] Maybe query schema, figure out if anything's missing?
    - [ ] How do we ensure we don't leave the DB in a funky state?
    - [ ] Package thread-safe SQLite with the project instead of using whatever's installed
    - [ ] Swap font to roboto-mono, maybe
    - [ ] If no tagg.db, open launch screen with a list of existing directories. For use when launching not from CLI
