use std :: io;
    
// fn circle_area(r:f32)->f32{
//     const PI :f32 = 3.14;
//     PI*r*r
// }


// fn spher_vol(r:f32)->f32{
//     const PI :f32 = 3.14;
//     1.333*PI*r*r*r
// }


// fn num_check (num:i32)->String{
//     if num > 0 {
//         "positive number entered".to_string()
//     }
//     else if num < 0 {
//         "negative number is entered".to_string()
//     }
//     else {
//         "number is zero".to_string()
//     }
// }


// fn num_div_check (num_01:i32, num_02:i32)->String{
//     if num_01 % num_02 == 0 {
//         "number is completly divisible by ".to_string()
//     }
//     else {
//         "number is not compeletly divisible by ".to_string()
//     }
// }

//  fn string_copy (mut n:i32, text:String) {
//      while n !=0 {
//          n-=1;
//          print!("{}",text);
//      };

//  }

// fn parity_check (num:i32)->String{
//     if num % 2 == 0 {
//         "is even".to_string()
//     }
//     else {
//         "is odd".to_string()
//     }
// }

// fn vovel_test(letter:char){
//     match letter {
//         'a' | 'e' | 'i' | 'o' | 'u' => print!("{} is a vovel",letter),
//         'A' | 'E' | 'I' | 'O' | 'U' => print!("{} is a vovel",letter),
//         _                           => print!("{} is not a vovel",letter)
//     }
// }

// fn triangle_area(base:f32,height:f32)->f32{
//     (base*height)/2.0
// }

// fn distance (x1:f32,x2:f32,y1:f32,y2:f32)->f32{
//     (((x1-y1)*(x1-y1))+((x2-y2)*(x2-y2))).sqrt()
// }

// fn converter(feet:f32)->f32{
//     feet*34.48
// }

// fn bmi(weight:f32,height:f32)->f32{
//     let h=height/100.0;
//     weight/(h*h)
// }

// fn sum(mut n:i32)->i32{
//     let mut sum = 0;
//     while n != 0 {
//         sum = sum+n;
//         n-=1;
//     }
//     sum
// }

// fn bin_converter(mut num:i32)->i32{
//     let mut rem =0;
//     let mut bin = 0;
//     let mut pos = 1;
//     while num != 0 {
//         rem = num%2;
//         num = num/2;
//         bin = bin + (rem*pos);
//         pos = pos*10;
//     };
//     bin  }

// fn deci_converter(mut num:i32)->i32{
//     let mut rem =0;
//     let mut deci = 0;
//     let mut i = 0;
//     let a:i32 = 2;
//     while num != 0 {
//         rem = num%10;
//         num = num/10;
//         deci = deci + (a.pow(i))*rem;
//         i+=1;
//     };
//     deci  }

fn main() {
    // // area of a circle user input:
    // let mut radius = String::new();
    // io::stdin().read_line(&mut radius);
    // let radius:f32 = radius.trim().parse().unwrap();

    // //function call for area of a circle:
    // let area_circle = circle_area(radius);
    // println!("the area of a circle with radius {} is : {}",radius,area_circle);

//**************************************//
    // // number check user input:
    // println!("Enter number : ");
    // let mut number = String::new();
    // io::stdin().read_line(&mut number);
    // let number:i32 = number.trim().parse().unwrap();

    // //function call for number check:
    // let check_num = num_check(number);
    // println!("{}",check_num);

//**************************************//
    // // number divisiblety check inputs:
    // println!("Enter numerator : ");
    // let mut numerator = String::new();
    // io::stdin().read_line(&mut numerator);
    // let numerator:i32 = numerator.trim().parse().unwrap();

    // println!("Enter denominator : ");
    // let mut denominator = String::new();
    // io::stdin().read_line(&mut denominator);
    // let denominator:i32 = denominator.trim().parse().unwrap();


    // //function call for divisibility check:
    // let div_check = num_div_check(numerator,denominator);
    // println!("{} by {}",div_check, denominator);

//*************************************//
    // // area of a spher user input:
    // let mut radius = String::new();
    // io::stdin().read_line(&mut radius);
    // let radius:f32 = radius.trim().parse().unwrap();

    // //function call for area of a circle:
    // let vol_spher = spher_vol(radius);
    // println!("the area of a circle with radius {} is : {}",radius,vol_spher);

//**************************************//
    // // print test number of times user inputs:
    // println!("Enter String: ");
    // let mut text = String::new();
    // io::stdin().read_line(&mut text);
    // let text:String = text.trim().parse().unwrap();

    // println!("Enter how many copies of string you want: ");
    // let mut num = String::new();
    // io::stdin().read_line(&mut num);
    // let num:i32 = num.trim().parse().unwrap();

    // //function call to copy string:
    // string_copy(num,text);

//**************************************//
    // // parity check user input:
    // println!("Enter number : ");
    // let mut number = String::new();
    // io::stdin().read_line(&mut number);
    // let number:i32 = number.trim().parse().unwrap();

    // //function call for number check:
    // let parity = parity_check(number);
    // println!("{} {}",number,parity);

//**************************************//
    // // vovel tester user inputs:
    // println!("Enter character: ");
    // let mut letter = String::new();
    // io::stdin().read_line(&mut letter);
    // let letter:char = letter.trim().parse().unwrap();

    // //call to vovel tester function
    // vovel_test(letter);

//**************************************//
    // // collect base and height value from user inputs:
    // println!("Enter the megnitude of base: ");
    // let mut base = String::new();
    // io::stdin().read_line(&mut base);
    // let base:f32 = base.trim().parse().unwrap();

    // println!("Enter the megnitude of height: ");
    // let mut height = String::new();
    // io::stdin().read_line(&mut height);
    // let height:f32 = height.trim().parse().unwrap();

    // //print area of a triangle calling function
    // let area = triangle_area(base,height);
    // println!("the area of a triangle is: {}",area);

//**************************************//
    // // collect values of cordinates from user inputs:
    // println!("Enter cordinate for x1: ");
    // let mut x1 = String::new();
    // io::stdin().read_line(&mut x1);
    // let x1:f32 = x1.trim().parse().unwrap();

    // println!("Enter cordinate for x2: ");
    // let mut x2 = String::new();
    // io::stdin().read_line(&mut x2);
    // let x2:f32 = x2.trim().parse().unwrap();

    // println!("Enter cordinate for y1: ");
    // let mut y1 = String::new();
    // io::stdin().read_line(&mut y1);
    // let y1:f32 = y1.trim().parse().unwrap();

    // println!("Enter cordinate for y2: ");
    // let mut y2 = String::new();
    // io::stdin().read_line(&mut y2);
    // let y2:f32 = y2.trim().parse().unwrap();

    // let result = distance(x1,x2,y1,y2);
    // println!("distance between two points is: {}",result); 

//**************************************//
    // // // collect values in feet from user inputs:
    // println!("Enter height in feet: ");
    // let mut value = String::new();
    // io::stdin().read_line(&mut value);
    // let value:f32 = value.trim().parse().unwrap();

    //print covverted value calling function
    // let result = converter(value);
    // println!("There are {} Cm in {} ft",result,value);

//**************************************//
    // collect weight and hight from user inputs:
    // println!("Enter height in Cm: ");
    // let mut height = String::new();
    // io::stdin().read_line(&mut height);
    // let height:f32 = height.trim().parse().unwrap();

    // println!("Enter weight in Kg: ");
    // let mut weight = String::new();
    // io::stdin().read_line(&mut weight);
    // let weight:f32 = weight.trim().parse().unwrap();

    // let answer = bmi(weight,height);
    // println!("your BMI is {} kg/m2",answer);

//**************************************//
    // //collect positve interger from user inputs:
    // println!("Enter a value of n: ");
    // let mut num = String::new();
    // io::stdin().read_line(&mut num);
    // let num:i32 = num.trim().parse().unwrap();

    // // print and call to summing function
    // let result = sum(num);
    // println!("sum of n positive integer till {} is: {}",num,result);

//**************************************//
    // //collect integer value from user inputs:
    // println!("Enter a number: ");
    // let mut num = String::new();
    // io::stdin().read_line(&mut num);
    // let num:i32 = num.trim().parse().unwrap();

    // // print and call to bin_converter function
    // println!("binary representation of {} is: {}",num,bin_converter(num)); 

//**************************************//
    // //collect binary value from user inputs:
    // println!("Enter a number: ");
    // let mut num = String::new();
    // io::stdin().read_line(&mut num);
    // let num:i32 = num.trim().parse().unwrap();

    // // print and call to Deci_converter function
    // println!("binary representation of {} is: {}",num,deci_converter(num)); 

//**************************************//
 
}   
