pub fn hamming_weight(n: u32) -> i32 {
    let mut bytes_n = n << 31;
    let mut count = 0;

    println!("bytes_n {:b}", n);

    while bytes_n > 0 {
        println!("bytes_n {:b}", bytes_n);
        if (bytes_n & n) != 0 {
            count += 1;
        }

        bytes_n >>= 1;
    }

    println!("count {:?}", count);

    count
}
