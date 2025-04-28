
use nano_arena::{Arena, ArenaAccess};

fn main() {
    let mut arena = Arena::new();
    let arr = arena.alloc([100, 200, 300, 400]); // allocate an array of 4 integers

    {
        let arr_ref = arena.get_mut(&arr).unwrap();  // get mutable reference to the array

        let (first_half, second_half) = arr_ref.split_at_mut(2);  // split into two slices at index 2

        // Safe swap between halves
        let temp = first_half[0];
        first_half[0] = second_half[0];
        second_half[0] = temp;
    }

    println!("Array after safe swap: {:?}", *arena.get(&arr).unwrap());
}
