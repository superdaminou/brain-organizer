Welcome to my brain manager. 

Functionnality:

- Save your web references and associate them with tags to better retrieve them
- Create and write your thoughts, each one saved on a simple text file.
- read, write and visualize dot graphs.
- Export all your data in a single repository 
- Import From a directory
- Expose it as a web server


How to run:
- cargo run -- [gui|import|export|web] (optionnal)

How to release:

- cargo build --release 
- retrieve executable under ./target/release/ and run

Modes:
- [DEFAULT] GUI: Visualize and manager all through a simple gui 
- Export: Save all you data in a single export repository 
- Import: Restore All from the import directory 
- Web: Expose API as a http server