
#[derive(Debug, PartialEq,Clone)]
pub enum MapValue {
    Empty,
    Str(String),
    I32(i32),
    BOOL(bool),
}

impl Into<bool> for &MapValue {
    fn into(self) -> bool {
        if let MapValue::BOOL(i) = self {
            *i
        } else {
            panic!("not a bool")
        }
    }
}

impl Into<bool> for MapValue {
    fn into(self) -> bool {
        if let MapValue::BOOL(i) = self {
            i
        } else {
            panic!("not a bool")
        }
    }
}

impl From<bool> for MapValue {
    fn from(src: bool) -> MapValue {
        MapValue::BOOL(src)
    }
}

impl From<i32> for MapValue {
    fn from(src: i32) -> MapValue {
        MapValue::I32(src)
    }
}

impl Into<i32> for &MapValue {
    fn into(self) -> i32 {
        if let MapValue::I32(i) = self {
            *i
        } else {
            panic!("nnot an i32")
        }
    }
}

impl Into<i32> for MapValue {
    fn into(self) -> i32 {
        if let MapValue::I32(i) = self {
            i
        } else {
            panic!("nnot an i32")
        }
    }
}

impl Into<String> for MapValue {
    fn into(self) -> String {
        if let MapValue::Str(i) = self {
            i
        } else {
            panic!("not a string")
        }
    }
}
 

impl From<String> for MapValue {
    fn from(src: String) -> MapValue {
        MapValue::Str(src)
    }
}

impl<'a> From<&'a String> for MapValue {
    fn from(src: &'a String) -> MapValue {
        MapValue::Str(src.clone())
    }
}

impl From<&str> for MapValue {
    fn from(src: &str) -> MapValue {
        MapValue::Str(src.to_owned())
    }
}

impl<'a> Into<&'a str> for &'a MapValue {
    fn into(self) -> &'a str {
        if let MapValue::Str(i) = self {
            i
        } else {
            panic!("not an str")
        }
    }
}
