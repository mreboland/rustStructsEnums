fn main() {
    // A struct is a type that's composed of other types. Like tuples, the pieces of a struct can be different types, but you can name each piece of data so it's clear what the values mean.

    // Structs come in three flavours:

    // A struct with named fields
    // struct Person {
    //     name: String,
    //     age: u8,
    //     likes_oranges: bool
    // }

    // A tuple struct
    // struct Point2D(u32, u32);

    // A unit struct
    // struct Unit;

    
    // Classic C structs are the most commonly used. Each field defined within them has a name and a type. After they're defined, they can be accessed by using example_struct.field syntax.
    // Tuple structs are similar to classic structs, but their fields have no names. For accessing individual variables, the same syntax is used as with regular tuples, namely, foo.0, foo.1, and so on, starting at zero.
    // Unit structs are most commonly used as markers. We'll learn more about why these might be useful when we learn about Rust's traits feature.

    // INSTANTIATE STRUCTS

    // To use a struct after we've defined it, we create an instance of that struct by specifying concrete values for each of the fields.

    // Instantiate a classic struct, with named fields. Order does not matter.
    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    // Instantiate a tuple struct by passing the value in the same order as defined
    let origin = Point2D(0, 0);

    // Instantiate a unit struct
    let unit = Unit;


    // ENUMS

    // Enums are types that can be any one of several variants. The enum keyword allows the creation of a type, which might be one of a few different variants. Enum variants, just like struct, can have fields with names, fields without names, or no fields at all.

    enum WebEvent {
        // An enum may either be unit-like
        PageLoad,
        PageUnload,
        // An enum can include characters and strings
        KeyPress(char),
        Paste(String),
        // or include tuple structs
        Click { x: i64, y: i64 },
    }

    // This enum has four variants with different types:

    // PageLoad and PageUnload have no data associated with it at all.
    // Keypress includes a single character in it.
    // Paste includes a single string.
    // Click includes an anonymous struct inside it.

    // ll the variants are grouped together under the same WebEvent type and each variant is not its own type. This means we can't have functions that only accept KeyPress and not other variants of the WebEvent enum.

    // We can chose to define separate structs for each variant and then have each variant hold on to the different structs. These would hold the same data that the preceding enum variants held. But this definition would allow users to refer to each logical variant on its own.

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(KeyPress),
        Paste(String),
        Click(Click)
    }

    struct Click {
        x: i64,
        y: i64
    }

    struct KeyPress(char);

    // Now in our code we can refer to a WebEvent::Click which is a variant of the type WebEvent and you can also just refer to Click's on their own, separate from WebEvents.
}
