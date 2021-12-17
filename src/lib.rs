use std::ops::{Deref, DerefMut};
use std::mem::MaybeUninit;

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
        return Self::new(v);
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
