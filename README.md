Welcome to my brain manager. 

Functionnality:

- Save your personnal web references and associate them with tags to better retrieve them
- Create and write your thoughts, each one saved on a simple text file.
- Manage your graph
- Export all your data in a single repository 
- [IN PROGRESS] Graph export
- Import From a directory

How to run:
- cargo run -- [gui|import|export] (optionnal)

How to release:

- cargo build --release 
- retrieve under ./targer/release/brain_manager.exe and run

Modes:
- [DEFAULT] GUI: Visualize and manager all through a simple gui 
- Export: Save all you data in a single export repository 
- Import: Restore All from the import directory 