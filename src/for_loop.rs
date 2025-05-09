pub fn for_loop(){
    let v:Vec<u32> = vec![1,2,3];
    for ele in &v{
        println!("{:?}",ele);
    }
}