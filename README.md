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
