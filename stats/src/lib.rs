mod statfunctions{

    use std::thread;

    pub fn calculate_mean( list : &Vec<f64>) -> f64 {

        let size : f64 = (*list).len() as f64;

        let mut sum : f64 = 0.0;

        for i in (*list).iter(){
            sum = sum + i;
        }

        return sum / size;
    }

    
    fn square( num : f64)-> f64{
       return num.powf(2 as f64);     
    }

    #[allow(dead_code)]
    pub fn calculate_pearson_correlation( x_values:Vec<f64>, y_values:Vec<f64>) -> (f64, f64) {

        let _x_mean = calculate_mean(&x_values);
        
        let _y_mean = calculate_mean(&y_values);

        let _x_dev = (*x_values).into_iter().map(|x| x - _x_mean as f64);
        let _x_dev_square = (*x_values).into_iter().map(|x| (x - _x_mean as f64).powf(2.0));
        
        let _y_dev = (*y_values).into_iter().map(|y| y - _y_mean as f64);
        let _y_dev_square = (*y_values).into_iter().map(|y| (y - _x_mean as f64).powf(2.0));

        let _zip_xy_dev = _x_dev.into_iter().zip(_y_dev.into_iter()).map(|(x, y)| x * y);

        return (_x_mean as f64, _y_mean as f64);
    }
}



#[cfg(test)]
mod tests {

    use statfunctions;

    #[test]
    fn check_calculate_mean() {
        let mean = statfunctions::calculate_mean(vec![2.0, 3.0, 4.0]);
        assert_eq!(mean, 3.0);
    }

    #[test]
    fn check_pearson_coeff(){
        let (_x_mean, _y_mean) = statfunctions::calculate_pearson_correlation(vec![2.0, 3.0, 4.0], vec![3.0, 2.0, 4.0]);
        assert_eq!(_x_mean, 3.0);
        assert_eq!(_y_mean, 3.0);
    }

}
