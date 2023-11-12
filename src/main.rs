
struct Array<T>{
    arr:Vec<T>
}

impl<T: PartialOrd> Array<T>{
    fn new(arr: Vec<T>) -> Array<T> {
        Array {arr}
    }
    fn sort(&mut self) {
        bubble_sort(&mut self.arr);
    }
}

fn bubble_sort<T>(arr: &mut [T])
    where
        T: PartialOrd
{
    for i in 0..arr.len() - 1 {
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[i] {
                arr.swap(i, j);
            }
        }
    }
}

fn main(){
    let mut int_array = Array::new(vec![34,54,21,56]);
    println!("Before sorting: {:?}", int_array.arr);
    int_array.sort();
    println!("After  sorting: {:?}", int_array.arr);

    let mut float_array = Array::new(vec![1.22,9.222,5.44,7.888]);
    println!("Before sorting: {:?}", float_array.arr);
    float_array.sort();
    println!("After  sorting: {:?}", float_array.arr);

    let mut bool_array = Array::new(vec![true,false]);
    println!("Before sorting: {:?}", bool_array.arr);
    bool_array.sort();
    println!("After  sorting: {:?}", bool_array.arr);

    let mut char_array = Array::new(vec!['2','a','t','1']);
    println!("Before sorting: {:?}", char_array.arr);
    char_array.sort();
    println!("After  sorting: {:?}", char_array.arr);

    let mut string_array = Array::new(vec!["dsf","ytn","gfn","dww"]);
    println!("Before sorting: {:?}", string_array.arr);
    string_array.sort();
    println!("After  sorting: {:?}", string_array.arr);
}

