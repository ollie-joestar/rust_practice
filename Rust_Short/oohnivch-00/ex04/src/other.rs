// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   other.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: oohnivch <oohnivch@student.42vienna.com>   +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2024/10/07 13:40:44 by oohnivch          #+#    #+#             //
//   Updated: 2024/10/07 14:50:46 by oohnivch         ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

fn  main() {
    if cfg!(debug_assertions) {
        println!("Hey! I'm the other bin target!");
    } else {
        println!("Hey! I'm the other bin target!");
        println!("I'm in release mode!");
    }
}
