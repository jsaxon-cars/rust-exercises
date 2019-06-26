

// pub trait Convertable {
// 	fn convert(&self) -> Convertable {
// 		self
// 	}
// };

#[derive(Debug)]
pub enum Temperature {
	Farenheit(f64), 
	Celcius(f64)
}

#[derive(Debug)]
struct Farenheit(pub f64);

impl Farenheit {
	fn convert(self) -> Celcius {
		Celcius(3.0)
	}
}


#[derive(Debug)]
struct Celcius(pub f64);

impl Celcius {
	fn convert(self) -> Farenheit {
		Farenheit(3.0)
	}
}

pub fn convert(value: Temperature) -> Temperature {
    println!("Value: {:?}", value);
    value.convert()
}



#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_no_elements() {
        let value: f64 = 10.0;
        let result = convert(&value);
        assert_eq!(result, 3.0);
    }

}
