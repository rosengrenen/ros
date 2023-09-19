use acpi::aml::term_list;

fn main() {
    let aml = include_bytes!("../dsdt.aml");
    let aml = &aml[36..];
    let _ = term_list(aml);
    // println!("{:x?}", aml);
    println!("hello");
}
