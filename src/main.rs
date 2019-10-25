mod str_printer;
mod a_struct;

fn main() {
    str_printer::print_string("Test");

    let my_struct = a_struct::AStruct { a_string:"Test", four_bytes:0xFFFFFFFF };

    println!("my_struct: {}", my_struct.to_string());
}
