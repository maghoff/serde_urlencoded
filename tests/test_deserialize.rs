#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;

#[test]
fn deserialize_bytes() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_bytes(b"first=23&last=42"),
               Ok(result));
}

#[test]
fn deserialize_str() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_str("first=23&last=42"),
               Ok(result));
}

#[test]
fn deserialize_reader() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(serde_urlencoded::from_reader(b"first=23&last=42" as &[_]),
               Ok(result));
}

#[test]
fn deserialize_option() {
    let result = vec![
        ("first".to_owned(), Some(23)),
        ("last".to_owned(), Some(42)),
    ];
    assert_eq!(serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_unit() {
    assert_eq!(serde_urlencoded::from_str(""), Ok(()));
    assert_eq!(serde_urlencoded::from_str("&"), Ok(()));
    assert_eq!(serde_urlencoded::from_str("&&"), Ok(()));
    assert!(serde_urlencoded::from_str::<()>("first=23").is_err());
}

#[test]
fn deserialize_string_in_struct_in_enum() {
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct ItemStruct {
        field: String
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(tag = "tag")]
    enum Test {
        Item(ItemStruct)
    };

    let result = Test::Item(ItemStruct{ field: "42".to_owned() });

    assert_eq!(serde_urlencoded::from_str("tag=Item&field=42"), Ok(result));
}

#[test]
fn deserialize_i32_in_struct_in_enum() {
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct ItemStruct {
        field: i32
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(tag = "tag")]
    enum Test {
        Item(ItemStruct)
    };

    let result = Test::Item(ItemStruct{ field: 42 });

    assert_eq!(serde_urlencoded::from_str("tag=Item&field=42"), Ok(result));
}
