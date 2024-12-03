use restaurante1::front_of_house::hosting;
use restaurante1::front_of_house::serving;

fn main() {
    hosting::add_to_wait_list();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
    println!("The value of PI is: {}", restaurante1::PI);
}
