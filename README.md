## puffyfind - openbsd pkg search tool made in rust
alot faster and easier than 'pkg_info'
## how to compile?
```
git clone https://github.com/suigarra/puffyfind
cd puffyfind
cargo build
doas cp ./target/debug/puffy_find /usr/bin/puffyfind
```
done! now run it like this
```
puffyfind
```
you can leave the program by using "quit".
