// NOTE: rename the file to repeated_string.rs or main.rs to compile.

fn repeated_string(s: &str, n: i64) -> Option<i64> {
    // cuento la cantidad de letras 'a' dentro del string
    let mut count: i64 = s
        .chars()
        .filter(|&c| c == 'a')
        .count() as i64;

    // divido la cantidad `n` por el tamaño del string `s`
    // para obtener un número entero y lo multiplico por
    // la cantidad de letras `a` contadas en `count`
    count *= n / s.len() as i64;

    // si el resto de la división entre `n` y el tamaño de `s`
    // no es 0, itero para contar lo que falta
    for i in 0..(n as usize % s.len()) {
        if Some('a') == s.chars().nth(i) {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let mut buffer = String::new();
    for _ in 0..2 {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }

    let input: Vec<&str> = buffer
        .trim()
        .split("\n")
        .collect();

    let s = input[0];
    let n = input[1].parse::<i64>().unwrap();

    if let Some(result) = repeated_string(s, n) {
        println!("{}", result);
    } else {
        println!("Error: result `None`");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeated_string() {
        let s: &str = "aba";
        let n: i64 = 10;
        assert_eq!(Some(7), repeated_string(s, n));

        let s: &str = "a";
        let n: i64 = 1000000000000;
        assert_eq!(Some(1000000000000), repeated_string(s, n));
    }
}
