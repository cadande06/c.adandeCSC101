fn main(){
    let sales_arr:[u64;5]=[450_000, 1_500_000, 750_000, 2_850_000, 250_000];
    let quantity_arr:[u8;5]=[2, 1, 3, 3, 1];
    let mut sum:u64 = 0;
    let mut fq :u16 = 0;
    
    for x in sales_arr {
        sum+=x;
    }
    for y in quantity_arr {
        fq+=u16::from(y);
    }
    let avg = sum / u64::from(fq);

    println!("sum => {} and average => {}",sum, avg);
}