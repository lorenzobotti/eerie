## main.go
this is the main file, the entry point. the `main()` function is the first one that gets called. this paragraph serves as documentation for the file's contents.  

```go
package main

import (
    "fmt"
    "os"
)

func main() {
    if len(os.Args) < 2 {
        fmt.Fprintln(os.Stderr, "expected at least one argument")
        os.Exit(1)
    }

    name := os.Args[1]
    fmt.Print(hello(name)) // notice how there's no newline at the end
}
```

## hello.go
this is where the `hello()` function is defined. it's a separate file but since it's in the same package it shares symbols with `main.go`.  
```go
package main

func hello(name string) string {
    return "Hello, " + name + "!"
}
```

## go.mod
this is not code, it's the Go equivalent of `package.json`  
```
go 1.16

module hamburger
```


## command
this is the command that should be run.  
```
go run . Giovanni
```

## stdout
we can define here what we expect the command's output to be. if the output is not exactly this the test will fail.  
notice how there's four backticks instead of the usual three. four backticks tells eerie to not consider the trailing newline
````
Hello, Giovanni!
````

## status
this is the status code we expect from the command.  
```
0
```