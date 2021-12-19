use itertools::Itertools;

#[derive(Default)]
struct Decoder {
    bits: Vec<char>,
    offset: usize,
    versions: Vec<usize>,
    values: Vec<usize>
}

trait Converter {
    fn to_decimal(&self) -> usize;
}

impl Converter for Vec<char> {
    fn to_decimal(&self) -> usize {
        let b = self.iter().collect::<String>();
        usize::from_str_radix(&b, 2).unwrap()
    }
}

impl Decoder {
    fn new(data: &str) -> Self {
        let bits = data
            .chars()
            .into_iter()
            .map(Self::char_to_binary)
            .map(|s| s.chars())
            .flatten()
            .collect_vec();

        Self::new_from_bits(bits)
    }

    fn new_from_bits(bits: Vec<char>) -> Self {
        Decoder {
            bits,
            ..Default::default()
        }
    }

    fn decode(&mut self) {
        let pkt_ver = self.take_next(3).to_decimal();
        self.versions.push(pkt_ver);
        let pkt_type = self.take_next(3).to_decimal();
        println!("pkg type {}", pkt_type );

        match pkt_type {
            4 => {
                let val = self.literal_pkt();
                self.values.push(val);
            }
            _ => {
                self.operator_pkt();
            }
        };
    }

    fn operator_pkt(&mut self) {
        let length = self.take_next(1).to_decimal();
        match length {
            0 => {
                let sub_pkt_len = self.take_next(15).to_decimal();
                let new_pkt = self.take_next(sub_pkt_len);
                let mut new_decoder = Decoder::new_from_bits(new_pkt);
                while new_decoder.offset < sub_pkt_len {
                    new_decoder.decode();
                }
                self.versions.push(new_decoder.sum_versions());
                self.values.append(&mut new_decoder.values);
            }
            1 => {
                let sub_pkt_num = self.take_next(11).to_decimal();
                for _ in 0..sub_pkt_num {
                    self.decode();
                }
            }
            _ => unreachable!(),
        }
    }

    fn literal_pkt(&mut self) -> usize {
        let mut vals = vec![];
        loop {
            let next = self.take_next(5);
            vals.push(next[1..next.len()].to_owned());
            if next[0] == '0' {
                break;
            }
        }
        vals.into_iter().flatten().collect_vec().to_decimal()
    }

    fn take_next(&mut self, next: usize) -> Vec<char> {
        let from = self.offset;
        self.offset += next;
        self.bits[from..self.offset].to_owned()
    }

    fn sum_versions(&self) -> usize {
        self.versions.iter().sum()
    }

    fn char_to_binary(c: char) -> &'static str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_16::Decoder;

    #[test]
    fn should_decode_test_data() {
        let mut dec = Decoder::new("020D790050D26C13EC1348326D336ACE00EC299E6A8B929ED59C880502E00A526B969F62BF35CB4FB15B93A6311F67F813300470037500043E2D4218FA000864538E905A39CAF77386E35AB01802FC01BA00021118617C1F00043A3F4748A53CF66008D00481272D73308334EDB0ED304E200D4E94CF612A49B40036C98A7CF24A53CA94C6370FBDCC9018029600ACD529CA9A4F62ACD2B5F928802F0D2665CA7D6CC013919E78A3800D3CF7794A8FC938280473057394AFF15099BA23CDD37A08400E2A5F7297F916C9F97F82D2DFA734BC600D4E3BC89CCBABCBE2B77D200412599244D4C0138C780120CC67E9D351C5AB4E1D4C981802980080CDB84E034C5767C60124F3BC984CD1E479201232C016100662D45089A00087C1084F12A724752BEFEA9C51500566759BF9BE6C5080217910CD00525B6350E8C00E9272200DCE4EF4C1DD003952F7059BCF675443005680103976997699795E830C02E4CBCE72EFC6A6218C88C9DF2F3351FCEF2D83CADB779F59A052801F2BAACDAE7F52A8190073937FE1D700439234DBB4F7374DC0CC804CF1006A0D47B8A4200F445865170401F8251662D100909401AB8803313217C680004320D43F871308D140C010E0069E7EDD1796AFC8255800052E20043E0F42A8B6400864258E51088010B85910A0F4ECE1DFE069C0229AE63D0B8DC6F82529403203305C00E1002C80AF5602908400A20240100852401E98400830021400D30029004B6100294008400B9D0023240061C000D19CACCD9005F694AEF6493D3F9948DEB3B4CC273FFD5E9AD85CFDFF6978B80050392AC3D98D796449BE304FE7F0C13CD716656BD0A6002A67E61A400F6E8029300B300B11480463D004C401889B1CA31800211162204679621200FCAC01791CF6B1AFCF2473DAC6BF3A9F1700016A3D90064D359B35D003430727A7DC464E6401594A57C93A0084CC56A662B8C00AA424989F2A9112");
        dec.decode();
        assert_eq!(906, dec.sum_versions())
    }

    #[test]
    fn should_decode_part_2() {
        let mut dec = Decoder::new("04005AC33890");
        dec.decode();
        println!("{:?}", dec.values);
    }
}
