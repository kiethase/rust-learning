#![allow(non_snake_case)]


// Use T and U so that we can make x and y ez to change type.
struct Point <T,U>{
    x: T,
    y: U,
}
// Mixed up impl and fn to make it more define type 
impl<T,U> Point<T,U>{
    fn mixup<V,W>(self,other: Point<V,W>) -> Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}


struct Point2<T>{
    x:T,
    y:T,
}

impl<U> Point2<U>{
    fn x(&self) -> &U{
        &self.x
    }
}

fn main() {
    let number_list = vec![11,12,13,15,22,21,67];
    let largest_number = fun_get_largest_type(number_list);
    println!("The largest number is {}", largest_number);

    //We will have a error when use character with i32 type
    // if we duplicate the fun get_largest_number it so bad => duplicate 
    // we use generic for that 
    let character_list = vec!['a','b','c','o','g'];
    let largest_character = fun_get_largest_type(character_list);

    println!("The largest number is {}", largest_character);
//--------------------
//We can use generic to make struct hve ez to define type 
    let p1 = Point{x:5, y:7};
    let p2 = Point{x:5.0, y:10.0};
    let p3 = Point{x:5, y:10.0};

//--------------------
// We can use generic into enum so that we can control the type.
    enum Option<T>{
        Some(T),
        None,
    }

    enum Result <T,E>{
        Some(T),
        Err(E),
    }

}

// Use the T to define  Type for value so that use can compare them to get largest value 
fn fun_get_largest_type<T: PartialOrd + Copy> (number_list: Vec<T>) -> T {
    let mut largest_number = number_list[0];
    //Loop create to find largest
    for number in number_list{
        if number > largest_number {
            largest_number = number;
        }
    }
    largest_number
}
