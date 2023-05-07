![rust-containers](./assets/rust-containers.png)

#### borrow rules
- local variable : life time known at compile time
- local variable borrow rules : based on life time (scope)
- no multiple mutable references at the same time

#### static mut
- static mut type is unsafe : lifetime is static
- ==> no static mut in this project

#### Box
- exclusive ownership

#### Rc
- multiple ownership, but immutable

#### Arc
- multiple ownership, immutable, thread safe

#### RefCell
- interior mutability : actually mutable, but defined as immutable
- use it through `borrow() -> Ref<T>` or `borrow_mut() -> RefMut<T>`
- check borrow rules at runtime

#### `borrw_mut()` and `drop()` (manually)
- The `drop()` function will be automatically called when the variable goes out of scope. (That is, even the value will not be used anymore, the compiler wouldn't call `drop()` until the current scope ends.)
- We can manually call `drop(RefMut)` to release the borrow lock.

#### Cell
- interior mutability
- exclusive access by copy

#### Mutex
- lock and unlock

#### RefCell is not thread safe

https://users.rust-lang.org/t/why-refcell-can-not-be-send-between-threads-safely/90196/6


RefCell is always used with `Mutex<RefCell<T>>`

#### Arc and Mutex

why Mutex needs an Arc : just a container to hold the mutex, to avoid lifetime issue

https://stackoverflow.com/questions/56574632/why-mutex-was-designed-to-need-an-arc-in-rust

#### How to use RecCell globally 
- I don't know how to implement a `Mutex`.
- Our OS does not support multi-threading.
- just create a new struct `UPSafeCell` and we assert that it is thread safe. (we only have one thread...) 
- ~~tuo ku zi fang pi~~
