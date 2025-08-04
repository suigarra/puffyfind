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

## how to change the mirror, structure or repo directory?
you can configure all that stuff on ./src/main.rs
```
    let mirror = "openbsd.cs.toronto.edu";

    let _ = client.cwd("pub/OpenBSD/7.7/packages/amd64");
```
these 2 lines are the ones that you should want if you are looking to configure the tool.
### note: the tool uses ftp for checking packages so it must be ftp mirrors
ftp mirrors can be found in https://www.openbsd.org/ftp.html#ftp
### PLANETUNIX FTP MIRROR DOES NOT WORK BECAUSE IT DOESNT SUPPORT PASSIVE MODE
