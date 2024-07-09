fn print_out_item(item: &Vec<i32>) {
    for item in item.iter() {
        println!("{:?}", item);
    }
}

fn main() {
    let _item = vec![1, 2, 3];

    print_out_item(&_item);
    print_out_item(&_item);
}
