struct Position{
    id: i32,
    name: String,
    wage: rust_decimal::Decimal,
}
struct Worker{
    id: i32,
    full_name: String,
    hire_date: chrono::NaiveDateTime,
    position: Position,
    is_fired: bool,
}
struct Salary{
    id: i32,
    worker: Worker,
    salary_size: rust_decimal::Decimal,
    salary_date: chrono::NaiveDateTime,
}
struct Equipment{
    id: i32,
    inv_number: String,
    maintenance_date: chrono::NaiveDateTime,
    worker: Worker,
}
struct Material{
    id: i32,
    name: String,
    quantity: i32,
    category: String
}
struct RecipeElement{
    id:i32,
    material: Material,
    equipment: Equipment,
    quantity: i32
}
struct Recipe{
    id: i32,
    name: String,
    elements: std::vec::Vec<RecipeElement>,
}
struct Client{
    id: i32,
    name: String,
    address: String
}
struct Order<'a>{
    id: i32, 
    client: Client,
    order_date: chrono::NaiveDateTime,
    total_amount: rust_decimal::Decimal,
    elements: std::vec::Vec<OrderElement<'a>>
}
struct OrderElement<'a>{
    id: i32,
    product: Product<'a>,
    quantity: i32,
    total_amount: rust_decimal::Decimal
}   
struct Product<'a>{
    id: i32,
    recipe: &'a Recipe,
    volume: f64,
    price_per_unit: rust_decimal::Decimal,

}