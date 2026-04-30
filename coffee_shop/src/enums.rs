#[derive(Debug,Clone,PartialEq)]
pub enum CoffeeType{
    Espresso,
    Latte,
    Cappuccino,
    Americano,
    Mocha,
}

impl CoffeeType{
    pub fn base_price_cents(&self) -> u32 {
        match self {
            CoffeeType::Espresso => 250,
            CoffeeType::Latte => 300,
            CoffeeType::Cappuccino => 400,
            CoffeeType::Americano => 380,
            CoffeeType::Mocha => 450,
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Size{
    Small,
    Medium,
    Large,
}

impl Size{
    pub fn price_modifier_cents(&self) -> u32 {
        match self {
            Size::Small => 0,
            Size::Medium => 50,
            Size::Large => 100,
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Milk{
    Whole,
    Skim,
    Oat,
    Almond,
    None,
}

#[derive(Debug,Clone,PartialEq)]
pub enum SyrupFlavor{
    Vanilla,
    Caramel,
    Hazelnut,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Extra{
    ExtraShot,
    Decaf,
    Syrup(SyrupFlavor),
}

impl Extra{
    pub fn price_cents(&self) -> u32 {
        match self {
            Extra::ExtraShot => 75,
            Extra::Decaf => 0,
            Extra::Syrup(_) => 50,
        }
    }
}

#[derive(Debug)]
pub struct OrderDetails{
    pub coffee: CoffeeType,
    pub size: Size,
    pub milk: Milk,
    pub extra: Vec<Extra>,
    pub customer_name: String
}

impl OrderDetails{
    pub fn total_price_cents(&self) -> u32{
        let base:u32 = self.coffee.base_price_cents();
        let size_modifier = self.size.price_modifier_cents();
        let extras: u32 = self.extra.iter().map(|e:&Extra| e.price_cents()).sum();
        base + size_modifier + extras
    }
}

