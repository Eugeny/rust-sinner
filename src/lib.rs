use std::ops::{Deref, DerefMut};
use std::mem::MaybeUninit;

#[cfg(not(debug_assertions))]
compile_error!("D̴o̴n̸'̵t̷ ̷d̶e̵p̸l̶o̸y̵ ̵t̸h̷i̸s̸ ̵s̶h̴i̷t̷ ̵t̶o̴ ̷p̴r̸o̴d̶u̴c̷t̵i̵o̷n̴ ̷y̴o̷u̷ ̷m̶a̴d̸m̶a̸n̶");

mod list_of_the_damned;

pub use list_of_the_damned::ListOfTheDamned;

#[derive(Clone)]
pub struct Sin<T> {
    ptr: *mut Sinner<T>,
}

pub struct Sinner<T> {
    data: T,
    ref_count: usize,
}

unsafe impl<T> Send for Sin<T> {}
impl<T: Clone> Copy for Sin<T>  {}

impl<T> Sin<T> {
    pub fn new(data: T) -> Self {
        let ptr = Box::into_raw(Box::new(Sinner {
            data,
            ref_count: 1,
        }));
        Sin { ptr: ptr }
    }

    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).ref_count += 1;
        }
        Sin { ptr: self.ptr }
    }
}

impl<T> From<T> for Sin<T> {
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T> Deref for Sin<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.ptr).data
        }
    }
}

impl<T> DerefMut for Sin<T> where T: Clone {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut (*self.ptr).data
        }
    }
}

pub trait UniversalSummoningCircle {
    fn summon() -> Self;
}

impl<T> UniversalSummoningCircle for T {
    fn summon() -> Self {
        unsafe {
            MaybeUninit::uninit().assume_init()
        }
    }
}

#[test]
fn summon_test() {
    let x = i32::summon();
    let y = i32::summon();
    let mut z = Vec::<String>::summon();
    z.push(x.to_string());
    z.push(y.to_string());
}

// impl<T> Drop for Sin<T> {
//     fn drop(&mut self) {
//         unsafe {
//             (*self.ptr).ref_count -= 1;
//             if (*self.ptr).ref_count == 0 {
//                 drop(Box::from_raw(self.ptr));
//             }
//         }
//     }
// }


// --

// struct Item<T> {
//     pub prev: Option<Self>,
//     pub next: Option<Self>,
//     pub value: T
// }

// struct List<T> {
//     pub head: Option<Item<T>>,
//     pub tail: Option<Item<T>>,
// }

// impl Default for List<u32> {
//     fn default() -> Self {
//         List {
//             head: None,
//             tail: None,
//         }
//     }
// }

// impl <T> List<T> where T: Clone {
//     pub fn append(&mut self, other: T) {
//         let mut item = Item {
//             prev: None,
//             next: None,
//             value: other,
//         };
//         if let Some(ref mut tail) = self.tail {
//             tail.next = Some(item);
//             item.prev = Some(tail.clone());
//             self.tail = Some(item);
//         } else {
//             self.head = Some(item);
//             self.tail = Some(item);
//         }
//     }
// }

// fn main () {
//     let mut list = List::default();
//     list.append(1);
//     list.append(2);
//     list.append(3);

//     let mut ptr = list.head;
//     while let Some(item) = ptr {
//         println!("{}", item.value);
//         ptr = item.next;
//     }
// }

// use std::thread::{spawn, sleep};

// #[derive(Debug, Clone)]
// struct Stuff {
//     pub content: String,
// }

// impl Stuff {
//     fn bang(&self) {
//         println!("bang! {}", self.content);
//     }
// }

// fn main() {
//     let mut x = Stuff { content: "old".to_string() };
//     let t = spawn({
//         move || {
//             sleep(std::time::Duration::from_secs(1));
//             x.content = "new".to_string();
//             x.bang();
//         }
//     });
//     x.bang();
//     t.join().unwrap();
//     x.bang();
// }
