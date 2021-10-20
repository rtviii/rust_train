use std::thread;
use std::collections::HashMap;




struct Cacher <T>
where
T: Fn(u32) -> u32,{
    calculation: T,
    values: HashMap<u32, u32>
}



impl <T> Cacher <T>
where T:Fn(u32)->u32,
{
    fn new(calculation:T) ->Cacher<T> {

        Cacher{
            calculation,
            values:     HashMap::new()
            }
    }



    fn value(&mut self, arg:u32)->u32{
        match self.values.get(&arg)
        {
            Some(v) =>*v,
            None =>{
                let v = (self.calculation)(arg);
                self.values.insert(arg, v+1);
                v+1
            }
        }
    }
}


fn generate_wout(intensity:u32,random_n:u32){
    let mut exp_calc = Cacher::new(|num| {
        println!("Calculating slowly");
        thread::sleep_ms(2000);
        num
    });


    if intensity  <25 {
        println!("Today do {} of this ", exp_calc.value(intensity));
        println!("Today do {} and of that", exp_calc.value(intensity));

    } else{
        if random_n == 3{
            println!("can csalck")
        }else{
            println!("An another one : {}", exp_calc.value(intensity));
        }
    }


}

fn main() {

    generate_wout(2, 4)



}
