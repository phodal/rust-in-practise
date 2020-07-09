use ptlv::tlv::TlvBox;

fn main() {
    println!("Hello, world!");
    let mut tlv_box = TlvBox::new();
    tlv_box.put_string_value(01, String::from("Hello, world!"));
    tlv_box.serialize();
}
