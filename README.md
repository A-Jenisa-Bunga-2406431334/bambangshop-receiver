#### Reflection Subscriber-1

1. We used RwLock<> instead of Mutex<> because RwLock allows multiple 
readers to access the data simultaneously, while only one writer can 
access it at a time. This is necessary for our case because we have many 
read operations (list_all_as_string) and fewer write operations (add). 
Using Mutex<> would only allow one thread to access the data at a time, 
even for read operations, which would be unnecessarily restrictive and 
less performant. Since our Vec of Notifications is read more often than 
it is written, RwLock<> is the better choice.

2. Rust does not allow us to mutate static variables directly because of 
its strict ownership and borrowing rules. In Java, static variables can 
be mutated freely, but this can lead to race conditions in multi-threaded 
environments. Rust enforces thread safety at compile time, so to mutate 
a static variable, we need to use thread-safe wrappers like RwLock or 
Mutex. The lazy_static library allows us to define complex static variables 
that are initialized lazily (on first use) rather than at compile time, 
which is necessary because Rust cannot evaluate complex expressions at 
compile time for static variables