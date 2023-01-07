## Usage
```
cargo build
./target/debug/rs_textviewer sample.txt
1| I'm nobody! Who are you?
2| Are you nobody, too?
3| Then there's a pair of us - don't tell!
4| They'd banish us, you know.
5| 
> 
```

### commands
- line
```
> line 4
4| They'd banish us, you know.
```

```
> line 1-3
1| I'm nobody! Who are you?
2| Are you nobody, too?
3| Then there's a pair of us - don't tell!
```
