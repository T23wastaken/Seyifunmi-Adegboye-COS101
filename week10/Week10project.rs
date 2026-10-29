
struct LaptopRecord {
    brand: String,
    quantity: u32,
    price: u32,
}


fn main() {
    let HP = LaptopRecord{
        brand: String::from("HP"),
        quantity: 10,
        price: 650000,
    };

      let IBM = LaptopRecord{
        brand: String::from("IBM"),
        quantity: 6,
        price: 755000,
    };

      let Toshiba = LaptopRecord{
        brand: String::from("Toshiba"),
        quantity: 10,
        price: 550000,
    };

      let Dell = LaptopRecord{
        brand: String::from("Dell"),
        quantity: 4,
        price: 850000,
       };

let purchaseamt = 3;
  
    
    let total = purchaseamt * HP.price + purchaseamt * IBM.price + purchaseamt * Toshiba.price + purchaseamt * Dell.price;

    println!("Total Cost: N{}",total);

  }
