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


### Topics about thread safe

https://stackoverflow.com/questions/59428096/understanding-the-send-trait

#### `Send` trait and `Sync` trait

- `Send` : transfer ownership between threads
- `Sync` : multiple threads can access the same variable at the same time

Most of rust's primitive data types are `Send`, except for `Rc` and `RefCell`.
- `Rc` should manage all the *reference count* stuff which refers to the same data.
- `RefCell` should manage all the *borrow* and *borrow_mut* stuff which refers to the same data.

When `&T` is `Send`, `T` is `Sync`.

#### `Mutex<T> where T: Send`

- When `T` is `Send`, `Mutex<T>` is `Send` and `Sync`, which makes sense.
- So `Mutex<Rc>` is not `Sync`.

#### Rc
- Reference counting, but not with atomic operation

#### Arc
- With atomic operation, which guarantees no data race.

#### RefCell is not thread safe

https://users.rust-lang.org/t/why-refcell-can-not-be-send-between-threads-safely/90196/6


RefCell is always used with `Mutex<RefCell<T>>`

#### Arc and Mutex : from a container view

`Arc<Mutex>` : `Arc` can be viewed as a container with reference counting. `Mutex` provides `Sync` trait.

https://stackoverflow.com/questions/56574632/why-mutex-was-designed-to-need-an-arc-in-rust

#### Cycle Reference in Rust

- Cycle reference in `Arc` : memory leak
- `Weak` : weak reference, no reference counting, no ownership.