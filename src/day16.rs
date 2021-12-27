use std::fs;
use std::path::Path;

pub(crate) fn day16(){
    let data = fs::read_to_string(Path::new("resources/day16_data")).expect("could not ope file");
    let (_, version_sum ) = decode(0, &read_to_binary(&data));
    println!("Day 16 , 1 : {}", version_sum);
}


fn decode(start :usize, bin: &Vec<u8>) -> (usize, u32) {
    let version = bin_to_dec(&bin[start..start+3]);
    let type_id = bin_to_dec(&bin[start + 3.. start + 6]);
    let mut pos = start + 6;
    let (akt_pos, sum) = match type_id{
        4 => {
            //is literal
            let mut x = 0;
            while bin[pos] == 1{
                x = (x<<4) +  bin_to_dec(&bin[pos+1..pos+5]);
                pos += 5;
            }
            //is last
            x = (x<<4) +  bin_to_dec(&bin[pos+1..pos+5]);
            pos += 5;
            (pos, version)
        }
        _ => {
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum) = decode(pos, &bin);
                    pos = new_pos;
                    version_sum += vers_sum;
                    if pos - start_pos >= total_length {
                        not_reached = false;
                    }
                }
            }else{
                //determine subpackets by number
                let sub_packets = bin_to_dec(&bin[pos..pos+11]);
                pos += 11;
                for _ in 0..sub_packets{
                    let (new_pos, vers_sum_subpacket) = decode(pos, &bin);
                    pos = new_pos;
                    version_sum += vers_sum_subpacket;
                }
            }
            (pos, version_sum)
        }
    };
    (akt_pos, sum)
}

fn bin_to_dec(bin: &[u8]) -> u32 {
    let max = bin.len() as u32 -1;
    let mut res : u32 = 0;
    for (i, x) in bin.iter().enumerate(){
        res += (*x as u32) << max-i as u32;
    }
    res
}

fn read_to_binary(input: &String) -> Vec<u8> {
    let x: Vec<u8> = input.chars().map(|y| u8::from_str_radix(&y.to_string(), 16).unwrap()).collect();
    let mut v = Vec::new();
    for y in x{
        for i in 0..4{
            v.push((y.clone() >>(3-i) )% 2);
        }
    }
    v
}

#[cfg(test)]
mod test {
    use crate::day16::{bin_to_dec, decode, read_to_binary};

    #[test]
    fn test_day17() {
        let data = "8A004A801A8002F478".to_string();
        let binary = read_to_binary(&data);
        assert_eq!(binary.len(), data.len() *4);
        let start = 0;
        let version = bin_to_dec(&binary[start..start+3]);
        let id = bin_to_dec(&binary[start+3..start+6]);
        assert_eq!(version, 4);
        let (pos,sum) = decode(0, &binary);
        assert_eq!(sum, 16);

    }


}