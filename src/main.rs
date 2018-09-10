extern crate itertools;
use itertools::Itertools;
use std::fmt;

fn hd(x: u8, y: u8) -> u32 {
    (x ^ y).count_ones()
}

#[test]
fn test_hd() {
    assert_eq!(hd(0b111, 0b000), 3);
    assert_eq!(hd(0b111, 0b001), 2);
    assert_eq!(hd(0b110, 0b001), 3);
    assert_eq!(hd(0b1110, 0b1001), 3);
}

/// Find the set of hamming distances (and the minimum hamming distance) for a given list of values
fn hd_for_set(vals: &[u8]) -> (u8, Vec<u8>, Vec<u8>)
{
    let mut hds = vec![0u8; vals.len() * vals.len()];
    let mut hd_cts = vec![0u8; 8];
    let mut min_hd = 0xf;

    // calculate hds for all pairs
    // track the minimum hd for this set of vals
    // count the number of instances of each HD
    for ((an, a), (bn, b)) in vals.iter().enumerate().tuple_combinations() {
        let chd = hd(*a, *b) as u8;
        hds[an * vals.len() + bn] = chd;
        hd_cts[chd as usize] += 1;

        if chd < min_hd {
            min_hd = chd;
        }
    }

    (min_hd, hd_cts, hds)
}

#[test]
fn test_hd_for_set() {
    //  left: `(1, [0, 1, 2, 0, 0, 3, 0, 0, 0])`,
    assert_eq!(hd_for_set(&[0b101, 0b111, 0b000]),
        (1, vec![0,1,1,1,0,0,0,0],
         vec![
        //          0b101,0b111,0b000
        /* 0b101 */    0, 1, 2,
        /* 0b111 */    0, 0, 3,
        /* 0b000 */    0, 0, 0,
         ]));
}

struct BinFmt<'a, B: fmt::Binary + 'a> {
    bit_width: u8,
    base: &'a [B],
}

impl<'a, B: fmt::Binary + 'a> fmt::Display for BinFmt<'a, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[" )?;
        for i in self.base {
            write!(f, "{:#01$b},", i, self.bit_width as usize + 2)?;
        }
        write!(f, "]")
    }
}

fn generate(sym_max: u8, needed_codes: usize)
{
    let symbols = 0..=sym_max;

    let mut best = vec![];
    let mut curr_min_hd = 0;

    let symbols = symbols.combinations(needed_codes);

    for i in symbols {
        let candidate = hd_for_set(&i);
        if candidate.0 > curr_min_hd {
            curr_min_hd = candidate.0;
            best.clear();
            best.push((i, candidate));
        } else if candidate.0 == curr_min_hd {
            best.push((i, candidate));
        } else {
            // not good enough, discard
        }
    }

    println!("{} candidates with HD({}):", best.len(), curr_min_hd);

    best.sort_by_key(|x| ((x.1).1).to_owned());

    for (vals, (_min_hd, hd_cts, _hd_table)) in best {
        println!("= {}", BinFmt {
            bit_width: (u8::max_value().count_ones() - sym_max.leading_zeros()) as u8,
            base: &vals[..]
        });

        println!(" > {:?}", hd_cts);
    }
}

fn main() {
    // TODO: "given minimum codes N, determine the combinations that 
    //       maximize number of potential codes without reducing HD"

    // Example:
    //  min_codes = 5; sym_max = 0xf
    //  -> max min HD == 2
    //  -> max codes with HD(2) == 8

    let sym_max = 0b1111u8;
    let needed_codes = 8;

    generate(sym_max, needed_codes)
}
