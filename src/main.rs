
extern crate rand_core;

use mcore::bn254::big;
use mcore::bn254::ecp;
use mcore::bn254::ecp2;
use mcore::bn254::fp2;
use mcore::bn254::pair;
use mcore::bn254::rom;
use mcore::rand::{RAND,RAND_impl};



use std::env;


fn main() {
    



    println!("Zero knowledge proof using bn254 Pairings");


    let G1 = ecp::ECP::generator();
    let G2 = ecp2::ECP2::generator();


    let args: Vec<String> = env::args().collect();
  

      let mut x:isize=7;
      let mut a:isize=-1;
      let mut b:isize=-42;

      if args.len() >1 { x = args[1].parse().unwrap();}
      if args.len() >2 { a = args[2].parse().unwrap();}
      if args.len() >3 { b = args[3].parse().unwrap();}

      println!("Solution proposed: {}",x);
    
      // Equ: x^2 + ax + b = 0
      let mut xbig=mcore::bn254::big::BIG::new_int(isize::abs(x));
      let mut xG1 = pair::g1mul(&G1,&xbig);
      let mut xG2 = pair::g2mul(&G2,&xbig);
      if (x < 0) { 

        xG1.neg();
       
        xG2.neg()
      } 
      let mut abig=mcore::bn254::big::BIG::new_int(isize::abs(a));
      let mut xGa = pair::g2mul(&G2,&abig);

      if (a < 0) { xGa.neg();}

      let mut bbig=mcore::bn254::big::BIG::new_int(isize::abs(b));
      let mut xGb = pair::g2mul(&G2,&bbig);
      // let xGa = BN254.G2mul(G2,BN254.NewBIGint(Abs(a))); 
      if (b < 0) { xGb.neg();}

    
      if (b < 0 && a < 0) { 
        println!("\nEqn: x^2 - {} x - {}\n",isize::abs(a),isize::abs(b)) ;
      } else if (b < 0) { 
        println!("\nEqn: x^2 + {} x + {}\n",a,isize::abs(b)); 
      } else if (a < 0) { 
        println!("\nEqn: x^2 + {} x + {}\n",isize::abs(a),b) ;
      } else {
        println!("\nEqn: x^2 + {} x + {}\n",a,b) ;
      }
 
      println!("\nxG1: {}",xG1.to_string()) ;
      println!("\nxG2: {}",xG2.to_string()) ;
      println!("\nxGa: {}",xGa.to_string()) ;
      println!("\nxGb: {}",xGb.to_string()) ;
    
      let mut p1 = pair::ate(&xG2, &xG1); p1 = pair::fexp(&p1);
      let mut p2 = pair::ate(&xGa, &xG1); p2 = pair::fexp(&p2);
      let mut p3 = pair::ate(&xGb, &G1); p3 = pair::fexp(&p3);

      p1.mul(&p2);
      p1.mul(&p3);

      if (p1.isunity()) {println!("\nYou have proven it"); }
      else { println!("\nYou have not proven it" );}


}
