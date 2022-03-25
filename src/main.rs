use std::net::{Ipv4Addr, Ipv6Addr};

fn transform_ipv4(ipv4: &String) -> String {
    let decimal = u32::from_be(ipv4.parse::<u32>().unwrap());
    let ipv4 = Ipv4Addr::from(decimal);
    format!("{}", ipv4)
}

fn transform_ipv6(ipv6: &String) -> String {
    let split = ipv6.replace("{", "").replace("}", "");
    let split =  split.split(", ");

    let mut decimal: u128 = 0;
    // crash 这个软件真坑，字节序的反的，32位字也是反的
    for (i, m) in split.enumerate() {
        let m = m.parse::<u32>().unwrap();
        let m: u128 = m as u128;
        decimal += m << (i*32) ;
    }
    let v6 = Ipv6Addr::from(u128::from_be(decimal));
    format!("{}", v6)
}

fn transform_port(port: &String) -> u16 {
    u16::from_be(port.parse::<u16>().unwrap())
}

#[test]
fn test_port() {
    assert_eq!(transform_port(&String::from("59400")), (2280 as u16));
}

#[test]
fn test_ipv4() {
    assert_eq!(transform_ipv4(&"402657452".to_string()), "172.16.0.24")
}

#[test]
fn test_ipv6() {
    assert_eq!(transform_ipv6(&"{5112356, 9377040, 3281321984, 2867013992}".to_string()),
               "2402:4e00:1015:8f00:0:95c3:6829:e3aa");
}


fn main() {
    let transform_type = std::env::args().nth(1).expect("no such transform type");
    let context = std::env::args().nth(2).expect("no data");

    if transform_type == "p" {
        println!("port: {}", transform_port(&context));
    }else if transform_type == "4"{
        println!("ipv4: {}", transform_ipv4(&context));
    }else if transform_type == "6"{
        println!("ipv6: {}", transform_ipv6(&context));
    }
}
