use std::collections::HashSet;

fn consume_digits(input:&[u8])->(u64,&[u8]){
    let mut sum = 0u64;

    for(i,b) in input.iter().enumerate(){
        let d = *b as i8-b'0' as i8;
        if d<0 || d>9{
            return (sum,&input[i..])
        }
        sum=sum*10+d as u64;
    }
    (sum,&[])
}

fn consume_dash(input:&[u8])->&[u8]{
    if input.get(0)!=Some(&b'-'){
        panic!("expected '-'");
    }
    &input[1..]
}

fn consume_non_digit(input:&[u8])->(&[u8],&[u8]){
    for(i,b) in input.iter().enumerate(){
        let d = *b as i8-b'0' as i8;
        if 0<=d && d<=9 {
            return (&input[..i],&input[i..])
        }
    }
    (input,&[])
}

fn iter_pairs(mut input:&[u8],mut f:impl FnMut((u64,u64))){
    while !input.is_empty(){
        let (first,rest) = consume_digits(input);
        let rest = consume_dash(rest);
        let (second,mut rest) = consume_digits(rest);
        f((first,second));

        (_,rest) = consume_non_digit(rest);
        input = rest;
    }
}



// fn get_half_tens(num:u64)->u64{
//     let mut tens_count = 0;
//     let mut tens = 1;
//     while tens<=num {
//         tens_count+=1;
//         tens*=10;
//     }

//     let half_count =tens_count/2+tens_count%2;
//     let mut half_tens = 1;
//     for _ in 0..half_count{
//         half_tens*=10;
//     }

//     half_tens
// }

// fn get_tens(num:u64)->u64{
//     let mut tens = 1;
//     while tens<=num {
//         tens*=10;
//     }

//     tens
// }



// fn sum_repeated(start:u64,end:u64)->u64{
//     assert!(end>=start);

//     let start_half = get_half_tens(start);

//     let mut ans = 0;

//     let top_start = (start)/start_half;
//     let top_end = (end)/start_half;

//     // println!("sub iter range is {top_start}:{top_end}");


//     for i in top_start..=top_end{
//         let num = i*(1+get_tens(i));
//         if num<=end && num>=start{
//             // print!(" [ has {i}{i} which is {num} ] ");
//             ans+=num;
//         }
//     }

//     ans
// }

fn sum_all_repeated(start:u64,end:u64)->u64{
    assert!(end>=start);
    let mut found = HashSet::new();
    let mut i = 0;
    let mut tens = 1;

    loop {
        i+=1;
        if i == tens {
            tens = tens*10;
        }

        let mut try_num = i*(1+tens);
        if try_num > end {
            return found.into_iter().sum();
        }

        while try_num<=end{
            if try_num >= start {
                found.insert(try_num);
            }

            try_num = try_num*tens+i;
        }
    }
}

fn main() {
    let input = include_str!("input.txt").as_bytes();
    
    let mut sum = 0;

    iter_pairs(input,|(a,b)| {
        // println!("{a}-{b}");
        let s = sum_all_repeated(a,b);
        println!("{a}-{b} has {s}", );
        sum+=s;

    });

    println!("sum is {sum}")
}
