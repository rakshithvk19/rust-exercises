# Variants can hold data

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Our `Status` enum is what's usually called a **C-style enum**.\
Each variant is a simple label, a bit like a named constant. You can find this kind of enum in many programming
languages, like C, C++, Java, C#, Python, etc.

Rust enums can go further though. We can **attach data to each variant**.

## Variants

Let's say that we want to store the name of the person who's currently working on a ticket.\
We would only have this information if the ticket is in progress. It wouldn't be there for a to-do ticket or
a done ticket.
We can model this by attaching a `String` field to the `InProgress` variant:

```rust
enum Status {
    ToDo,
    InProgress {
        AssignedTo: String,
    },
    Done,
}
```

`InProgress` is now a **struct-like variant**.\
The syntax mirrors, in fact, the one we used to define a struct—it's just "inlined" inside the enum, as a variant.

## Accessing variant data

If we try to access `AssignedTo` on a `Status` instance,

```rust
let status: Status = /* */;

// This won't compile
println!("Assigned to: {}", status.AssignedTo);
```

the compiler will stop us:

```text
error[E0609]: no field `AssignedTo` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.AssignedTo);
  |                                        ^^^^^^^^^^^ unknown field
```

`AssignedTo` is **variant-specific**, it's not available on all `Status` instances.\
To access `AssignedTo`, we need to use **pattern matching**:

```rust
match status {
    Status::InProgress { AssignedTo } => {
        println!("Assigned to: {}", AssignedTo);
    },
    Status::ToDo | Status::Done => {
        println!("Done");
    }
}
```

## Bindings

In the match pattern `Status::InProgress { AssignedTo }`, `AssignedTo` is a **binding**.\
We're **destructuring** the `Status::InProgress` variant and binding the `AssignedTo` field to
a new variable, also named `AssignedTo`.\
If we wanted, we could bind the field to a different variable name:

```rust
match status {
    Status::InProgress { AssignedTo: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("Done");
    }
}
```
