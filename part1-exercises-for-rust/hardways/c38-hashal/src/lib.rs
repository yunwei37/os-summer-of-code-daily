
pub fn fnv1a_hash(data:String)-> u32 {
    let mut hash:u32 = 2166136261;
    for i in data.chars(){
        hash = hash ^ (i as u32);
        hash *= 16777619;
    };
    hash
}

pub fn adler32_hash(data:String)-> u32 {
    let mut a = 0;
    let mut b = 0;
    for i in data.chars(){
        a = (a + (i as u32)) % 65521;
        b = (b+a) % 65521;
    };
    (b<<16)|a
}

pub fn djb_hash(data:String)-> u32 {
    let mut hash:u32 = 5381;
    for i in data.chars(){
        hash = ((hash<<5)+hash)+(i as u32);
    };
    hash
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_djb(){
        let a1 = djb_hash(String::from("test data 1"));
        assert_ne!(a1,0);

        let a2 = djb_hash(String::from("test data 2"));
        assert_ne!(a2,0);

        let a3 = djb_hash(String::from("xest data 3"));
        assert_ne!(a3,0);
        assert_ne!(a1,a2);
        assert_ne!(a3,a2);
        assert_ne!(a1,a3);

    }

    #[test]
    fn test_adler32(){
        let a1 = adler32_hash(String::from("test data 1"));
        assert_ne!(a1,0);

        let a2 = adler32_hash(String::from("test data 2"));
        assert_ne!(a2,0);

        let a3 = adler32_hash(String::from("xest data 3"));
        assert_ne!(a3,0);
        assert_ne!(a1,a2);
        assert_ne!(a3,a2);
        assert_ne!(a1,a3);

    }

    #[test]
    fn test_fnv1a(){
        let a1 = fnv1a_hash(String::from("test data 1"));
        assert_ne!(a1,0);

        let a2 = fnv1a_hash(String::from("test data 2"));
        assert_ne!(a2,0);

        let a3 = fnv1a_hash(String::from("xest data 3"));
        assert_ne!(a3,0);
        assert_ne!(a1,a2);
        assert_ne!(a3,a2);
        assert_ne!(a1,a3);

    }
}