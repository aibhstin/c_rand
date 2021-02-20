#[link(name = "c")]
extern "C" {
    fn rand() -> u64;
    fn srand(_: u64);
}

pub fn do_srand(seed: u64) {
    unsafe {
        srand(seed);
    };
}

pub fn get_rand() -> u64 {
    let n = unsafe { rand() };
    n
}

pub fn get_rand_n(n: u64) -> u64 {
    get_rand() % n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rand_test() {
        println!("{}", get_rand());
    }

    #[test]
    fn get_rand_n_test() {
        println!("{}", get_rand_n(100));
        println!("{}", get_rand_n(100));
    }

    #[test]
    fn do_srand_test() {
        println!("{}", get_rand_n(100));
        do_srand(100);
        println!("{}", get_rand_n(100));
    }
}
