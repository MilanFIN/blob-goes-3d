// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::fixnum::Num;
use agb::fixnum::num;
//use agb::fixnum;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {


    let mut bitmap4 = gba.display.video.bitmap4();

    // Set a palette entry 1
    bitmap4.set_palette_entry(1, 0x001F);

    /*
    //Num::new(5);
    let a: Num<i32, 8> = 10.into();
    let b: Num<i32, 8> = 5.into();

    let c: Num<i32, 8> = a / b * 100;
    bitmap4.draw_wide_point(c.trunc(), 0, 1);


    bitmap4.flip_page();*/


    //constants
    let width = 240;
    let height = 160;
    let mut scale:Num<i32, 8>  = Num::new(100);//100;
    let middle: [Num<i32, 8>; 2] = [Num::new(width/2), Num::new(height/2)];  // x, y


    let translated_points: [[Num<i32, 8>; 3]; 8] = [
        [Num::new(-1), Num::new(-1), Num::new(4)],
        [Num::new(1), Num::new(-1), Num::new(4)],
        [Num::new(1), Num::new(1), Num::new(4)],
        [Num::new(-1), Num::new(1), Num::new(4)],
        [Num::new(-1), Num::new(-1), Num::new(2)],
        [Num::new(1), Num::new(-1), Num::new(2)],
        [Num::new(1), Num::new(1), Num::new(2)],
        [Num::new(-1), Num::new(1), Num::new(2)],
    ];


  
    loop {
        
        bitmap4.clear(0);

        // loop here to not exit
        for translated_point in & translated_points {
            //perspective
            let z:Num<i32, 8> = translated_point[2];
            let zero:Num<i32, 8> = Num::new(0);
            let x: Num<i32, 8>;
            let y: Num<i32, 8>;
    
            if (z != zero) {
                let perspective_scale: Num<i32, 8> = scale / z;//scale / z;
                x = (translated_point[0] * perspective_scale) + middle[0];
                y = (translated_point[1] * perspective_scale) + middle[1];
            }
            else{
                x = middle[0];
                y = middle[1];
            }
    
            bitmap4.draw_point(x.trunc(), y.trunc(), 1);
    
        }
        bitmap4.flip_page();
        scale -= Num::new(10);
        if (scale < Num::new(10)) {
            scale = Num::new(10);
        }
        

    
    }
    
}

//[[417.0, 268.0], [435.0, 306.0], [410.0, 306.0], [397.0, 279.0], [380.0, 288.0], [405.0, 333.0], [387.0, 325.0], [368.0, 294.0]]
