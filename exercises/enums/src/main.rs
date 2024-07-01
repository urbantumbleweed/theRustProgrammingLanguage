#[derive(Debug)]
struct Address {
    location: String,
    port: String
}

impl Address {
    fn new(location: &str) -> Address {
        Address {
            location: location.to_string(),
            port: String::from(""),
        }
    }
}



#[derive(Debug)]
enum IpAddrKind {
    V4(Address),
    V6(Address),
}

impl IpAddrKind {
    fn call(&self) {
        match self {
            IpAddrKind::V4(address) => println!("{:#?}", address),
            IpAddrKind::V6(address) => println!("{:#?}", address),
        }
    }
}

fn main() {
    
    let addr: IpAddrKind = IpAddrKind::V4(Address::new("127.0.0.1"));
    let addr2: IpAddrKind = IpAddrKind::V6(Address::new("::1"));

    print_address(&addr);
    print_address(&addr2);

    addr.call();
    addr2.call();
}

fn print_address(addr: &IpAddrKind) {
    match addr {
        IpAddrKind::V4(a) => println!("V4: {:#?}", a.location),
        IpAddrKind::V6(a) => println!("V6: {:#?}", a.location),
    };
}
