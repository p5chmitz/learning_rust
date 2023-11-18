enum IpAddrKind {
    v4,
    v6,
}

fn route(ip_kind: IpAddrKind){
}
let home = IpAddrKind::v4(String::from("Not a real thing"));
let away = IpAddrKind::v6(23);

fn main() {
    let four = IpAddrKind::v4;
    route(four);

}
