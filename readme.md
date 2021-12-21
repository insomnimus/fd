# fd
Simple but fast file finder that respects your ignore files.

## Installation
`cargo install --locked --git https://github.com/insomnimus/fd --branch main`

## Usage
```sh
fd '*.pdf' # find a pdf document under the current directory
fd -p /bin --file 'clang*' -n0 # find every file that starts with clang under /bin
```
