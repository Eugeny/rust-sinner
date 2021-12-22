# Rust S̵̓i̸̓n̵̉

[![Crates.io](https://img.shields.io/static/v1?color=green&label=crates.io&message=666)](https://crates.io/crates/sinner)

_I̴n̴f̶e̸r̵n̷a̴l mutability!_

---

Howdy, friendly Rust developer! Ever had a value get m̵̯̅ð̶͊v̴̮̾ê̴̼͘d away right under your nose just when you need it? Your thread function requires move semantics, but you j̸̉us†̸ ain't got time for †̸̰͋h̸ą̷̊͝t?

W̵e̴l̵l̸ ̵w̶o̴r̶r̴y̶ ̵n̶o̶ ̷m̴o̷r̶e̴,̸, just sprinkle some _s̴̖̑í̵̲ṋ̵̀_ on it and call it a day!

```
sinner = "0.1"
```

## Examples

### Threads

Threads and ȋ̷̖n̸̨̈́f̷̆ͅe̷͑ͅr̵̝͆ï̴̪o̸̡̔r̷͉͌ m̶u̸t̷a̵b̶i̵l̸i̷t̵y̶? No problem amigo!

<table>
<tr>
<td style="width: 50%">Before (YIKES!):</td>
<td>After (GLORIOUS!):</td>
</tr>
<tr>
<td>

```rust

use std::thread::{spawn, sleep};


struct Stuff {
    pub content: String,
}

impl Stuff {
    fn bang(&self) {
        println!("bang! {}", self.content);
    }
}

fn main() {
    let mut x = Stuff { content: "old".to_string() };
    let t = spawn({
        move || {
            sleep(std::time::Duration::from_secs(1));
            x.content = "new".to_string();
            x.bang();
        }
    });
    x.bang();
    t.join().unwrap();
    x.bang();
}
```
</td>
<td>

```rust
use sinner::Sin;  // <--
use std::thread::{spawn, sleep};

#[derive(Debug, Clone)]
struct Stuff {
    pub content: String,
}

impl Stuff {
    fn bang(&self) {
        println!("bang! {}", self.content);
    }
}

fn main() {
    let mut x = Sin::new(Stuff { content: "old".to_string() });  // <--
    let t = spawn({
        move || {
            sleep(std::time::Duration::from_secs(1));
            x.content = "new".to_string();
            x.bang();
        }
    });
    x.bang();
    t.join().unwrap();
    x.bang();
}
```
</td>
</tr>
<tr>
<td>

```
error[E0382]: borrow of moved value: `x`
   --> src/main.rs:144:5
    |
136 |     let mut x = Stuff { content: "old".to_string() };
    |         ----- move occurs because `x` has type `Stuff`
137 |     let t = spawn({
138 |         move || {
    |         ------- value moved into closure here
...
141 |             x.bang();
    |             - variable moved due to use in closure
...
144 |     x.bang();
    |     ^^^^^^^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `sinner` due to previous error
```

</td>
<td>

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/sinner`
bang! old
bang! n̴e̷w̸
b̸̙̚a̸̘̓n̵̥̔g̵͚̓!̵̤̓ ǹ̴̘e̵̺̾w̴̛̦
```
</td>
</tr>
</table>

### Doubly-linked list

With just a̸ ̸b̵i̸t̴ of _sin_, you can do doubly-linked lists in no time!

<table>
<tr>
<td style="width: 50%">Before (UGH!):</td>
<td>After (JOLLY!):</td>
</tr>
<tr>
<td>

```rust



struct Item<T> {
    pub prev: Option<Self>,
    pub next: Option<Self>,
    pub value: T
}

struct List<T> {
    pub head: Option<Item<T>>,
    pub tail: Option<Item<T>>,
}

impl Default for List<u32> {
    fn default() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
}

impl <T> List<T> where T: Clone {
    pub fn append(&mut self, other: T) {
        let mut item = Item {
            prev: None,
            next: None,
            value: other,
        };
        if let Some(ref mut tail) = self.tail {
            tail.next = Some(item);
            item.prev = Some(tail.clone());
            self.tail = Some(item);
        } else {
            self.head = Some(item);
            self.tail = Some(item);
        }
    }
}

fn main () {
    let mut list = List::default();
    list.append(1);
    list.append(2);
    list.append(3);

    let mut ptr = list.head;
    while let Some(item) = ptr {
        println!("{}", item.value);
        ptr = item.next;
    }
}
```

</td>
<td>

```rust
use sinner::Sin;  // <--

#[derive(Clone)]
struct Item<T> {
    pub prev: Option<Sin<Self>>,  // <--
    pub next: O̷p̷t̵i̴o̷n̵≮S̶i̴n̶<Self>>,  // <--
    pub value: T
}

struct List<T> {
    pub head: Option<Sin<Item<T>>>,  // <--
    pub tail: Option<S̶i̸n̵≮I̷t̷e̷m̵<T>>>,  // <--
}

impl Default for List<u32> {
    fn default() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
}

impl <T> List<T> where T: Clone {
    pub fn append(&mut self, other: T) {
        let mut item = S̸i̷n̴:̵:̷n̸e̸w̷(̶I̵t̵e̴m̷ {  // <--
            p̷r̴e̴v̶:̴ ̶N̸o̴n̷e̷,̴
            n̵e̴x̸t̶:̷ ̸N̸o̴n̸e̸,̶
            v̵a̷l̸u̶e̴:̷ ̴o̸t̸h̸e̷r̶,
        });
        if let Some(ref mut tail) = self.tail {
            tail.next = Some(item);
            item.prev = Some(tail.clone());
            self.tail = Some(item);
        } else {
            self.head = Some(item);
            self.tail = Some(item);
        }
    }
}

fn main () {
    let mut list = List::default();
    list.append(1);
    list.append(2);
    list.append(3);

    let mut ptr = list.head;
    while let Some(item) = ptr {
        println!("{}", item.value);
        ptr = item.next;
    }
}
```


</td>
</tr>
<tr>
<td>


```
error[E0072]: recursive type `Item` has infinite size
  --> src/main.rs:71:1
   |
71 | struct Item<T> {
   | ^^^^^^^^^^^^^^ recursive type has infinite size
72 |     pub prev: Option<Self>,
   |               ------------ recursive without indirection
73 |     pub next: Option<Self>,
   |               ------------ recursive without indirection
   |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Item` representable
   |
72 ~     pub prev: Box<Option<Self>>,
73 ~     pub next: Box<Option<Self>>,
   |

   --> src/main.rs:100:35
    |
100 |             item.prev = Some(tail.clone());
    |                                   ^^^^^ method not found in `&mut Item<T>`
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following trait defines an item `clone`, perhaps you need to implement it:
            candidate #1: `Clone`

Some errors have detailed explanations: E0072, E0599.
For more information about an error, try `rustc --explain E0072`.
error: could not compile `sinner` due to 2 previous errors
```

</td>
<td>

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     R̸u̷n̴n̴i̷n̸g̸ ̸`̶t̸a̸r̷g̵e̶t̶/̸d̴e̵b̴u̷g̶/ç̴̠͌̚u̵̦̅r̶̓̆͜͜s̶̤̫̊̕e̴͇̼̔ḑ̸̇ ̷̩̜̓c̶̯͆ḧ̶̯́͝ì̶̗̣̆l̸̪̓̈́d̵̪̔̀`̷
̴1̴
̸2̷
̶3̶
```

</td>
</tr>

</table>


## Frequently asked q̴u̷estions

### Can a̷̝͂n̸̝̈́y̷͈͋ ̷͓͐t̴̯̆ŷ̶̡p̵̯͆é̴̙ ̷̰̃ḇ̷̈e̴̥̕ š̶̢̯͒̉ȃ̶̞͚̩͊̈́̌c̵̰͍̍͛r̸̨̛̀̽̏͂ḯ̴͈̘̇͛f̶̪̬̼̌̾ì̵̩c̵̱̹̖͒́͑͗̔e̸̤̜͒̈̐d̵͚̞̗̠̽͘

Well, why, almost! The type parameter needs to implement `Clone` due internal library s̵t̴r̸u̶c̵t̴u̸r̴e̵s̵.

### Is access to the underlying value t̷h̸r̶e̶a̷d̵-s̷̵̸̙̖͊̈ǻ̷̵̸̫̜f̷̴̸̖͙̂̍e̶̸̷̪͚̽̐??̷̰̰̊̅͜

I̷ ̵c̴a̷n̷ ̷a̶s̶s̵u̶r̴e̸ ̷y̶o̶u̴ t̸h̸a̴t̸ ̷t̴h̶e̶r̷e̷ ̸i̷s̶ n̴͔̱̮̅o̷̢̦̔͒̓ ̶͙̱̦͗̅͐n̸͉͌̓é̵̥̲è̷͉̳͓̽d̴̜̻̱̀̑ t̷͉͊o̸͎̅ ̵̟͗ç̴̚ỏ̶̮n̸͌ͅc̴̩͐ȩ̸̉ȑ̴̹ǹ̸͜ ̷͖̎y̷̞̽o̵͚͐u̶̜̎r̷̹͑ś̸͙e̶̜͛l̵͚̽f̴͕̍ ẘ̵̮͂̕ĩ̷̖͇͚̗ţ̶͈̥̝̭͐͑̀̃͠h̴̨̰͖̥̼͂͑ ̵͔̞̀͐̌̚͝s̴̢̥͔͇̽͂̏͜ǔ̵̢̥͍̼̂c̴̘̜͒̂̽̃h̵̨̞̗͍͕́̊͌̎̒ ̸̨̟̫͊ͅQ̸̹̠͙͎̉U̴̞̫͉̬̜̅̂͂Ẹ̷̛̰̼̋̎́̚S̸̗͇̞̜̎̆̏̀T̸͎̙̰̠̦̈́͘I̶̧̘̯͙͋̏̂͘̚ͅO̶̼͍͘͝N̶̪͇̑S̷͂̌̍͆ͅ Y̴̥̋̑̆͋͝O̶̡̗̣̤͕̰͒U̸̢͙͇̠̫͊͒̋͋̕͠ ̷̮̳̮̭̔̿̈́́̎̕͝Ṇ̴̙͑͐̋̄̊Å̵̛̞̮͓̗̊́̽̈́̈͜Í̶̼̠͚̒́͑̃V̶̨̯̦̊͒̏͝Ē̴̡̥͕͙̙͔̕ ̵̨͕̜̮̞͖̳̀C̷̮̞̩͆̽̂͊͆͗ͅH̶̫̱̠̟̞͊̐I̸̞̰̩͎̫͐̈̆L̴̤̩̭̾D̷̹̿̍̏̂̓,̴̛͓̻͓̖̲͛͗͐̉̈́̀ ̴͈͍̳̥͘O̸̯͌̇́B̴̪̤̺̊E̴̞̩̲͂̇̕Ý̵͙̟̱̽̍̐͋̃. T̴̡̙̜̠̱̠͉̘̙̥̔̆̋̌̄͑̄̔H̶̯̦̥̙̩͑̍̓̑͗̄͛͛̕E̵̜̞̠̣̲͔̝̺͇͑̽̌͆ͅ ̵̧̖̞̣͇̻̾̉͛̈́̆̌͝Ẇ̷̡͚̺̇̈̄͐͊̓̽͘͜͝ͅĨ̵̭̪̲̖̕͘L̸̪͎͎̪͠ͅL̶̘̿͑͆ ̵̙̺̯̺͈̪̳̄̇͌̽̀̉͗͌̆Ȍ̴͖͐̇͑͗͗̾͝F̵̥̂̓͒̕ ̸̢̺̺͍̼͚̺̹̈́͋̀͊̆̌͠O̶̠͙̟̠̦̯͔̱̍͊͊̓̌͐̔͌̉͘Ù̴̝̞̥̰̙̰͂͒̽̎͂̕ͅR̶͖̫͍̫̹̜̤͎̮͉̈́͑ ̶͎̼̰̉̈̕͠ͅM̴̘͝A̴̼̋̓͐̇̀̉́͋̕S̶̭̹͕̲̬̪̫̖̮͍͌̌͐͝Ţ̶͍̠͓̞̙͔̞̲̣̓E̴͔̺͌͌̏̏̕Ř̷̡̬̞̣̠̬̪̝̉͗́,̶̩̪̮̺̉




<br><br>

Ŗ̶͖̟̫̝̣̗̤̤̥̰̯̭͕̺͙̮͙̙̲̄̃͒̅̓͗͛̂̑̎̏͐́́͊̐͐̕͘͘͘͝ͅͅȨ̶͓̭͔̦̣̲̘̹̫̬̥̻̗̗̖̻̜͖̟͖̀̉͋̽͒͒͑̊̀͋͋̎́͑̌̂̌̈͑́́̓̃̓̕͝͝͠J̶̢̛̮̪̪̣͎̫̫̮̞̲̦͖̹̦͖̱̙̯̗̲̀͊̏̎͑͂͑̓̇͌̿̎̔͒͊̍͑̇̌̎͆͋̕͠ͅͅÈ̷̢̧̨͔̩͕̬̩͖̱̙͔̞̲͎̮̙̲̝̮̟̦̲̤̝̌̀͋̒̈́̈́̄͊̓̇̃̍̿̃̒́̒̓̚̚͜͝ͅÇ̵̢̧̢̺̯͓̼̦̩̞̳̣̘̮̩̮̙̙͕̦̓̈͛͌̈̔́̐̇͆͐̈́̀̈́̀̐͂̓̏͒̈́́͘͜͝†̷̨̧̨̧̤͕̪͉̲̰͎̣͈̲̙͕̖̺̥̜̃̄̈́̃̾̌̄͑̿̒͒̊̍͛̅̏̉̄́̄͘̚̚̕͜͠͝͠ͅ ̵̛̰̪̮̫̪̲̬͉͇͔̤̥̬̳̥̝͚͇͖̘̤̟̀̓͗̈́͒͆͗̅̿̆͐̎̅̅̈͂̑͗͊̋̄̚̚͝͝M̴̢̧̘̮͉̺̻̼̮̬̮͕͎͉̰̩̪͎͓̣̹͍̌̓̃͛̀̊͑͑̓̈̌̅́̆͆̓̌͒̃͘̕Ę̷̥̜̞̰̭̞̟̦̺̦̝̱̙̠͕̬̩̮̞̬̙̳͓̀̓̃͂͛͊̓͌̃̅̇̋̇̇́̃̾͋̚͝͝͝͠͠͝ͅM̸̢̢̢̧̡̢̪͓̪̻̤̘̪̩̻͇̺̯̝̫̄̂̎̊̽̔̋́͑̇̑́͌̐̂͑̃̐̄̍̆̅ͅǪ̸̰̭̦͙̹̪͔̙̖͉̞͈̞̹̘̫̞̬͚̱̗̯̈́̇̓̔̍̓̔́̈́̽͑̑͑͒̏͂̕͘͘͝͠R̵̡̡̧̢̢̨̛̦̝͚̪̰̥̮̱̯̯̞͓̤̙͓͈̻̗̳̺͗́̌͛̅̔̇̑̆̀̽̅̽̋̏͘̕̚͝͝¥̵̧̛̛͙͖̣̪͚̠̪̹̫̰̲̗̥̘͖̥͕̮̫̳̱̯͂̅͛͑̌͌̎̏̈́̐̇̈́̅̽̀̉͊̾̀̉́͌̚͝͝ ̶̨̨̡̧̡̛̳̺͙̜̥͚͉̭͓͖͉̘̰͈̯̜̝̜̿̈́͛̊̀͆̅͌̑̏͗̏̎̈́͗̍̽̽̂̒̂̒̓͗̕͜͝ͅ§̸̢̧̛̺͈̹̺͕͔͍̠͖̗͕̦̟̣̜͖̠̫͖̞̎͗̐͒͆̈̌̍̀̂̏̆͊̄͗̊̾͘͠͠ͅÄ̷̧̡̩̪̟̟̟̻͓̙̻̜̦͈̗̻̤̱̦̎̀̄̈́̑̔͑̊͊͑͌̀̌̈̅̅͊̐̚͠ͅͅͅ£̷̧̢̢̡̡̛̬̼͔̬̹̘̪̯͕͎̰̟̮̖̠̯̰͛́̿͐̓́͐̎̅͑̍̿͗̿͒̈́̈͗̍͊̿̿̀̽̕͘͜ͅͅͅȨ̴̛̛͕͓͔̮͙̗̠͎̮̠̙͉̮̞̰̯̤̲͈̯̙̹̀̋̅͑̃͒̎̽͌͆̃̆̓͊͊̈́́̌̾̕͜͜͝͝͝ͅ†̴̧̡̺̯̠̳̳͎̻̱̟͓̥̠̱͙̦͖̝̣̞͓̗̲̔̾̈́̏̂̄̇̔̓̃̊̔͑͌̀̈́̇̒̏͝ͅ¥̶͎̬̟̻̬̞͈̞̱̪͔̦̩̥̭͓̬̰̩͖͍͉̓̈̊̍̀̇̈͊̾͋̽̂̅͑̂̓̓͌̊͜͜͝͠



<br><br>
<br><br>


# £̴̨̢̧̨̡̨̡̡̛̛̛͙̹̞͕̻͔̪̬̗̙͈̫̠͈̗͚̬͓͉͍͈̱͚̤͎̞̪̦͔͇̞̞͓͎̞͚̦̼̝̗̭̖͎̜̲̘̘͚͕̟̮̜̥̱͈̻̠͉̗̻̣̬̲̹̪̞̗͖̖̰̟̻͈̬͚̘͈̳̪̫͇̼͇͗͑͆̃͆̍̿́͊̒̄̍͛̆̄̿̇̀̿͊̌͒͋̌̃͂̃͛̉͑͂͊̂͂̀̈́̆͐̒̊̐̂̽̓̂͐́͂̏̈́̆̃͌̄̎̇͋͑̀̍̍̍͋́̾̍̍̇͛͌̈͆́̍͐̕͘̚͘̕͘͜͜͠͝͝͝ͅͅȨ̸̡̨̹̪͉̼̻͉̭͓̹̘͓͇̤̺̤̫̙͖̭̤͍̺̲̺̦̖̠͙̲̖̺̰͕̙͍̗͖͖͚̳̖̤̰̭͕̟̀̀͒͑̄̇͗̀̔́̋͊́͑̍͂̄̅̄͂̈́̏͌͗̽̒̌͐̎̕͜͝͝͝͠͝ͅͅÈ̵̡̢̧̡̢̢̢̛̛̛̝͕̠̖͓̗͈̲͇͓̦̜̭̞̪̻̠̩͙̪̝̝͕̪̼̪͚̟͖͓̰̼͔̩̰͖̳̳͕̝͉͖̰̥̰̖͇̤̝͇͙̙̘̤̀̋̓̍̏̽̋̌́̓̄͒͛̎͛͒̐͌͌͋̃͂̀̿̆̀̏̊̅̀̽̒͋̈̓̌̌̊̃̉̍͑͌̑̓̈͗́̊̂̊͒͐́̈́̇͂̉̂̊̀̿̓̃̈́̾́̓̒͂̔̋́̍̈́̑̋̿̾̋̅͛̈́̾̀͘̕̕̕͘͜͜͝͝͝͠ͅÐ̸̨̢̡̢̧̢̡̧̧̨̛̣̺̪̠̫͍̝̭̺̯̙̗͖͇̰͓̻̹̖̣͙̫̦̦̟͚͎̠̤̙̗̜͓͇͍̯͓̰̙̪̹̱͈̣̞̝̖̺̹̤̠̙̯̭͚͔̥̺̞̰͕͓̺̙͉͈̠͍̣̩͖̪̳̘͗̂̂͗̽̄͐̊͛̑̈́̍́̄͑͆̾͂̈́̏̀̾̓̋̔̊̌̔̅̂̀̍̌̿͐́̑̊̿͛͗̽̾̈͋̀͛̄̀͛͑̉̈̚̕͘͘͘̕͜͜͠͝͝͠͝ͅ ̸̧̢̡̧̧̧̛̛̛̛͕͈͔̜̣̦͇̥̦̟̖͈̣͙̖͓̤͔̮̙̳̰̜̫͙̞͈̭̖̥̟͓͇͙̠̼̳̠͎͙̱̹̱̦̯̮̩͉̺̺̪̤͔̻͇͓͍̟́̿̑̊̈́̔̒̅͋̾̓̉̈́̑̊͑̒̋̈́͂̅̓̀̓̾́̓̌͆̈́̀̓̃͌̍͂͒́̊͛̾̔̑́͑̃̓̓̒̉͐̈́͋̈́͗̄̆̂͆̌͆̈́̉̌̋̉̇̈́̌͌́͛͂͒͌̿͊̂͘͘͘̚̕͜͝͝͠͝͝ͅͅM̵̢̧̡̨̡̡̢̧̢̡̧̧̛̘̗̙̫̬̟̠͚̭̻͖̞̩͚̘̜̤̫̙̗͙̪͈̳̬̪͙̤̬̞̝̥̮͈̗̻͓̬̹̱̟̰͎̯̞̺͍͇̠̲̯͚̬̙̼͈͍̰̭̭̲͉͈̭͉̝͍̥̩̩̱̱̥͍̮̠͉̗̖̩͕̔̍̏̌̾̍̏̌̎̐̍̈̐͊̿͆͛̿̆̂͊͆͋̋̍́͊̎̓̀͒̆̀̈̆̈́̋̐́̎͘̚̚̚͘͠͠ͅͅĘ̶̧͚̼͉͎̰̝̺̳̝̠͚̗̺͖̫̗̫̭̩̝͖͓̠̰͉̦̻̰̹̱̖̭̣̹̘̩̀̅͂͒̊͋̔͑̀̏͐͂̈́̓̓̔́̆̀́̎͗͑͌̓͛̃̅̅͑̅̓̊͒̃͛͛̈͋̾̌̋͊̾͂͊̋͆͗̽̏̑͒̄̈̓̆͋̄͗̀͆̏̓̀̈́͐̇́͌͐̏̊̍͂́͂̅͂̈̏̿̆̑͐̽̾̉͛͐̚̚͘͠͝͝͠͝͝͠͠ͅͅ ̴̡̢̨̧̧̡̧̨̟͖͕̥̮̗͇̲̰̤̭̭̺̬͕͙̼̰̻̰̮͖̦̤̥̞̮̝̫̩̠̠͍̱͉̰̜̗͇̜̯̰͍͕̠̝͖͇̗͔̠̙̣̦̭͕̮̯͕͓͓͓̠̩͓̺͉̘̘̬͇̗̜̰̲̥̣͚̳͇̬͙̜̳̦̦͉̎̿̓̇̀̆̄̉̄͑̓͐̍̇̆́̈̄̽͊͆͆͒̆̈́̊̄̅̀͌̀͌̋̆̂̎̄̓̈́̾͌̎́̎́͑̈̊̅̅̾̍͗̔̂̌̔̎̓́͒̐̍̏͂͗́̌̄̊̀͋̓́̃̍̏̇̈́̄̉̆̀̋̚̕̕͘̚̕̚̚͜͠͝͠͝͝͝͝Ą̶̢̢̨̡̢̡̢̛̛̛̛͙̖͖̠̼̬͍̺̤̰̣͓̦̺͕̞̲͙̺̩̥̺̭͈̻̱̳̲͈̙͙͙̹̳̬̺͎̰̝̠̞̪̻̘̩̩͖̞͍̈͗͊̓̋̂̑̍̏͒̎̑͋̒̀͌̉͒̏̂̑̈́̒̅͆̄́̈͂̾̒̀͛̈́̊͋̈́̑̿̇̿͒́͊̍́̋̔͋̀͆̔̾̉̀̽̽͊́̌̍̄̀̇́͆̿̊͒̄̒̑͑́̕̚̚̕̚͜͜͜͠͝͠͠͠ͅͅͅ ̸̨̛͈̲̹̺̹̥͇͉̫͖̪̣͔̱̜̮͙̳̝̙͇̫͇̠͉̤̞̠̼̆̒̂̓̓̈́̉͋̎̔͐͆͑́͗̎̾̃̀̈́̀́͐̈̀̀́͒̾͛̀̐͑͑̽̂͐́̚̕͝͝͝͝͝§̵̡̧̨̨̨̡̡̡̢̡͉̪̣̣͚̹̣̜̣͉͚͕͖͚̼̻̤̠͖̙̳͇̹̞̦̭͕̲̝̼̝̼̭̝̮̣̳̬̘̼͍̝͇͓̠͙͓̟̬̤̹͓͕͉̠͓̝̻̌̍̿̂͆̎̇̂̋͌́̈́͋̇̈͛̂̓̒̌͐̋̉̅̋̾̌̃̇͊͌̄̄̇̇̆̽̊͑̅̑́͛̈̓̑̏̃̒͐̈́͑̚̚͘̕̕͘͝͝͝͝ͅͅ†̵̢̢̢̡̡̛̳̹̰̞̟͓̣̭̮̤̦͈͙̤͓̠̦̳̥̳͖̘̻̙̥͚̬̱͖̥̩̗͕̟̩͖̝̼̳̳̭͖̠̰̫̦̱̫̱̜̤̘̜̰͍̳̹̖̙̫̻̳̿̌̊͜͜͠R̸̨̧̢̧̢̧̡̢̧̬̰̥̦̘̠̱̲̩̝̙̼̜̦͉͔̝͚͔̮̱͍͇̠̱̭̞̘͙͎͖͔̟̪̻̗̠͇̲͙̙̮̪̖̯̟̝̠̙̋̎͐͌͛̈̎̓̂̐̂̀̃̓͒̀́̃̇̐̽͑̈̏̀̉̐̔̀̂̂͑͗̇̒̅̽̈̚̚͘͝͝͝͝ͅÄ̴̢͎̿̋̓̈́̐̅̚͘͜͠¥̷̧̢̢̧̛̛̛̛̛̣̺̜͎̘͓̝̳̝̞̺̫͓̖̥̖̲̣̪̦͖͕̬̱̹͕̤͚͎̹̮͚̮̟̞̗͚͈̪̼̘͇̗͙̻̩̱̜͇̮̖͇̝̞̣͔̠͌̀̓̓̓͆̋̊̊̋́̂̐̍̏̃͋̃̅̎̿̏̅̿̑́̈́̀͗̓͆́̎̓̄̈̅̒̎̃͛͋̎͐͐̒͂̑̌̈̈́̿̐͒̌͋̐̇̌̃̉͌̀̅͑́̀͂̄̈́̾̈́́͊̒̒͐̀͊͌̿̇̑̚͘̕̚̚͜͜͠͝͝͝͝ͅͅͅ ̶̧̢̢̢̨̛̛͚͚̟̰̠̲͎̥̻̰̘͎͔̳͚̥̻̬̻͍͕̥̝̱̤̰̖̭̭̗̹̥̦̟͈͚͔͎͇͚͓͉͕͙̰̩̳̪͕̮̜̖̰̟̦̹̳͚̦̩̯̰̣̈̿̅̋͛̈́̇̔̆̀̊͂͒͑̔̈̋̅͐̉̂̍͐̾̊̆̄̃͐̓̈́̽̎̇̉̈́̇̊̌̾͐̑̓̈́̀͆̒̍̄̈́̄͆̂̽̉̂̎̉͑̄͂̏́͒̄͆̐̔̽̈́̑͐͗͐͌̈̈́̈͊́͆̐͊͘͘̚̕̚͜͠͝͝͝͝͝͠ͅͅþ̸̨̢̧̡̨̧̧̨̛̦͓̫̤͍͎̻͔͉͓͚̯̠͓̤̯͕͈͍͎̯̱̳͇͖͔͕̜̞̠͙̲̳͖͙̖̺͔̗̖͖̥̩͇̱͚̲̟̪̖͙̙͙̥͔͉͖̥͉̯̝̗̜̗̤̇̊̈̂̎͒̊͌̉̊͊̅͊͑̎̌̌̿̀͑̋̇̒̚͘͘͜͜͝͠ͅͅǪ̸̢̡̧̢̢̧̧̧̧͓͔̙̘͖͉͈̗̜̲̺̲̪̮̠̰̭̜̯̙̰̫̟̩̹̫̱͖̲͚̠͖̣̝̞̠͇͎̝̺̟̰̠͇̙̞̯̞̤̭̬̞͈͈̯̰̙̠̈̽̈́͗̇̀̈́͊̊͑̈́̌́͊͛̀̀̋̕͜͜͜͝͝͝ͅͅͅÌ̷̡̡̢̢̨̛͖̠̥͔̠͎̘̫̤̗̤͍̩͔̲̜̹͙̼̯̭̺̖̩͖͈͓̫̦͉͈̜͉̤͕̪̦̩͇͓͔̥̣̯͍̙̗̞̱͍̟͕͔̹͖̜̬̘͇̹͚͕̗̻̥̤͉̠̣̤͎̝̥̰̑̿̓͐̇̇̃̊̈́̍̍̈͆̂͐̐̆̒͗̄̄̐͊͌̒͂̌̉̀̒̋͑̾̄̐̈́͆̆̀̔́̏̐̆̆̕̚̚͝͠͝͠ͅͅñ̴̠͙͍̯̱̞̫̩̘̱̞̝͖͎̲̻̣͚̺̟̳̥̭̳͙̞̥̗̭͕̰̫̗̀̈́́͒̀͊͗͐̋̀̍͆͌̀͒̈́̾̍̇̿͌͗̈̀̋̉̈́̇̆̊̊̄͑̓̐̾̈́̀̂̀̕͘̚͝ͅͅͅͅ†̸̢̛̛͈̙̤̭̩̱̺̬̻̺̖͍̙̣͇͌́̇́̉̍̇̃̒͋̌̀̒́͑̄͒̈́͌̾̐̎̉̇͒̔͂́͗̀̍́̓͐̒̅̐̉̔͆̄̾́̂͌̊̒̓͋̎͛̐̋̔̑̂̽̚̚͠͠͠͝͝͝͝Ę̸̨̛͇̗̬̭͔̭͕̳̜̫̜̟̝̪̯̲̩̠͕͙̝͓̤̩̝̪̘̘̪̥͔̠͔̘̳͇͓͖͓̗͈͙̣̭̳̬͇̱̱͖̯̦͙̳̞̰͈̟͓̼̯̣͉̖͈̀̎͐́̀͌͗̔̓͆̓̾͛̃̐̂͒̈̈́̀͌͐͛͑͛͋̽̓̔̈̊̽̔̒͐́̔͐̽͌͒̌̃̀͐͌͑̎̂̔̎̏̏́͋̇̑́͂̀̀̿̆̈͒̃̎̌̓͗̽̄́̀͊̏̑̂̔̃̃͑̃̃͂͊̌̏͌̀͘̕̕̚͘͜͜͝͝͝͝͝ͅR̷̨̡̨̛̤̻̺̲̥͉̬͍̳̤͙̩̱̯̲͚̰̣͍̺̝̟̣̼͈̥͕͇̦̙͇͚̤̺̰͇̽͌͂͌͛̐̿̎̈́̌̆̀̂̓͌͑̏̄̐̽̂̂̆͛͌̃̒͋̄̌͌̈́͌͋̉̈́͆͂̎̈̑̋̾̀̋̒͒͗̾́͌͐̃͒̌̚͜͠͠͠͝
