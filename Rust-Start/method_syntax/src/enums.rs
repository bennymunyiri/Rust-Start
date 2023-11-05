enum IpkindAddress{
    v4(String),
    v6(String),
}

let home = IpkindAddress::v4(String::from("127.8.0.1"));
let loopback = IpkindAddress::v6(String::from("::01"));









enum IpkindAddress{
    v4,
    v6
}


struct Ipkind{
    kind: IpkindAddress,
    address: String,
}

let firstip = Ipkind{
    kind: IpkindAddress::v4,
    address: String::from("128.0.0.1"),
};

let secondip = Ipkind{
    kind: IpkindAddress::v6,
    address: String::from("::01"),
};