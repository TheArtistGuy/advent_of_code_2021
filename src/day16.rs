use std::fs;
use std::path::Path;

pub(crate) fn day16(){
    let data = fs::read_to_string(Path::new("resources/day16_data")).expect("could not ope file");
    let (_, version_sum, value ) = decode(0, &read_to_binary(&data));
    println!("Day 16 , 1 : {}", &version_sum);
    println!("Day 16 , 2 : {}", &value);
}


fn decode(start :usize, bin: &Vec<u8>) -> (usize, u32, u64) {
    let version = bin_to_dec(&bin[start..start+3]);
    let type_id = bin_to_dec(&bin[start + 3.. start + 6]);
    let mut pos = start + 6;
    let (akt_pos, sum, value) = match type_id{
        4 => {
            //is literal
            let mut value = 0;
            while bin[pos] == 1{
                value = (value <<4) +  bin_to_dec(&bin[pos+1..pos+5]) as u64;
                pos += 5;
            }
            //is last
            value = (value <<4) +  bin_to_dec(&bin[pos+1..pos+5]) as u64;
            pos += 5;
            (pos, version, value)
        }
        0 => {
            //is sum packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut value = 0;
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    value += val;
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    value += val;
                    version_sum += vers_sum_subpacket;
                }
            }
            (pos, version_sum, value)
        }
        1 => {
            //is product packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut value = 1;
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    value = value * val;
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    value = value * val;
                    version_sum += vers_sum_subpacket;
                }
            }
            (pos, version_sum, value)
        }
        2 => {
            //is minimum packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut values = Vec::new();
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
                    version_sum += vers_sum_subpacket;
                }
            }
            let mut min = u64::MAX;
            for x in values.iter(){ if x<&min{min = *x}};
            (pos, version_sum, min)
        }
        3 => {
            //is maximum packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut values = Vec::new();
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
                    version_sum += vers_sum_subpacket;
                }
            }
            let mut max = 0;
            for x in values.iter(){
                if x>&max { max = *x};
            }
                (pos, version_sum, max)
        }
        5 => {
            //is greater packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut values = Vec::new();
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
                    version_sum += vers_sum_subpacket;
                }
            }

            let val = if values[0] > values[1] {1} else {0};
            (pos, version_sum, val)
        }
        6 => {
            //is smaller than packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut values = Vec::new();
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
                    version_sum += vers_sum_subpacket;
                }
            }

            let val = if values[0] < values[1] {1} else {0};
            (pos, version_sum, val)
        }
        7 => {
            //is equal packet
            let mode = bin[pos];
            pos += 1;
            let mut version_sum = version;
            let mut values = Vec::new();
            if mode == 0 {
                //determine subpackets by length
                let total_length = bin_to_dec(&bin[pos..pos+15]) as usize;
                pos += 15;
                let start_pos = pos;
                let mut not_reached = true;
                while not_reached {
                    let (new_pos, vers_sum, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
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
                    let (new_pos, vers_sum_subpacket, val) = decode(pos, &bin);
                    pos = new_pos;
                    values.push(val);
                    version_sum += vers_sum_subpacket;
                }
            }

            let val = if values[0] == values[1] {1} else {0};
            (pos, version_sum, val)
        }
        _ => {
            print!("case reached which should not be reached");
            (0, 0, 0)
        }
    };
    (akt_pos, sum, value)
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
        let (pos,sum, _) = decode(0, &binary);
        assert_eq!(sum, 16);
        let (_, _, test_sum) = decode(0, &read_to_binary(&"C200B40A82".to_string()));
        assert_eq!(test_sum, 3);
        let (_, _, test_product) = decode(0, &read_to_binary(&"04005AC33890".to_string()));
        assert_eq!(test_product, 54);
        let (_, _, test_max) = decode(0, &read_to_binary(&"CE00C43D881120".to_string()));
        assert_eq!(test_max, 9);
        let (_, _, test_min) = decode(0, &read_to_binary(&"880086C3E88112".to_string()));
        assert_eq!(test_min, 7);
        let (_, _, test_less) = decode(0, &read_to_binary(&"D8005AC2A8F0".to_string()));
        assert_eq!(test_less, 1);
        let (_, _, test_greater) = decode(0, &read_to_binary(&"F600BC2D8F".to_string()));
        assert_eq!(test_greater, 0);
        let (_, _, test_eq) = decode(0, &read_to_binary(&"9C005AC2F8F0".to_string()));
        assert_eq!(test_eq, 0);
    }


}