// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   overflow.rs                                        :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: oohnivch <oohnivch@student.42vienna.com>   +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/10/07 13:40:31 by oohnivch          #+#    #+#             //
//   Updated: 2024/10/07 14:53:40 by oohnivch         ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

fn  main () {
    let first:u8 = 255;
    let second:u8 = 1;
    let _result:u8 = first + second;
    println!("{}u8 + {}u8 == {}", first, second, _result);
}
