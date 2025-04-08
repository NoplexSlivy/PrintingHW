enum Class{
    Math,
    English,
    Science,
    History,
}

struct Homework{
    class: Class,
    assignment: String,
    daysdue: usize,
}

fn main(){
    let hw1 = Homework {
        class: Class::Math,
        assignment: String::from("Algebra"),
        daysdue: 3,
    };

    let hw2 = Homework {
        class: Class::English,
        assignment: String::from("Essay"),
        daysdue: 7,
    };

    let hw3 = Homework {
        class: Class::Science,
        assignment: String::from("Lab Report"),
        daysdue: 5,
    };

    let hw4 = Homework {
        class: Class::History,
        assignment: String::from("Research Paper"),
        daysdue: 10,
    };


    let homW = vec![hw1,hw2,hw3,hw4];
    for x in homW{

        println!("the {} assignment, in {} class, is due in {} days" , x.assignment, x.class, x.daysdue); 
    }
    

}