fn main() {
    
    test_first_prime();
    test_second_prime();
    test_sixth_prime();
    test_big_prime();
}

pub fn nth(n: u32) -> u32 {

    println!("LOOKING FOR NTH PRIME: {} ----------------------------", n);
    
    //n is the index of the nth prime, being 0 the first one, 1 the second, etc

    let mut primesFound : u32 = 0;
    let mut counter : u32 = 1; //Start counting in number 1
    let mut lastPrime : u32 = 0; 

    while primesFound != n+1 {

        //print!("Checking number: {} --- \n", counter);
        if is_prime(counter){

            //print!("Is PRIME --- ");
            primesFound += 1;
            lastPrime = counter;
           // print!("Number of primes found: {} --- Last found: {} \n", primesFound, lastPrime);
        }
        
        counter +=1;
    }

    lastPrime
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}