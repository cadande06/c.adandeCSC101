struct Supply{
    hp:u32,
    ibm:u32,
    toshiba:u32,
    dell:u32
}

struct Prices{
    hp:u64,
    ibm:u64,
    toshiba:u64,
    dell:u64
}


impl Prices{
    fn total_cost(&self)->u64{
        return 3*(self.hp + self.ibm + self.toshiba + self.dell);
    }
}

fn main() {
    let supply = Supply{
        hp:10,
        ibm:6,
        toshiba:10,
        dell:4
    };

    let prices = Prices{
        hp:650_000,
        ibm:755_000,
        toshiba:550_000,
        dell:850_000
    };

    println!("Welcome to Alaba International Market. We have a variety of laptops:\n");
    println!("{} HP laptops at N{}.0 each",supply.hp,prices.hp);
    println!("{} IBM laptops at N{}.0 each",supply.ibm,prices.ibm);
    println!("{} Toshiba laptops at N{}.0 each",supply.toshiba,prices.toshiba);
    println!("{} Dell laptops at N{}.0 each\n\n",supply.dell,prices.dell);

    println!("Your order:");
    println!("3 HP laptops");
    println!("3 IMB laptops");
    println!("3 Toshiba laptops");
    println!("3 Dell laptops");
    println!("\nThe total cost for this purchase is N{}.0",prices.total_cost());
    println!("\nThank you for shopping at Alaba International Market!");
}
