struct Monkey {
    items: Vec<i128>,
    operation: fn(&i128, &i128) -> i128,
    operation_value: i128,
    divisor: i128,
    target: (usize, usize),
    count: u128,
}

fn mult(a: &i128, b: &i128) -> i128 {
    a * b
}

fn add(a: &i128, b: &i128) -> i128 {
    a + b
}

fn exp(a: &i128, _: &i128) -> i128 {
    a * a
}

type Monkeys = Vec<Monkey>;
fn parse_lines(contents: &str) -> Result<Monkeys, String> {
    let mut monkeys: Monkeys = Vec::new();

    let monkey_strs = contents.split("\n\n");
    for monkey_str in monkey_strs {
        let lines = monkey_str.lines();
        let mut lines = lines.skip(1); // Monkey #

        let items: Vec<i128> = lines
            .next()
            .ok_or("Invalid items line")?
            .split_once(": ")
            .ok_or("Invalid items line")?
            .1
            .split(", ")
            .map(|x| x.parse::<i128>().unwrap())
            .collect();

        let op_line = lines
            .next()
            .ok_or("Invalid operation line")?
            .split(' ')
            .collect::<Vec<&str>>();
        let operation = match op_line[op_line.len() - 2] {
            "*" => match op_line[op_line.len() - 1] {
                "old" => exp,
                _ => mult,
            },
            "+" => add,
            _ => panic!("Bad operator, why can't we handle this with an result/error or something"),
        };
        let operation_value = op_line
            .last()
            .ok_or("Couldn't parse operation line")?
            .parse::<i128>()
            .unwrap_or_default();
        let divisor = lines
            .next()
            .ok_or("Invalid divisor line")?
            .split(' ')
            .last()
            .ok_or("Couldn't parse operation line")?
            .parse::<i128>()
            .or(Err("Couldn't parse int"))?;

        let true_target = lines
            .next()
            .ok_or("Invalid true line")?
            .split(' ')
            .last()
            .ok_or("Couldn't parse true line")?
            .parse::<usize>()
            .or(Err("Couldn't parse usize"))?;

        let false_target = lines
            .next()
            .ok_or("Invalid false line")?
            .split(' ')
            .last()
            .ok_or("Couldn't parse false line")?
            .parse::<usize>()
            .or(Err("Couldn't parse usize"))?;
        monkeys.push(Monkey {
            items,
            operation,
            operation_value,
            divisor,
            target: (true_target, false_target),
            count: 0,
        });
    }
    Ok(monkeys)
}

pub fn solve_a(contents: &str) -> Result<String, String> {
    let mut monkeys = parse_lines(contents)?;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items = Vec::new();
            for item in items {
                monkeys[i].count += 1;
                let item = (monkeys[i].operation)(&item, &monkeys[i].operation_value) / 3;
                if item % monkeys[i].divisor == 0 {
                    let target = monkeys[i].target.0;
                    monkeys[target].items.push(item);
                } else {
                    let target = monkeys[i].target.1;
                    monkeys[target].items.push(item);
                }
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    Ok(format!("{}", monkeys[0].count * monkeys[1].count))
}

pub fn solve_b(contents: &str) -> Result<String, String> {
    let mut monkeys = parse_lines(contents)?;
    let mut cm = 1;
    for monkey in &monkeys {
        cm *= monkey.divisor;
    }
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items = Vec::new();
            for item in items {
                monkeys[i].count += 1;
                let item = (monkeys[i].operation)(&item, &monkeys[i].operation_value) % cm;
                if item % monkeys[i].divisor == 0 {
                    let target = monkeys[i].target.0;
                    monkeys[target].items.push(item);
                } else {
                    let target = monkeys[i].target.1;
                    monkeys[target].items.push(item);
                }
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| a.count.cmp(&b.count).reverse());
    Ok(format!("{}", monkeys[0].count * monkeys[1].count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let contents = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let result = solve_a(contents);
        assert_eq!(result.unwrap(), "10605");
    }

    #[test]
    fn test_b() {
        let contents = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let result = solve_b(contents);
        assert_eq!(result.unwrap(), "2713310158");
    }
}
