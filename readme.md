
# Parsers

# Json parser / deserializer
I made this parser in three iterations. This was very benefitial for me learning how to program with bytes and how a parser / tokenizer works.
This also helped me a lot to develop a greater understanding for the Rust programming language.
The parser works, but there might be some bugs here and there but it fine.   

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
    "lastCoordinates" : [["lat 84,45369", "long 12.5467"], ["lat 55.255657", "long 67.35677"]]
}
```
returns 

```rust
[
    Object(
        {
            "age": Value(
                Int(
                    23,
                ),
            ),
            "residency": Object(
                {
                    "address": Value(
                        String(
                            "One Way 21",
                        ),
                    ),
                    "city": Value(
                        String(
                            "Big City",
                        ),
                    ),
                    "zip": Value(
                        Int(
                            123567,
                        ),
                    ),
                },
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
                            "age": Value(
                                Int(
                                    2,
                                ),
                            ),
                            "name": Value(
                                String(
                                    "Tom",
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
                            "age": Value(
                                Int(
                                    1,
                                ),
                            ),
                            "animal": Value(
                                String(
                                    "mouse",
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
                                            "lat 84",
                                        ),
                                    ),
                                    Value(
                                        String(
                                            "45369",
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
            "firstName": Value(
                String(
                    "John",
                ),
            ),
        },
    ),
]

```




#Html parser
- On going
