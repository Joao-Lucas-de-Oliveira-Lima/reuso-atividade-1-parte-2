use::restaurant::constants::PI;
use:: restaurant::back_of_house::take_care_trash;
use:: restaurant::front_of_house::hosting::add_to_wait_list;

fn main() {
    println!("Valor de PI: {}", PI);
    take_care_trash();
    add_to_wait_list();
}
