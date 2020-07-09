

pub fn bubble_sort(array:&mut Vec<i32>){
    let len = array.len();
    let mut flag = false;
    while flag == false {
        flag = true;
        for i in 0..len-1 {
            if array[i] > array[i+1] {
                array.swap(i,i+1);
                flag = false;
            }
        }
    }
}

fn merge( array:&mut Vec<i32>, start:usize, end:usize){
    println!("{:?},{},{}",array,start,end);
    if start >= end - 1 {
        return;
    }
    let len = end - start;
    merge(array,start,start+len/2);
    merge(array,start+len/2,end);
    let mut i1 = start;
    let mut i2 = start+len/2;
    let mut buffer = vec![0;len];
    for i in 0..len {
        if i1 < start+len/2 && i2 < end {
            if array[i1] < array[i2] {
                buffer[i] = array[i1];
                i1 = i1+1;
            }else {
                buffer[i] = array[i2];
                i2 = i2+1;
            }
        }else if i1 < start+len/2 {
            buffer[i] = array[i1];
            i1 = i1+1;
        }else if i2 < end {
            buffer[i] = array[i2];
            i2 = i2+1;
        }
    }
    for i in 0..len {
        array[i+start] = buffer[i];
    }
} 

pub fn merge_sort(array:&mut Vec<i32>){
    let len = array.len();
    merge(array,0,len);
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_bubble_sort(){
        let mut a = vec![1,4,5,2,3];
        let mut b = vec![1,4,5,2,3];
        bubble_sort(&mut a);
        b.sort();
        assert_eq!(a,b);
    }

    #[test]
    fn test_merge_sort(){
        let mut a = vec![1,4,5,2,3];
        let mut b = vec![1,4,5,2,3];
        merge_sort(&mut a);
        b.sort();
        assert_eq!(a,b);
    }
}