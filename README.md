# todara - A TODO Aggregator

This tool was created for the purpose of learning Rust. 

todara pulls TODOs from all python files given a directory. If a directory contains a `.gitignore` then todara excludes files that match the git exclude patterns.

## Demo
![gif](https://i.imgur.com/JXTZqQC.gif)

## Requirements

To run todara you must have Rust installed.

To install Rust on OS X or Linux run the following:

```bash
curl https://sh.rustup.rs -sSf | sh
```
For Windows click on this [link](https://win.rustup.rs/) to download an executable.

## Run

Clone the repository and from the root run the following:

```bash
cargo run [directory]
```

Where `[directory]` is the directory of a python repo or directory with python files. It prints all TODOs found in python files into a table.

You can also write the table into a file by running the following

```bash
cargo run [directory] > TODOs.txt
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
