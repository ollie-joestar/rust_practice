// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   fizzbuzz.rs                                        :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: oohnivch <oohnivch@student.42vienna.com>   +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/10/07 11:34:07 by oohnivch          #+#    #+#             //
//   Updated: 2024/10/07 16:39:30 by oohnivch         ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

fn  main() {
    for i in 1..101 {
        match (i % 3, i % 5, i % 11) {
            (0, 0 , _) => println!("fizzbuzz"),
            (0, _ , _) => println!("fizz"),
            (_, 0 , _) => println!("buzz"),
            (_, _ , 3) => println!("FIZZ"),
            (_, _ , 5) => println!("BUZZ"),
            (_, _, _) => println!("{}", i),
        }
    }
}

//fn  main() {
//    let mut n:i32 = 1;
//    while n < 101 {
//        if n%3 == 0 && n%5 == 0 {
//            println!("fizzbuzz");
//        } else if n%3 == 0 {
//            println!("fizz");
//        } else if n%5 == 0 {
//            println!("buzz");
//        } else if n%11 == 3 {
//            println!("FIZZ");
//        } else if n%11 == 5 {
//            println!("BUZZ");
//        } else {
//            println!("{}", n);
//        }
//        n += 1;
//    }
//}
