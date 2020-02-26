use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u32, &'static str>> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        Mutex::new(m)
    };
}

pub fn typecheck_root() {}

mod test {
    use super::HASHMAP;

    #[test]
    fn test_hashmap() {
        assert_eq!(HASHMAP.lock().unwrap().get(&1).unwrap(), &"bar");
        HASHMAP.lock().unwrap().insert(3, "fff");
        assert_eq!(HASHMAP.lock().unwrap().get(&3).unwrap(), &"fff");
    }
}
