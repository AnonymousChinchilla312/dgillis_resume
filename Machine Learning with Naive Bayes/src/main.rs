use core::num;
use std::any::Any;
use rand::prelude::*;
use std::io;
use std::io::Read;

fn main() {
    // Read Iris file
    let mut iris_file = std::fs::File::open("C:/Users/gilli/OneDrive/Documents/Rust/project/src/iris.csv").unwrap();
    let mut iris_contents = String::new();
    iris_file.read_to_string(&mut iris_contents).unwrap();
    // println!("{}", iris_contents);

    // Create data vec
    let mut iris_table:Vec<&str> = iris_contents.split_whitespace().collect();
    let iris_train_length = iris_table.len() * 7 / 10;
    let mut iris_train_table:Vec<&str> = vec![];
    let mut num_of_matches = 0.0;

    let mut rng = thread_rng();
    while iris_train_table.len() < iris_train_length {
        let rand_num = rng.gen_range(0..iris_table.len());
        iris_train_table.push(iris_table.remove(rand_num));
        // println!("Num: {}", iris_train_table.len());
    }

    // let new_flower = gnb_get_new();
    // println!("{}", gnb_predict(iris_train_table, &new_flower[0], &new_flower[1], &new_flower[2], &new_flower[3]));

    for x in 0..iris_table.len() {
        // println!("{}", x);
        let new_flower = unpack_iris_string(iris_table[x].to_string());
        // println!("{:?}", iris_train_table.clone());

        // println!("{}", gnb_predict(iris_train_table.clone(), &new_flower[0], &new_flower[1], &new_flower[2], &new_flower[3]));
        // println!("{}", get_iris_species(iris_table[x].to_string()));

        if gnb_predict(iris_train_table.clone(), &new_flower[0], &new_flower[1], &new_flower[2], &new_flower[3]) == get_iris_species(iris_table[x].to_string()) {
            num_of_matches += 1.0;
        }
    }

    println!("Accuracy {}", num_of_matches / iris_table.len() as f64)

}

fn unpack_iris_string(iris_string: String) -> [f64; 4] {
    let mut new_sl = 0.0;
    let mut new_sw = 0.0;
    let mut new_pl = 0.0;
    let mut new_pw = 0.0;

    for i in iris_string.split(",").enumerate() {
        // let f = j.1.parse::<f64>().unwrap();
        // println!("{}", f);
        
        match i.0 {
            0 => new_sl = i.1.parse::<f64>().unwrap(),
            1 => new_sw = i.1.parse::<f64>().unwrap(),
            2 => new_pl = i.1.parse::<f64>().unwrap(),
            3 => new_pw = i.1.parse::<f64>().unwrap(),
            4 => break,
            _ => println!("Something went wrong."),
        }
    }

    return [new_sl, new_sw, new_pl, new_pw];
}

fn get_iris_species(iris_string: String) -> String {
    let mut species: String = "".to_string();

    for i in iris_string.split(",").enumerate() {
        // let f = j.1.parse::<f64>().unwrap();
        // println!("{}", f);
        
        match i.0 {
            0 => continue,
            1 => continue,
            2 => continue,
            3 => continue,
            4 => species = i.1.to_string(),
            _ => println!("Something went wrong."),
        }

    }    
    return species;
}

fn gnb_get_new() -> [f64; 4] {
    // Sepal Length
    print!("Input the sepal length of the new flower: ");
    println!();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    let mut trimmed_string = input_text.trim();
    
    let new_sl: f64 = trimmed_string.parse().unwrap();

    // Sepal Width
    print!("Input the sepal width of the new flower: ");
    println!();
    input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    trimmed_string = input_text.trim();
    
    let new_sw: f64 = trimmed_string.parse().unwrap();

    // Petal Length
    print!("Input the petal length of the new flower: ");
    println!();
    input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    trimmed_string = input_text.trim();
    
    let new_pl: f64 = trimmed_string.parse().unwrap();

    // Petal Width
    print!("Input the petal width of the new flower: ");
    println!();
    input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

    trimmed_string = input_text.trim();
    
    let new_pw: f64 = trimmed_string.parse().unwrap();

    let result = [new_sl, new_sw, new_pl, new_pw];
    return result;
}

fn gnb_predict(table: Vec<&str>, new_sl: &f64, new_sw: &f64, new_pl: &f64, new_pw: &f64) -> String {
    // println!("{}", sv1[0]);

    let mut current_new_sl = 0.0;
    let mut current_new_sw = 0.0;
    let mut current_new_pl = 0.0;
    let mut current_new_pw = 0.0;

    let mut setosa_sl_num = 0.0;
    let mut setosa_sw_num = 0.0;
    let mut setosa_pl_num = 0.0;
    let mut setosa_pw_num = 0.0;
    let mut setosa_count = 0.0;

    let mut versicolor_sl_num = 0.0;
    let mut versicolor_sw_num = 0.0;
    let mut versicolor_pl_num = 0.0;
    let mut versicolor_pw_num = 0.0;
    let mut versicolor_count = 0.0;

    let mut virginica_sl_num = 0.0;
    let mut virginica_sw_num = 0.0;
    let mut virginica_pl_num = 0.0;
    let mut virginica_pw_num = 0.0;
    let mut virginica_count = 0.0;

    // Mean values
    for i in 0..table.len() {
        for j in table[i].split(",").enumerate() {
            // let f = j.1.parse::<f64>().unwrap();
            // println!("{}", f);
            match j.0 {
                0 => current_new_sl = j.1.parse::<f64>().unwrap(),
                1 => current_new_sw = j.1.parse::<f64>().unwrap(),
                2 => current_new_pl = j.1.parse::<f64>().unwrap(),
                3 => current_new_pw = j.1.parse::<f64>().unwrap(),
                4 => if j.1 == "Iris-setosa" {
                    // println!("{}", current_new_sl);
                    setosa_sl_num += current_new_sl;
                    setosa_sw_num += current_new_sw;
                    setosa_pl_num += current_new_pl;
                    setosa_pw_num += current_new_pw;
                    setosa_count += 1.0;
                } else if j.1 == "Iris-versicolor" {
                    versicolor_sl_num += current_new_sl;
                    versicolor_sw_num += current_new_sw;
                    versicolor_pl_num += current_new_pl;
                    versicolor_pw_num += current_new_pw;
                    versicolor_count += 1.0;
                } else if j.1 == "Iris-virginica" {
                    virginica_sl_num += current_new_sl;
                    virginica_sw_num += current_new_sw;
                    virginica_pl_num += current_new_pl;
                    virginica_pw_num += current_new_pw;
                    virginica_count += 1.0;
                },
                _ => println!("Something went wrong."),
            }
        }
    }
    // println!("{} over {}", virginica_sl_num, virginica_count);

    // Calculate Means
    let setosa_sl_mean = setosa_sl_num / setosa_count;
    let setosa_sw_mean = setosa_sw_num / setosa_count;
    let setosa_pl_mean = setosa_pl_num / setosa_count;
    let setosa_pw_mean = setosa_pw_num / setosa_count;

    let versicolor_sl_mean = versicolor_sl_num / versicolor_count;
    let versicolor_sw_mean = versicolor_sw_num / versicolor_count;
    let versicolor_pl_mean = versicolor_pl_num / versicolor_count;
    let versicolor_pw_mean = versicolor_pw_num / versicolor_count;

    let virginica_sl_mean = virginica_sl_num / virginica_count;
    let virginica_sw_mean = virginica_sw_num / virginica_count;
    let virginica_pl_mean = virginica_pl_num / virginica_count;
    let virginica_pw_mean = virginica_pw_num / virginica_count;

    // Variance values
    setosa_sl_num = 0.0;
    setosa_sw_num = 0.0;
    setosa_pl_num = 0.0;
    setosa_pw_num = 0.0;
    setosa_count = 0.0;

    versicolor_sl_num = 0.0;
    versicolor_sw_num = 0.0;
    versicolor_pl_num = 0.0;
    versicolor_pw_num = 0.0;
    versicolor_count = 0.0;

    virginica_sl_num = 0.0;
    virginica_sw_num = 0.0;
    virginica_pl_num = 0.0;
    virginica_pw_num = 0.0;
    virginica_count = 0.0;

    for i in 0..table.len() {
        for j in table[i].split(",").enumerate() {
            // let f = j.1.parse::<f64>().unwrap();
            // println!("{}", f);
            match j.0 {
                0 => current_new_sl = j.1.parse::<f64>().unwrap(),
                1 => current_new_sw = j.1.parse::<f64>().unwrap(),
                2 => current_new_pl = j.1.parse::<f64>().unwrap(),
                3 => current_new_pw = j.1.parse::<f64>().unwrap(),
                4 => if j.1 == "Iris-setosa" {
                    // println!("{}", current_new_sl);
                    setosa_sl_num += (current_new_sl - setosa_sl_mean).powf(2.0);
                    setosa_sw_num += (current_new_sw - setosa_sw_mean).powf(2.0);
                    setosa_pl_num += (current_new_pl - setosa_pl_mean).powf(2.0);
                    setosa_pw_num += (current_new_pw - setosa_pw_mean).powf(2.0);
                    setosa_count += 1.0;
                } else if j.1 == "Iris-versicolor" {
                    versicolor_sl_num += (current_new_sl - versicolor_sl_mean).powf(2.0);
                    versicolor_sw_num += (current_new_sw - versicolor_sw_mean).powf(2.0);
                    versicolor_pl_num += (current_new_pl - versicolor_pl_mean).powf(2.0);
                    versicolor_pw_num += (current_new_pw - versicolor_pw_mean).powf(2.0);
                    versicolor_count += 1.0;
                } else if j.1 == "Iris-virginica" {
                    virginica_sl_num += (current_new_sl - virginica_sl_mean).powf(2.0);
                    virginica_sw_num += (current_new_sw - virginica_sw_mean).powf(2.0);
                    virginica_pl_num += (current_new_pl - virginica_pl_mean).powf(2.0);
                    virginica_pw_num += (current_new_pw - virginica_pw_mean).powf(2.0);
                    virginica_count += 1.0;
                },
                _ => println!("Something went wrong."),
            }
        }
    }

    // Calculate Standard Deviations
    let setosa_sl_sd = (setosa_sl_num / setosa_count).sqrt();
    let setosa_sw_sd = (setosa_sw_num / setosa_count).sqrt();
    let setosa_pl_sd = (setosa_pl_num / setosa_count).sqrt();
    let setosa_pw_sd = (setosa_pw_num / setosa_count).sqrt();

    let versicolor_sl_sd = (versicolor_sl_num / versicolor_count).sqrt();
    let versicolor_sw_sd = (versicolor_sw_num / versicolor_count).sqrt();
    let versicolor_pl_sd = (versicolor_pl_num / versicolor_count).sqrt();
    let versicolor_pw_sd = (versicolor_pw_num / versicolor_count).sqrt();

    let virginica_sl_sd = (virginica_sl_num / virginica_count).sqrt();
    let virginica_sw_sd = (virginica_sw_num / virginica_count).sqrt();
    let virginica_pl_sd = (virginica_pl_num / virginica_count).sqrt();
    let virginica_pw_sd = (virginica_pw_num / virginica_count).sqrt();

    // println!("{}, {}, {}, {}", setosa_sl_sd, setosa_sw_sd, setosa_pl_sd, setosa_pw_sd)

    // Normal Distributions
    // Setosa
    let nd_new_sl_setosa_exp = -((new_sl - setosa_sl_mean).powf(2.0) / (2.0 * (setosa_sl_sd.powf(2.0))));
    let nd_new_sl_setosa = (1.0 / (setosa_sl_sd * (2.0_f64 * 3.14159).sqrt())) * 2.71828_f64.powf(nd_new_sl_setosa_exp);

    let nd_new_sw_setosa_exp = -((new_sw - setosa_sw_mean).powf(2.0) / (2.0 * (setosa_sw_sd.powf(2.0))));
    let nd_new_sw_setosa = (1.0 / (setosa_sw_sd * (2.0_f64 * 3.14159).sqrt())) * 2.71828_f64.powf(nd_new_sw_setosa_exp);

    let nd_new_pl_setosa_exp = -((new_pl - setosa_pl_mean).powf(2.0) / (2.0 * (setosa_pl_sd.powf(2.0))));
    let nd_new_pl_setosa = (1.0 / (setosa_pl_sd * (2.0_f64 * 3.14159).sqrt())) * 2.71828_f64.powf(nd_new_pl_setosa_exp);

    let nd_new_pw_setosa_exp = -((new_pw - setosa_pw_mean).powf(2.0) / (2.0 * (setosa_pw_sd.powf(2.0))));
    let nd_new_pw_setosa = (1.0 / (setosa_pw_sd * (2.0_f64 * 3.14159).sqrt())) * 2.71828_f64.powf(nd_new_pw_setosa_exp);

    let nd_new_total_setosa = nd_new_sl_setosa * nd_new_sw_setosa * nd_new_pl_setosa * nd_new_pw_setosa;

    // Versicolor
    let nd_new_sl_versicolor_exp = -((new_sl - versicolor_sl_mean).powf(2.0) / (2.0 * (versicolor_sl_sd.powf(2.0))));
    let nd_new_sl_versicolor = (1.0 / (versicolor_sl_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_sl_versicolor_exp);

    let nd_new_sw_versicolor_exp = -((new_sw - versicolor_sw_mean).powf(2.0) / (2.0 * (versicolor_sw_sd.powf(2.0))));
    let nd_new_sw_versicolor = (1.0 / (versicolor_sw_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_sw_versicolor_exp);

    let nd_new_pl_versicolor_exp = -((new_pl - versicolor_pl_mean).powf(2.0) / (2.0 * (versicolor_pl_sd.powf(2.0))));
    let nd_new_pl_versicolor = (1.0 / (versicolor_pl_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_pl_versicolor_exp);

    let nd_new_pw_versicolor_exp = -((new_pw - versicolor_pw_mean).powf(2.0) / (2.0 * (versicolor_pw_sd.powf(2.0))));
    let nd_new_pw_versicolor = (1.0 / (versicolor_pw_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_pw_versicolor_exp);

    let nd_new_total_versicolor = nd_new_sl_versicolor * nd_new_sw_versicolor * nd_new_pl_versicolor * nd_new_pw_versicolor;

    // Virginica
    let nd_new_sl_virginica_exp = -((new_sl - virginica_sl_mean).powf(2.0) / (2.0 * (virginica_sl_sd.powf(2.0))));
    let nd_new_sl_virginica = (1.0 / (virginica_sl_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_sl_virginica_exp);

    let nd_new_sw_virginica_exp = -((new_sw - virginica_sw_mean).powf(2.0) / (2.0 * (virginica_sw_sd.powf(2.0))));
    let nd_new_sw_virginica = (1.0 / (virginica_sw_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_sw_virginica_exp);

    let nd_new_pl_virginica_exp = -((new_pl - virginica_pl_mean).powf(2.0) / (2.0 * (virginica_pl_sd.powf(2.0))));
    let nd_new_pl_virginica = (1.0 / (virginica_pl_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_pl_virginica_exp);

    let nd_new_pw_virginica_exp = -((new_pw - virginica_pw_mean).powf(2.0) / (2.0 * (virginica_pw_sd.powf(2.0))));
    let nd_new_pw_virginica = (1.0 / (virginica_pw_sd * (2.0_f64 * 3.14159).sqrt())) * 2.718_f64.powf(nd_new_pw_virginica_exp);

    let nd_new_total_virginica = nd_new_sl_virginica * nd_new_sw_virginica * nd_new_pl_virginica * nd_new_pw_virginica;

    // println!("{}, {}, {}", nd_new_total_setosa, nd_new_total_versicolor, nd_new_total_virginica);
    if nd_new_total_setosa > nd_new_total_versicolor && nd_new_total_setosa > nd_new_total_virginica {
        // println!("I predict this flower is a setosa.");
        return "Iris-setosa".to_string();
    } else if nd_new_total_versicolor > nd_new_total_virginica {
        // println!("I predict this flower is a versicolor.");
        return "Iris-versicolor".to_string();
    } else {
        // println!("I predict this flower is a virginica.");
        return "Iris-virginica".to_string();
    }
}