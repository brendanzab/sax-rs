# sax-rs

Wrapper for libxml2's SAX parser.

## A simple example

~~~rust
let parser = sax::parse_str(XML_DATA);
for result in parser.iter() {
    match result {
        Ok(sax::StartDocument) => (),
        Ok(sax::EndDocument) => break,
        Ok(event) => println!("{}", event),
        Err(err) => println!("{}", err),
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

## Todo

- Messages for start/end element namespace callbacks

## License

This project is licensed under Apache License Version 2.0.

Please see the LICENSE file for more information.

