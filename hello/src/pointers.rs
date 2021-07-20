// Reference pointers point to a reference in memory
pub fn run()
{
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    
    // With non-primitives, if you assign another variable to a piece of data, the 
    // first variable no longer holds that value. You will need to use a reference '&'
    // to point to the resource
    
    // Vector ( non primitive)
    let vec1 = vec![1, 2, 3];
    // let vec2 = vec1; // This moves the values from vec1 to vec 2: 
    let vec2 = &vec1;
    
    
    println!("Values: {:?}, {:?}", vec1, vec2);
    println!("Values: {:?}", (&vec1, vec2));
}