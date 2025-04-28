use nano_arena::{Arena, ArenaAccess};

fn main() {
    let mut arena = Arena::new();
    let arr = arena.alloc([100, 200, 300, 400]); // allocate an array of 4 integers

    {
        let arr_ref = arena.get_mut(&arr).unwrap();  // get mutable reference to the array

        let (first_half, second_half) = arr_ref.split_at_mut(2);  // split into two slices at index 2

        // SAFE swap using references
        let temp = first_half[0];
        first_half[0] = second_half[0];
        second_half[0] = temp;
    }

    println!("Array after safe swap: {:?}", *arena.get(&arr).unwrap());

    // Now create unsafe leak
    let leaked_val = arena.get(&arr).unwrap() as *const [i32; 4];
    drop(arena);

    unsafe {
        println!("Accessing after free: {:?}", *leaked_val);
    }
}

