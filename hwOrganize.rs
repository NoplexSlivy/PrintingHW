enum Classes{
    Math,
    English,
    Science,
    History,
}

struct Homework{
    class: Classes,
    assignment: String,
    daysdue: usize,
}

fn main(){
    let hw1 = &Homework {
        class: Classes::Math,
        assignment: String::from("Algebra"),
        daysdue: 3,
    };

    let hw2 = &Homework {
        class: Classes::English,
        assignment: String::from("Essay"),
        daysdue: 7,
    };

    let hw3 = &Homework {
        class: Classes::Science,
        assignment: String::from("Lab Report"),
        daysdue: 5,
    };

    let hw4 = &Homework {
        class: Classes::History,
        assignment: String::from("Research Paper"),
        daysdue: 10,
    };
    let homW = vec![&hw1,&hw2,&hw3,&hw4];
    for x in homW{
        println!("the {} assignment, in {} class, is due in {} days" , x.assignment, x.class, x.daysdue); 
    }
}