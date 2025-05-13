use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("Value: {:?}", map);
}

/*
use std::fmt;
use std::io;

// Como los dos tienen el mismo nombre, necesitamos usar el mismo nombre del padre
// podemos resolver con as para referirnos con nombres distintos para nombres conflictivos.
fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

*/
