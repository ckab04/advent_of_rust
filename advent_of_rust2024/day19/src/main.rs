// 1. We have 3 states:
// - Empty
// - Ready
// - Flying


// #[derive(PartialEq)]
// pub enum StateSleigh<State>{
//     Empty,
//     Ready,
//     Flying,
// }

// 2. Finish the Seligh struct definition
// pub struct Sleigh{
//     state: StateSleigh,
// }

use std::marker::PhantomData;


pub struct Empty;
pub struct Ready;
pub struct Flying;

struct Sleigh<State> {
    _state: PhantomData<State>,
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Sleigh{
            _state: PhantomData
        }
    }

    pub fn load(self)-> Sleigh<Ready>{
        Sleigh { _state: PhantomData}
    }
}


fn main(){}

// 3. Write the Sleigh Implementations for all states

// impl Sleigh{
//     fn new() -> Self{
//         Self{
//             state: StateSleigh::Empty
//             }
//     }

//     fn load(&mut self){
//         if self.state == StateSleigh::Empty{
//             self.state = StateSleigh::Ready;
//         }
//     }

//     fn take_off(&mut self){
//         if self.state == StateSleigh::Ready{
//             self.state = StateSleigh::Flying;
//         }
//     }

//     fn unload(&mut self) {
//         if self.state == StateSleigh::Ready{
//             self.state = StateSleigh::Empty;
//         }
//     }

//     fn land(&mut self){
//         if self.state == StateSleigh::Flying{
//             self.state = StateSleigh::Ready;
//         }
//     }
//}
