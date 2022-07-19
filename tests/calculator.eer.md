## main.go
Questa è la descrizione del file `main.go`. Il parser dovrebbe ignorarla senza problemi.  
```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}
```

## stdout
Questo è lo standard output che mi aspetto.  
Sto pensando che potrei mettere `stdout`, `stdin`, `command` e `stderr` come nomi riservati allo scopo di testing automatico
```
Hello, world!
```

## command
```
go run main.go
```

## status
```
0
```