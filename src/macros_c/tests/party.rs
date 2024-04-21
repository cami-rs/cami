use crate::c_partial_eq;
use crate::Locality;
use alloc::string::String;

struct Empty {}
c_partial_eq! {
    Empty {
        Locality::Both
    }
    []
    []
    []
}

type Amount = u16;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Food {
    name: String,
    amount: Amount,
}
impl Food {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

pub struct FoodList {
    common: Food,
    gluten_free: Food,
    dairy_free: Food,
    vegan: Food,
}
impl FoodList {
    pub fn common(&self) -> &Food {
        &self.common
    }
    pub fn gluten_free(&self) -> &Food {
        &self.gluten_free
    }
    pub fn dairy_free(&self) -> &Food {
        &self.dairy_free
    }
    pub fn vegan(&self) -> &Food {
        &self.vegan
    }
}

pub struct Table(Food, Food);

pub struct Room(Table, Table);

c_partial_eq! {
    Food {
        Locality::Both
    }
    [amount]
    [ (|this: &Food, other: &Food| this.name==other.name) ]
    []
}
// @TODO Food, but using eq_local & eq_non_local from String .name

c_partial_eq! {
    FoodList {
        Locality::Both
    }
    [
        .common.amount,
        {|food_list: &FoodList| food_list.gluten_free.amount},
        .dairy_free.amount(),
        (|this: &FoodList, other: &FoodList| this.vegan.amount==other.vegan.amount)
    ]
    [   common.name,
        gluten_free.name(),
        dairy_free().name,
        (|this: &FoodList, other: &FoodList| this.vegan.name==other.vegan.name)
    ]
    []
}
c_partial_eq! {
    Table {
        Locality::Both
    }
    [
        .0.amount,
        {|table: &Table| table.1.amount}
    ]
    [   .0.name(),
        (|this: &Table, other: &Table| this.1.name==other.1.name)
    ]
    []
}
c_partial_eq! {
    Room {
        Locality::Both
    }
    [
        .0.0.amount,
        {|room: &Room| room.0.1.amount},
        (|this: &Room, other: &Room| this.1.0.amount==other.1.0.amount),
        (|this: &Room, other: &Room| this.1.0.eq_local(&other.1.0))
    ]
    [   .0.0.name,

        (|this: &Room, other: &Room| this.0.1.name==other.0.1.name)
    ]
    []
}
