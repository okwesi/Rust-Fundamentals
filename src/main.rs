fn main() {
  
        // Create an array of integers with 5 elements
        let arr = [1, 2, 3, 4, 5];
    
        // Access an element by its index
        println!("The second element is {}", arr[1]);
    
        // Iterate over the elements of the array
        for x in arr.iter() {
            println!("{}", x);
        }
    
        // Create an array of strings with 3 elements
        let names = ["Alice", "Bob", "Charlie"];
    
        // Modify an element of the array
        names[1] = "Bobby";
    
        // Get the length of the array
        println!("The array has {} elements", names.len());
    }
    
