use ::utils::u8x4_to_u32;

    /* CONSTANTES
      ************/

const INIT_STATE : (u32, u32, u32, u32) =
    (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

const K : [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

const S : [u8;  64] = [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21
];

    /* FONCTIONS UTILITAIRES
      ***********************/

fn rotl(n : u32, c : u8) -> u32 {
    (n << c) | (n >> (32-c))
}

fn tuple_add(tuple1 : (u32, u32, u32, u32), tuple2 : (u32, u32, u32, u32))
    -> (u32, u32, u32, u32)
{
    let (a1, b1, c1, d1) = tuple1;
    let (a2, b2, c2, d2) = tuple2;
    (a1.wrapping_add(a2), b1.wrapping_add(b2),
     c1.wrapping_add(c2), d1.wrapping_add(d2))
}

    /* ÉTAPES SUCCESSIVES
      ********************/

fn padding(bytes : &[u8]) -> Vec<u8>    {
    let mut vec        = bytes.to_vec();
    let     length     = vec.len();
    let mut bit_length = length as u64 * 8;
    let     pad_count  =
        if length % 64 < 56 { 55 - length % 64      }
        else                { 63 - length % 64 + 56 };

    vec.push(0x80);

    for _i in 0..pad_count  {
        vec.push(0x00);
    }

    for _i in 0..8  {
        let byte        = (bit_length % 0x100) as u8;
            bit_length /= 0x100;
        vec.push(byte as u8);
    }

    vec
}

fn slicing(bytes : &[u8]) -> Vec<Vec<u32>>  {
    let mut vec = Vec::new();

    for chunk in bytes.chunks(64)   {
        let mut vec2 = Vec::new();

        for word in chunk.chunks(4) {
            vec2.push(u8x4_to_u32(word));
        }

        vec.push(vec2);
    }

    vec
}

fn f(i : usize) -> Box<Fn(u32, u32, u32) -> u32>    {
         if i < 16 { Box::new(|b, c, d| d ^ (b & (c ^ d))) }
    else if i < 32 { Box::new(|b, c, d| c ^ (d & (b ^ c))) }
    else if i < 48 { Box::new(|b, c, d| b ^  c ^  d      ) }
    else           { Box::new(|b, c, d| c ^ (b | (  ! d))) }
}

fn g(i : usize) -> usize    {
         if i < 16 {      i           }
    else if i < 32 { (5 * i + 1) % 16 }
    else if i < 48 { (3 * i + 5) % 16 }
    else           { (7 * i    ) % 16 }
}

fn main_loop(state : (u32, u32, u32, u32), m : Vec<u32>)
    -> (u32, u32, u32, u32)
{
    let (mut a, mut b, mut c, mut d) = state;
    let mut temp;

    for i in 0..64  {
        let f = f(i);
        let g = g(i);
        let new_b = b.wrapping_add(rotl(
            a.wrapping_add(f(b, c, d)).wrapping_add(K[i]).wrapping_add(m[g]), 
            S[i]));

        temp = d;
        d    = c;
        c    = b;
        b    = new_b;
        a    = temp;
    }

    (a, b, c, d)
}

    /* FONCTION PRINCIPALE
      *********************/

pub fn md5(bytes : &[u8]) -> (u32, u32, u32, u32)   {
    let padded = padding(bytes);
    let slices = slicing(&padded);

    let mut state = INIT_STATE;

    for slice in slices {
        state = tuple_add(state, main_loop(state, slice));
    }

    state
}

    /* TESTS
      *******/

#[cfg(test)]
mod tests   {
    #[test]
    fn md5()    {
        let vec = vec![
            0x2d, 0x00, 0x00, 0x00,
            0x01,
            0x00, 0x00, 0x00,
            0x04, 0x00, 0x00, 0x00,
            0x16, 0x00, 0x02, 0x00,
              0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
              0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,    
            0x0b, 0x00, 0x03, 0x00,
              0x0a, 0x00, 0x00,
              0x0a, 0x01, 0x00,
              0x30
        ];

        let m = super::md5(&vec);

        assert_eq!((0xcb2de7c3, 0x2015c523, 0x646219ae, 0x9d8a9a5b), m);
    }
} // End of tests.
