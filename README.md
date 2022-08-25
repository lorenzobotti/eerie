# eerie: e2e testing for command line apps
eerie is a tool to make testing easier for complex command line apps that work on large files. It lets you define the input files and the expected output of your command and tests it for you.

the eerie file format is just Markdown. your test is also its own documentation.

## how to use
### file format
each file is a subparagraph (h2). the paragraph's title (identified by `##`) is the file's name. the paragraph (optional) can contain a short description of what the file is and does. the file's content resides in the code block. three backticks add a newline at the end, four backticks don't. after the code block starting delimiters you can put the file's language.  
example:  
`````
## main.rs
this is a description of what this file does. the file's contents will follow shortly.  
```rust
// ^
// |
// this is the language this file is written in. it's optional


fn main() {
    println!("Hello, world!");
}


// the three backticks tell eerie to add a newline at the end
// |
// v
``` 
````` 

### turning it into a folder
you can use the cli to create a folder based on a docfile.
```
$ eerie create tests/example.eer.md ./target-folder
```

### running the test
```
$ eerie run tests/example.eer.md ./target-folder
```

