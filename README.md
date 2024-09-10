# hunter2gen
Password Generator CLI Tool

This is a small project to help me learn Rust. I typically use my browser/OS built in password manager but they don't always recognize when fields need a new generated password(this is primarily a browser issue). 

From a terminal a user would open the program by typing it's name `hunter2gen`
Standard behavior without flags would provide a 16 character password containing upper and lower case letters, numbers, and special characters in a randomized pattern. User would then have the option to store password, create new standard password, or exit program.

Future versions might include a --phrase flag to produce [xkcd](https://xkcd.com/936/) style passwords delimited and padded with numbers and special characters

Choosing to store a password would prompt for a keyword to attach to the password in an encrypted TOML file.

Pass a `--list` flag:

`$hunter2gen --list`

would display the contents of the TOML file. 

Future versions might try to implement system password or fingerprint scan before this is displayed.

TODO:
- ~~push basic working changes~~
- figure out UI
- set up build for CLI tool
- ~~research dictionary~~
- add flags (padding, words, reference)
- figure out persistence
- figure out encryption
- publish on crates?
- ?create compatible firefox plugin? *new rust/wasm project*