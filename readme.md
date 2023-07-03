
# Parsers

## Json parser / deserializer
Note: Done without any tutorial!

I made this parser in three iterations. (As I got better ideas for how to do it 3 times) This was very benefitial for me learning how to program with bytes and how a parser / tokenizer works.
This also helped me a lot to develop a greater understanding for the Rust programming language.
The parser works, but there might be some bugs here and there but it's fine.   

This project also allowed me to understand how objects, and arrays differ on a lower level

Try it out using the ```rust Json::parse()``` function and add a string slice 
```rust 
&str 
```
as an argument. Example:
```rust 
Json::parse("{\"firstName\" : \"John\",\"lastName\" : \"Doe\"}");
```
Below is another example of the result.

```json
{
    "firstName" : "John",
    "lastName" : "Doe",
    "age" : 23,
    "residency" : {
        "address" : "One Way 21",
        "zip" : 123567,
        "city" : "Big City"
    },
    "pets" : [{"animal" : "cat", "age" : 2, "name" : "Tom"}, {"animal" : "mouse", "age" : 1, "name" : "Jerry"}],
    "lastCoordinates" : [["lat 84.45369", "long 12.5467"], ["lat 55.255657", "long 67.35677"]]
}
```
returns 

```rust
Object(
        {
            "firstName": Value(
                String(
                    "John",
                ),
            ),
            "age": Value(
                Int(
                    23,
                ),
            ),
            "lastName": Value(
                String(
                    "Doe",
                ),
            ),
            "pets": Array(
                [
                    Object(
                        {
                            "animal": Value(
                                String(
                                    "cat",
                                ),
                            ),
                            "name": Value(
                                String(
                                    "Tom",
                                ),
                            ),
                            "age": Value(
                                Int(
                                    2,
                                ),
                            ),
                        },
                    ),
                    Object(
                        {
                            "name": Value(
                                String(
                                    "Jerry",
                                ),
                            ),
                            "animal": Value(
                                String(
                                    "mouse",
                                ),
                            ),
                            "age": Value(
                                Int(
                                    1,
                                ),
                            ),
                        },
                    ),
                    Value(
                        String(
                            "lastCoordinates",
                        ),
                    ),
                    Array(
                        [
                            Array(
                                [
                                    Value(
                                        String(
                                            "lat 84.45369",
                                        ),
                                    ),
                                    Value(
                                        String(
                                            "long 12.5467",
                                        ),
                                    ),
                                ],
                            ),
                            Array(
                                [
                                    Value(
                                        String(
                                            "lat 55.255657",
                                        ),
                                    ),
                                    Value(
                                        String(
                                            "long 67.35677",
                                        ),
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
            "residency": Object(
                {
                    "address": Value(
                        String(
                            "One Way 21",
                        ),
                    ),
                    "zip": Value(
                        Int(
                            123567,
                        ),
                    ),
                    "city": Value(
                        String(
                            "Big City",
                        ),
                    ),
                },
            ),
        },
    )

```




## Html parser
- On going
