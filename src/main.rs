#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    Paypal { username: String, password: String },
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl(Meat),
    VeganPlate,
}

#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

#[derive(Debug)]
enum LaundaryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundaryCycle {
    fn wash_laundary(&self) {
        match self {
            LaundaryCycle::Cold => {
                println!("Running the laundary with cold temparature");
            }
            LaundaryCycle::Hot { temperature } => {
                println!("Runing the laundary with a temparature of {temperature}");
            }
            LaundaryCycle::Delicate(fabrice_type) => {
                println!("Running a laundary eith delicate cycle for {fabrice_type}");
            }
        }
    }
}

#[derive(Debug)]
enum OnlineOrdersStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrdersStatus {
    fn check(&self) {
        match self {
            OnlineOrdersStatus::Ordered | OnlineOrdersStatus::Packed => {
                println!("Your item is beign prepped for shipment.");
            }
            OnlineOrdersStatus::Delivered => {
                println!("Your item has been delivered");
            }
            other_status => {
                println!("your item is {other_status:?}");
            }
        }
    }
}

enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favorite");
            }
            Milk::Lowfat(percent) => {
                println!(" you gave got the lowfat {percent} percent version!")
            }
            Milk::Whole => {
                println!("You have got thge whole milk!");
            }
            _ => {
                println!("none");
            }
        }
    }
}

fn main() {
    let my_beverage = Milk::Lowfat(2);

    let Milk::Lowfat(percent) = my_beverage else {
        println!("you do not have the low fat milk");
        return;
    };
    
    println!("{percent}% milk is avaliable later");

    if let Milk::Lowfat(percent) = my_beverage {
        println!("your beverage is {percent}% milk");
    } else {
         println!("you beverage is osmething else!");
    }

    Milk::Lowfat(1).drink();
    Milk::Lowfat(2).drink();

    OnlineOrdersStatus::Delivered.check();
    OnlineOrdersStatus::Ordered.check();

    //LaundaryCycle::Cold.wash_laundary();
    // let hot_cycle = LaundaryCycle::Hot { temperature: 84 };
    // hot_cycle.wash_laundary();

    // let delicate_cycle = LaundaryCycle::Delicate(String::from("Cotton"));
    // delicate_cycle.wash_laundary();

    let lunch = RestaurantItem::Burrito {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let dinner = RestaurantItem::Bowl(Meat::Steak);

    println!("{:?}", lunch);
    println!("{:?}", dinner);

    let mut my_payment_method =
        PaymentMethodType::CreditCard(String::from("0000-2222-4444-555500"));

    let paypal_credentials = Credentials {
        username: String::from("asd@qwe.com"),
        password: String::from("12345678"),
    };

    let paypal = PaymentMethodType::Paypal {
        username: String::from("abc@qwe.com"),
        password: String::from("asdfgghh"),
    };
    println!("{:?}", paypal);

    println!("{:?}", my_payment_method);

    let visa = PaymentMethodType::CreditCard(String::from("0000-1111-2222-3333"));
    let mastercard = PaymentMethodType::DebitCard(String::from("1111-2222-3333-4444"));

    println!("{:?}", visa);
    println!("{:?}", mastercard);

    let first_card = CardSuit::Hearts;

    let mut second_card = CardSuit::Diamonds;
    second_card = CardSuit::Clubs;

    let card_suits = [CardSuit::Clubs, CardSuit::Diamonds];

    let card_suits_tuple = (CardSuit::Clubs, CardSuit::Diamonds);

    println!("{:?}", first_card);

    let my_computer = OperatingSystem::MacOS;
    let age = years_since_release(my_computer);
    println!("{} years", age);

    // wash_laundary(LaundaryCycle::Cold);
    // wash_laundary(LaundaryCycle::Hot { temperature: 100 });
    // wash_laundary(LaundaryCycle::Delicate(String::from("silk")));
}

fn wash_laundary(cycle: LaundaryCycle) {
    match cycle {
        LaundaryCycle::Cold => {
            println!("Running the laundary with cold temparature");
        }
        LaundaryCycle::Hot { temperature } => {
            println!("Runing the laundary with a temparature of {temperature}");
        }
        LaundaryCycle::Delicate(fabrice_type) => {
            println!("Running a laundary eith delicate cycle for {fabrice_type}");
        }
    }
}

fn years_since_release(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Windows => {
            println!("Quite an old computer");
            32
        }
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}
