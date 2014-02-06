# sax-rs

Wrapper for libxml2's SAX parser.

## A simple example

~~~rust
let sax = parse_xml("<yo>hullo!</yo>");
loop {
    match sax.recv() {
        Ok(StartDocument) => (),
        Ok(EndDocument) => break,
        Ok(event) => println(event.to_str()),
        Err(err) => println(err.to_str()),
    }
}
~~~

## Compile

~~~
make
~~~

## Run tests

~~~
make check
~~~

## Install

~~~
make install
~~~

## Todo

- Messages for start/end element namespace callbacks

## License

This project is licensed under Apache License Version 2.0.

Please see the LICENSE file for more information.

