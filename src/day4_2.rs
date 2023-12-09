use anyhow::{anyhow, Result};
use std::collections::HashSet;

struct Adder {
    instances: usize,
    lifetime: usize,
}

fn process(input: &str) -> Result<usize> {
    let mut total_instances = 0;
    let mut stack: Vec<Adder> = vec![];

    for line in input.lines() {
        let mut sections = line
            .split(": ")
            .nth(1)
            .ok_or(anyhow!("Expected line be to be splittable by \": \""))?
            .split(" | ");

        let winning: HashSet<usize> = sections
            .next()
            .ok_or(anyhow!("Expected line be to splittable by \" | \""))?
            .split_ascii_whitespace()
            .filter_map(|c| if let Ok(n) = c.parse() { Some(n) } else { None })
            .collect();
        let numbers: HashSet<usize> = sections
            .next()
            .ok_or(anyhow!("Expected line be to splittable by \" | \""))?
            .split_ascii_whitespace()
            .filter_map(|c| if let Ok(n) = c.parse() { Some(n) } else { None })
            .collect();

        let score = numbers.intersection(&winning).count();
        let copies = stack.iter().fold(0, |a, s| a + s.instances);
        let instances = 1 + copies;

        total_instances += instances;

        stack.retain_mut(|i| {
            i.lifetime -= 1;
            i.lifetime > 0
        });

        if score > 0 {
            stack.push(Adder {
                instances,
                lifetime: score,
            });
        }
    }

    Ok(total_instances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(process(input)?, 30);

        Ok(())
    }
}

pub fn main() -> Result<()> {
    let input = "Card   1: 20 72 30 38 18 65  6 55 70 27 | 12 28 47 50 60 17 14 25 41 95 66 88 61 52 76  5 23 77 31 32 99 89 53 54 96
Card   2: 15 98 12  3 20 60 58 54 34 18 | 98 23 12 19 61 38 11 43 58 97 63 10 49 67 44 52 88  4 22  1 42 65 20 13 25
Card   3: 47 97  2 80 89 56 66 85 62 46 | 78 97 47 14 15 85 42 66 24 28 54 46 89 62 80 56 22  5 57 52 69  2  3 95 19
Card   4: 60 59 88 14 53 46 96 29 99 37 | 52 46 92 26 72 74 99 38  7 65 43 63 39 36 44  9 56 42 79 50 89 48 85 28 27
Card   5: 44 46 42  9 65 98 97 67 72 22 | 80 14 29 39 98 64  9 46 52 24 69 22 51 65 66 20 72 21 55 12 97 42 44 41 67
Card   6: 53 92  3 77 46 68 11 23 81 88 | 42  3 92  4 23 43 62 81 51  1 29 88 35 68 84 95 59 77 11 36 53 76 49 46 34
Card   7: 66 24 96 25 11 60 15 18 67 87 | 25 66 96 11 87 31 57 56 15 18 27  1 98 40 22 60 86 80 10 97 67 24 64 59 81
Card   8: 76 38 87 23 60 16  7 18 70 14 | 50 80 91 16 89 76 63 32 26  1 38 77 35 28 68 12 97  7 43 34 37 39 98  8 70
Card   9: 72 71 34 45 75 24 66 65 23  7 | 71 58 74 65 51 83 34 66 78 45 46 24 75 80  7 72 91 94 53 28 76 96 23 48 17
Card  10:  3 86 65 97 67 20 16 96 21 26 | 26 58 82 64 16 59  4 15 72 35 67 86 93 37  3 21 65 96 85  1 68 13 62 20 76
Card  11: 11  1 45 82 22 87 28 35 42 85 | 65 98 77 11 82 28  6  8 42 45 49 17 87  9 85  1 59  4 93 37 89 57 16 22 68
Card  12: 86 57 23 63  1 99  4 49 81 73 | 73 23  4 81 18 52 88 14 76 65 42 99 48 77 17  6 58 86 95 63 57 49 85  1 37
Card  13: 26 40 52  6 63 87 20  8 31 76 | 75 80  9 94 12 55 50 27 71 90 40  7 24 25 85 57 56 48 36 63 52  1 73 86 68
Card  14:  7 22 70 62 63 37 99  6 44 36 | 56 64 70 31 66 77  2 39 18 71 87 33 60 19 30 90 74 40 63 24 57 38 75 53 49
Card  15: 16 49 27  2  3 42 32 80 96 39 | 53 41 42 34 75 93 54 52 23 82 27 19 46  1 77 70 45 91 13 63 50 72 97  5 68
Card  16: 14 47 22 49 42 86 48 39 23 66 | 95 71  3 24 65 42 92 30 37 34 10 79 51 55 96 66 13 58 22 56 94 64 32 52 54
Card  17: 26 70 20 56 93 69  1 98 47 96 | 44 35  4 24 62 10  1 30  3 37 47 94 15 43 40 46 82 16 85 41 57 70 56 36 75
Card  18: 34  7 25  1  6 28 63 94 16 12 | 69 45 99  1 52 65 36 94 59  6 71 89 30 51 68 15 49  5  3 25 28 60 20 98 54
Card  19: 58 78 61 28 82 69 48 56 52  2 | 73 79 74  7  5 20  4 80 22 11 36 14 93 97 63 96 70 99 45 91 15 32  1 72 59
Card  20: 70 83 13 28 97 90 23 78 57 37 | 95 44 96 50 49 87 71 35 75 41 16 18 31 42 70 56 84 23 60 63 43 38 90  1 66
Card  21: 95 36 59 50 86 23 85 33 55 10 | 68 26 10 41 82 11 87 88 55  2 21 52 67 54 37 92 31 32 71 99 25 45 46 48 76
Card  22: 40  2 67  3 52 86 53 55 66 56 | 60 51  9 77 74 69 19  6 75 64 73 59 15  7 36 99 57 16 27 95 83 41 24 38 71
Card  23: 59 43 29 35 58 36 70  4 38 80 | 29 87 65 94 96 40 99 77 89 66 45 62 84 91 85 16  7 27 74 28 92 72 49 13  8
Card  24: 95 43 44 15 89 58  7 98  8 48 | 75 21 45 97 62 54 76 96 51 61 28 35 74 83 36 10  5  9 78 50 46 52 67 93 23
Card  25: 31 23 93 37 75 74 36 66 38 51 | 49 38 74 13 66 75 93 14 23 65 31 17 98 21 87 32 37  1 20 54 90 34 22 25  3
Card  26: 98 19 41 93 21 55  5 58 49 38 | 69 38 20  5 34 57 28 81 58 42 53 25 19 21 76  4 92  8 97 49 95 55 98 93 41
Card  27: 85 76 44 66 11 45 73 96 48  3 |  3 32 98 83 66 84 41 92 45 21 76 85 52 61 73  6 38 11 44 30 48 69 96 91 27
Card  28: 62 94 36 44 76 97 37 69 63 42 | 68 90 81 96  4 57 26 56  7 72 47 10 44 98 24 23 32  6 80 60 37  9 61 36 35
Card  29: 48 17 96 76 61 78 86 98 65 68 | 42 98 44 80 48 40 55 76 17 27 34 63 86 68 53 13 96 56 78 65 33 61  7  3 45
Card  30: 23  1 20  9 86 68 32 79 71  4 | 11  2 43 97 61  7 91 99 34 51 72 95 49 10 83 21  4 23 90 78 54 12 82 46 58
Card  31: 81  2 32 17 56 80 65 74 86 39 | 17 50 29 67 51 47 73 14 87 81 91 72 53 41 80 39 86 37 24 61 63 88 58  2 49
Card  32: 91 67 96 32 81 42 74 36 51  8 | 69  2 40 36 16 83  8 37 35 12 96 68 50 79 11 51  9 74  5 71  6 14 94 82 61
Card  33: 16 81 96  7  9 67 92 29 53 37 |  6 91  2 39 15  9 34 17 88 23 38 43 65  5 53 69 54  8 63 86 62  3 40 96  7
Card  34: 12  1 14  5 52 72 31 49 95 96 |  2 95 26  6 61 30 36 77  7 39 96 88 94 64 83 18  5 16 93 54 15 38  1 47 14
Card  35: 96 28  7 47 71 61 23  8 77 83 | 66 35 42 43 57 78 19 24 95 25 87 46 76 29  9 58 85 99 59 92 86 28 36 32 55
Card  36: 81 49 71 63 98 28 88 56  3 96 | 22 75 79 50 36 44 63 58 25 78 91 70 86 90 55 71 26 43 23 76 99 54 48 57 14
Card  37: 20 54 55 16 35 30 62 87 75 12 | 83 67 76 63 72 12 74 24 33 91 36 46 11 35 48 95  6 43 17 50 10 26 38 42 23
Card  38: 78 46 51 33 11 83 36 76 39  3 | 71 96 36 79 54 70 35 65  9 19 50 44  2 53 95 86 85 90 32 91  6 84 31 77 26
Card  39: 14 95 72 15 65 66 68 38 33 82 | 52 90 32 96 64 48 39 28 51 69 30 20 57  1 75 63 37 94 31 62 40 41 70 36 10
Card  40: 98 73 69 31 42 89 75 51 63 14 | 75 59 43 89 39 36 63 73 12 42 37 31 53 85 19 27 69 38 98 64 32 51 14 82 50
Card  41: 43 89 45 96 30 91 73 64 90 34 | 89 91 30 68 78 45 43 60 70  1 64 38 17 72 83 80  5 20 34 82 57 98 87  2 96
Card  42: 62 79 67 48 77 64 37 47 96 24 | 30 99 15 98 88 76 53 93 36 55 17  4 13 66 27  5 22 45 38 63 21 80 47 85 56
Card  43: 50 11 19  8 88 25 66 48 79 55 | 48 44 88 79 11 13  2 25 76 20 67 50 28 15 19 78 60  5 72 66  8 59 18 61 24
Card  44: 70 53 10 52 54 77 12 14 57 50 | 99 70 59 77 79 55  6 46 85 92  1 62 78 83 35 81 71 97 69 16 41 20 84 67 15
Card  45: 37 52  2 34 44 13 23 76 12 51 | 73 94 53 39 92 64  2 13 16 37 72 52 61  9 96 57 23  7 43 78  6 66 76 51 45
Card  46: 35 54 63 88  2 77 33 23 62  4 | 94 23  5 69 63 92 46 49 57 35 88 78  2 77 97 34 54 33 85  8  4 66 55 91 39
Card  47: 76 28 61 31 66 37 83 22 39 98 | 48 75 50 91 81 72  4 61 98 66 29 45 99 16 64 28 60 74 59 49 79 97 10 32 31
Card  48: 46 58 24 97 65 95 35 94  2 25 | 59 57 87 98 65 66 71 95 20 70 97 55 85 46  6 25 78 15 80 24 37 53 72 17 54
Card  49: 68 79 50 89 86 37  5 95 60 81 | 74 81 71 60 45 95 44 76 78 17 46 62 50 70  5 93 85 64 21 49 14 39  1 80 16
Card  50: 94 95 54 64 23 55 62 73  9 53 | 18  9  5 47 83 84 99 34 21  1 29 95  8 17 42 22 19 40 82 16 81 10 53 55  6
Card  51: 48 41 39 70 63 35 14 54 64 86 |  1 89  3 23 59 88 10 31 95 30 44 52 33 37 69 21 68 99 80 16  2 67 20 47 13
Card  52: 47 80 72 53 81 83  3 31 14 92 | 42  8 28 11 47 37 84 13 73 71 43 26 58 51 52 66 64 34 18 54 16 68 31 50 10
Card  53: 58 63 27 43 42 28 97 32 44  1 | 62 93 40 91 31 89 55 38 65 56 96 79 49 75 25 32 48 80 12 77  4 37 45 46 11
Card  54: 11 34 90 95 82 80 50 38 63 22 | 96 13 84 92 55 32 22 71 68 62 36  9 87 15 49 72 56 93 61 10 67 24  6 76 69
Card  55: 40 55 73 16 99 57 29 93 63  8 | 27 45 62 64 34 85 65 69 43 28 52 58 82 15 87 56  5 23 39 20 92  4 30  1 77
Card  56: 74 43 20  8 72 35 64 75 28 62 | 99 34 93 94 17  3 58 22 68 39 95 69 83 70 32 98 16 60 29 84 51 86 48 97 77
Card  57: 32 21 19 66 10  9 88 95 63 83 | 14  5 17 87  1 63 21 95 32 19  9 18 67 36 70 82 41 96 10 66 83 81 43 88 74
Card  58:  2 12 70 55 19 93 43 32 85 36 | 26 87 97 82 19 76 56 32  8 36 13 12 35 72 60 70 85 43 53 11  2 55 44 96 93
Card  59: 62 72 16 21 42 20 65 61 81 38 | 51 53 62  5 87 73 70 27 85 55 37 42 24 20 65 82 88 96  9  6 25 26 44 29 79
Card  60: 39 64 74 98 92 80 27 19 23 95 | 92 78 65 77 29 64 83 19  1 95 26 80 57  3 93 72 11  9 30 82 61 45 74 98 50
Card  61: 17 81 77 57 87 22 69  4 28 86 | 81 48 69 97 27 77 67  6 95  3 94 37 58 36 12 59 17 61 64 15 60 87 62 56 78
Card  62: 71 95 10 93 14 68 56 32 86 76 | 40 86 97 54 93 76 68 92 14  3 15 10 82 12 55 32 85 71 60 67 31 95  5 56 35
Card  63: 47  2 80 90 48  6 11 70  5 88 | 90 27 54  4 26 44 58 14 88  1 57 11 35 78  6 55 63 43 59 29 70 49 77 74 45
Card  64: 43 66 45 28 34  4 65 97 62  7 | 92 28 17 54 66 65 81 57  7 69 61 47 45 62  9 80 34 74 23 43 95  4 42 97 31
Card  65: 13 50 33 21 12 86 82 40 79 51 | 48 21 65 41 15 80 44 70 36 33 67  3 58 62 86 38  8 79 64 35 87 82  6 74  5
Card  66: 23 35 77 73 54 32 49 34 95 94 | 97  2 31 86 87 47 36 27  9 75 20 66 94 84 30 80 41 33 42 85 73  4  5 54 53
Card  67: 42 84 81 26 87 44 74 37 70 51 | 44 81 37 87 43 47 50 13 15 52 67 59 61 49 66 79 51 19 27 22 60  8 42 74 23
Card  68: 95 44 10 48 47 80 70 16 87 29 | 40 19 75 91 77 50 53 95  9 66 86 33  5 28 23  4 17 72 94 22 48 32 51 55 61
Card  69: 88 58 22 91 51 18 69 13 15 32 | 42 67 13 51 23 87 57 97 40 34 54 91 73 15 50 89 56  3 58 66 69 11 22 16  7
Card  70: 50 15 67  5  2 31 92 64 74 49 | 68  1 70 16  5 69 37 44 75 30 88 95 57  8 81 80 55 41 22 21 29 62 83 59 12
Card  71: 92 41  1 34 73 66 79 24 63 43 | 79 16 72 88 57 95  7  1 47 70 39 24 93 67 20 29 48 69 30 92 31 59 15 43 85
Card  72: 31 39 48 36 65 99 53 18 77 30 | 76 37 11 41  5 94 93 87 24 30 82 85 23 52 46 22 96  9 20 32 33 60 21 97 31
Card  73: 73 72 28  6 61 79 51 76 99 12 | 20 89 82 90 11 87 18 16 84 26 95 14 45 65 44 31 23 42 34 62 79 91 66 78 85
Card  74: 33 94 91 48 63 87 18  6 23 54 | 64 38 75 76 31 17 80 90 49 72 27  4 18 34 29 26  9 84 20 96 28 88  8 59 68
Card  75: 22 93 71 16  5 52 78 74 84 36 | 63  2 24 97 21 42 39 78  1 91 76  6 23 29 28 11 51 84 19 15 65 81 60 14 92
Card  76: 63 23 68 30 21 13 28 57 95  2 | 72 69 97 98  5 89 16 75 44 26 60 20 79 49 11 47 70 74 66 15 43 25 40 93 88
Card  77:  2 70 77 22 21 63 95 90 35 44 | 65  9 79 86 93 67 68 92 72  3 18 13 71 53 16 85 12 73 88 97 55 89 43 83 74
Card  78: 36  3 70  5 40 24 35 20 59  2 | 22  3 57 59  5 82 36  4 76 40 62 55 31 35 20 46  2 48 24 21 89 91 70 86 90
Card  79: 86  3 15 16 53  6  9 88 34 63 | 65  1 38 83 63 14 47 17 98 59 11 85 15  6 68 70 53 96 34  3 16 23  9 88 86
Card  80: 63 54  9 87 90  5 59 39 89 15 | 74 72 13 99 89 66 87 59 88 65 90 73 63 77 11 54  9 15 41 64 39  5  7 61 20
Card  81: 81 23 61 15 67 73 92 29 68 71 | 37 56 13 68 96 71 59 66 29 82  2 79 23 86 18 15 89 32 46 42 61  1 45 92 81
Card  82:  3 95 17 20 94 93 78 23 58 34 | 47 74  7  2 23  3 20 44 60 58 61 79 94 64 17 93 91 29 95 48 16 34 78 10 97
Card  83: 12 60 57 37  8 45 49 94 13 24 | 59 24 15 32  3 52 40 57 75  4 77 12 95 49  8 76 73 60 16 94 13 70 68 45 93
Card  84: 94 69 32 45  3 65 21 29 96 35 | 54 41 36 10 62 83  5 56 74 71 78 80 52 87 65 85  9 84  2 24 22 63 38 16 55
Card  85: 47 59 43 83 24  3  7 29 98 23 | 83 79 98 20 47 95 55 50 38  6 39 54 42  2 80  4 29  3 59 21 26 58 31 62 24
Card  86: 23 81 99 60 82 86 37  3  8 59 | 74 25 18  5 92 82 51 35 53  1 83  8 77 34 12 43 76 29 15 41 11 66 45 98 52
Card  87: 19 52 72 81 67 36 87 11  5  1 | 52 39 11 79 64 81 99 36  5 84  9 63  1 85 90 53 46 19 28 44 65 87 40 71 42
Card  88: 65 93 90 49 45 13 35 97 59 17 | 42 87 25 36 32 53 46 56 54 95 55 86 10  3 22 39 99  7 30  8 40 52 65 89 61
Card  89:  4 44 69 55 47 16 30 87 59 97 | 42  5 32 57 68 28 84 98 85 75 49  8 73  3 53 16 50 95 46 65 83 71 60 67 40
Card  90: 43 63 79 36 85 28 61 21  8  7 | 54 62 51 87  2  7 77 36 21 99 85 84 73  6 31 71 69 37 78 46 61 70 39 74 66
Card  91: 15 27 25  2 61 28 94  3 50 74 | 64 87 86 46 60 38 35 90 55 47 19 32 43 20 95 83 97 57 84 58 10 41 66 12 54
Card  92: 29 66 75 79 30 59 81 65  4  8 | 64 28  9 15 16 78 68 84 73 55 21 88 97 18 36 98 12 27 11 13 91 58 60 66 99
Card  93: 22 17 15 45 41 30  1  7 27 46 | 31 12 44 40 24 20 13 84 10 39 87 92 28 52 62 51 15 93 50  4 64 16  5 73 21
Card  94: 84 81 15 99 65 39 71 83 36 46 | 66 55  4  1 20 67 60 33 91 97 14 49 41  3 78  5 26 61 57 12 44 53 51 83  2
Card  95: 96 25  7 20 89 72 44 36 29 97 | 54 37 79 48 22 69 85 94 82 64 39 87  1 49  2 42 14 12 56 53 90 13 46 61 50
Card  96: 48 67 95 51 22  1  5 68 90 65 | 96 77 11 36  6 65 43 99 20 67 53 22  1 12 61 90 21 51 95 48  3 59 68 39  5
Card  97: 56 76 21 16 34  8 82 53 66 61 | 26 98 46 96 76  5 13 63 60 21 56  8 34 30 47 70 65 66 22 38 37 43 61  7 86
Card  98: 46 88 15 50 11 20 89  9 69 17 | 51  8 85 74 63 25 88 32 14 84 81  5 44 57 53 16 40 27 61 37 42 69 23 29 46
Card  99: 21 43 65 98 11 73 50 92 33 31 |  5 79 85 66 84 43  6 41 26 74 30 99 51 32 83 59 97 40 17  8 42 15 95 63 81
Card 100: 73 95  2 57 54 32 17 72 69 78 | 15 80 27 52 94 56 63 49 37 70 19 84 45 86 81 25 46  5 44 59 38 91 53 90 96
Card 101:  3 60  1 59 53 43 37 13 85 84 | 25 92 61 63 65 13 72 98 84 53 81 41 85 56 46 47  5 99  1 73 51 43 40 80  3
Card 102: 43 99 74 14 71 92 18 94  2  3 | 26 39 46 70 69 91 43 12 32 49 57 17 27 21 81 34 68 14 89 55 47 16 75  5 35
Card 103: 51 86  4 98 75 66 36 64 30 65 |  4 22 17  3 39 76 19 62 86 12 50 75 66 36 51 90 45 54 37 96 65 18 74 89 88
Card 104: 33 90 25 81  7 91 60  1 97 71 | 42 33 65 25 19  4 29  1 54 81 13 44 67 72 82 46 57 89 95  6 71 61 97 18 96
Card 105: 16 65 79 67 63 36 14 98 68 18 | 93 38 49 52 82 36 81  9 18 16 88 34 53 42 68 58 47 69 66 73 32 60 24 23 75
Card 106: 55 42 93 12 63 17 83 37 38 31 | 70 32 78  6 19 51 35 84 10 58  3  5 49 65 18 21 94 14 30 82 55 90 11 22 89
Card 107: 99 68 48 37 87 90 39 21 94 54 | 48 37 57 73 66 93 86 39 63 26 97 82 56 64 59  8 21 83 30 40 35  3 68 14 62
Card 108: 32 47 56 38 39 85 88  4 72 10 | 43 60 25 73  2 26  5 14  9 96 33 18 72 97 31 68  7 71  6 50  1 61 37 28  3
Card 109: 50 20 96 61 95 30 79 31 22 60 | 86 21 22 10  5  4 19 17 48 83 18 80 46 74 47 23 40 67 33 93 43 76 88 99 27
Card 110: 13 16 52 69 93 33 55 25 19 48 | 47 42 95 64  1 72 36 46 22 99 10 59 79 45 86 76 57 12  3 44 75 81  2  4 94
Card 111: 48 12 39 55 74 92 49 99 75 65 | 42  9 56 26 24 21 76 37 45 99 23 28 61 72 63 79 62 69 20 94  1 52 91 15 87
Card 112: 58 46  7 61 26 78 94 91 33 20 | 29 49 36 72 64 81 82 96 13 52 48 12  5 54 76 70 23 28 14 87 80 47 77 63 98
Card 113: 18 96 17 60 80 22 97 31 53 88 | 70 84  3 25 31 46 34 27 45 80 41 21 96 55 63 44 10 68 83 33 60 12 86 72  5
Card 114: 54 48 23 53 64 86 67 50 81 68 | 67 70 48 60 59 20 68 44 64 17 80 26 53 25 13 81 90 95 52 54 66 29 86  2 38
Card 115: 92 19 32 17 29 95 57 85 70 88 | 92 80 45 88 13 40 19 91 85 48 12 29 41 68 95 42 14 15 58 57 51 73 52 25 81
Card 116: 63 31 10 13 17 27 56 18 64 32 | 17 11 32 37 58 15 86 10 29 31 22 18 64 75 63 81  4 73 94 27 43 56 13 68 70
Card 117: 98 20 26 11 62  1 58 96 92 34 | 72 95 21 96 11 99 32 16 55 67 87 92 38 15 97 43 44 63 60 22 58 94 90 51  3
Card 118: 49 96 17 24 36 53 52  5 11 37 | 66 47 39 53 27  2 99 60 79 86  4 75 88  3 19 80 94 28 49 35 10 42  1 37 58
Card 119: 79 51 33  2 88 63 39 57 70 35 | 42  1 11 85 96 12 14 69 46 34 35 82 13 59 45 75 36 53 29 40 71 62 70 30 49
Card 120: 28 19 84 35 79  7 60 55 26 56 | 35 63 79 56 30 90 89 31 26 60 55  5 84 12 18 15 21 11  7 58 28 19 29 48 42
Card 121: 40 59 14 49 81 97 93 46 66 12 | 59 86 10 97 39 53 46 87 55 14 78 66 44 18 92 47 71 93 35 95 49  6 89 65 68
Card 122: 46 68 85 31 19 10 76 93 30 87 | 35 91  5 96 80 59 78 54 16 68 97 34 58 98 89 25 94 62 21 36 61 29  4 28 49
Card 123: 99 25 69 19 57  8 66 90 79 28 |  1 62 25  6 68  8 22  5 85 63 99 73 51 46 79 27 28 57 32 45 77 81 18 93 47
Card 124: 20 78 47 24 45 34 64 13 33 41 |  9 99 30 59 38 65 14 89 20 71 70 64 51 50  7 90 40 61 45 48 78 42 25 79 72
Card 125: 27 18 37 95 17 88 61 32  6 98 | 33 48 21 23  7 59  5 51 19 69 44 83 71 62 75  4 24 53 70 73 22 92 39  2 84
Card 126: 61 44 51 22 73  9 64 99 68 86 | 13 49 68 42 78 65 20 10 83 25 12 19 29 90 80 54 73  1 61 99 46 24  7 72 74
Card 127: 44 76 49 46 58 21 92  1 95 55 | 90 85 59 63 53 89 73 78 35 31 48 72 88 15 44 65 49 55 51 37 47 10 41 12 52
Card 128: 57 51 93 63  1 60 11 69 85 34 | 59  9 96 40 58 72 35 77 95 39 67 41 49 30  6 24  7 90 61 12 79 52 85 89 86
Card 129: 88 13 54 99 17  8 83 63  2 87 | 86 40 34 92 84 59 17 30 97 56 33 62 55 29 35  4 25 10 91 80 96 60 65 67 45
Card 130: 16 53 34 28 27  3 64 70  1 31 | 14 24 69 33 49 22 50 77 44 75  8 91 94 89 25 65 86 39 19 63 78 60 80 26  2
Card 131:  7 72 50 94 44 67 61 38 91 69 | 18 67 30  7 61 69 38 13 88  3 94 44 12 91 83 43  4 77 50 70 47 72 28 51 71
Card 132: 72 93 48 81 75 46  1 71 59 65 |  3 66 38 39 27 34 47 57 82 91  8 98 52 36 77 26 84 18 22 87 15 23  4 17 68
Card 133: 14 61 60  2 51 73 46 34 38 22 | 39 38 61  2 60 33 73 41 89 22 92 80 37 34 72  1 14 46 88 87 32 51 74 36 27
Card 134: 88 62 52 74 21 67 41 17 13 10 | 72 41 74 10 86  4 23 17 62 98 67 97 69 84 21 25  1 48 13 52 20 50 49 57  5
Card 135: 41 64 11 60 42 10 83 13 48 54 | 89  5 41 84 49 67 60 65 39 64 54 35 80 10 42 57  3 48 83 13  1 11 30 76 38
Card 136: 24 85 96 83 95 86  3 50  7 55 |  2 39 58 37 34 64 41 50  5 95 24 51  7 65 16 83 11 92 45 86 96  3 55 85 31
Card 137: 99 93 48  9 78 37 34 88  3 91 | 88 93 78 70 17  9  3 31 62  2 37 54 47 20 92 10 44 28 43 68 48 76 99 34 91
Card 138: 22 62 39 33 49 60 54 98 48 19 | 60 59 45 21 68 20 22 38 86  1 30 27 19  6 97 44  4 87 58 25 99 48 18 41 90
Card 139: 11 54 21  1 38 40 18 10 37  7 | 78 42 15 10 18 90 39  9 49 55 11 37  7 77 54 21 62 19 86 40 29 38 45  1 75
Card 140: 35 19 79 38 39 10 52 53 70 68 | 23 19 28 85  6 76 44 79 35 52 36 37  8 29 26 38 53 40 10 93 25 70 39 68 62
Card 141: 95 45  8 57 11 39 58 37 19 20 |  6 95 58 39 43 34 57  8 80 11 12 84 98 47 45 79 60 16 17 86 19 28 20 92 37
Card 142: 83 52 72 81 39 64 97  8 80 36 | 34 23 87 97 54 96 24 39 71 52 80 94 22 13  8 84 30 61 81 83 36 29 64 72 31
Card 143: 68 45 46 71  1 79 18 58 23 89 | 66 23 46  5 68  9 71 18 52 79 43 58 16 20 27 76 70  1 36 35 89 47 86 45 81
Card 144: 69 22 21  6 47 46 79 96 71 87 | 28 14 25 89 22 45 18 57 74 29 23 66 81 87 91 94 53 75 76  2  6 37 85 56 97
Card 145: 55 98 18 22 97 17 28 21 46  8 | 93 44 62 85 89 57 46 17  4 79 97 98 24 94 76 63 37 53 23 21 28 16  8 18 82
Card 146: 94 53 47  5 31 95 97 11 90  1 | 90 20 78  5 43 62 42 32 65 35 56 22 54  1 77  3 95  9 83 30 18 31 91 66 94
Card 147: 20  5 11 68 19 48 13 87 30 35 | 88 61 38 37 46 55 31 66 76 13 12 20 93 89 35 11  7 32 82 63 48 87 58 65 73
Card 148: 25  4 95 98 89 21 44 49 59 41 | 16  6 96  4 93 35 90 44 52 98 53 66 41  7 51 25 76 49 56 75 87 80 40 46 12
Card 149: 85 62 26 78 64  1 25 88 96 61 | 59 17 58 26 66  4 81 45 55 14  2 36 91 48 61 37 54 30 57 50 46 18 19 99 12
Card 150: 92 13 91 98  4 78 14 39 47 46 | 46  5 42 39 20 59  4 31 15 72 18 94 64  1 78 36 19 81 26 60  8 86  2 84 63
Card 151: 39 21 89  8 53 36 59 28 94 84 | 92 78 26 70 87 23 36 76 25 61 44 40 53 20 99 75 54 90 89 60 55 91 96 71  5
Card 152:  8 34 39 69 44 81 12 14 63 89 | 42 79  7 97 45 53 17 16 11 10 76 84 13 33 20 71 94 70 80 27 91 62 36 22 77
Card 153: 99 56 48 85 91 65 34  9 72 35 | 28 74 35 78 42 54 20 63  1 17 11 21 51 55 43 71 70 68 15 67 95 53 80 14 66
Card 154: 16  4 97 95 87 61 56 48 72 43 | 80 27 70 37 74 85  6 78 68  8 39 44  2 73 59 47 34 67 93 11 23 18 40 99 60
Card 155:  6 63 47 43 82 90 86 32 23 10 | 55 36 42 32 27 24 62 60  6  3 19 93 14 78 77 98 21 88 11 38 96 63 26 87 66
Card 156: 11 48 36 46 25 61 69 12 28 81 | 59 63 57 60 68 12  4 69 45 36  7 79 42 40 41 50 30 86 52 71 18 34 44 26 99
Card 157: 90 46 74 42 10 97 17 51 95 36 | 14 43 36 22 98 56 17 97 42 81 16 59 38 90 70 74 85 99 51 65 28 78 10 46 95
Card 158: 32 30 11 76 35 78 93 72 37  6 | 99 76  1 96 97 71 37 72 93  9 85 46 61 58 34 82 19 84 88 18 80 87 31 63 90
Card 159: 67 94 74 19 45 54 44 60  7 59 | 60 12 54  5 44  6 53  8 19 39 40 82 43 81 27 69 21 94 15 45 28 63 38 59 74
Card 160: 68 90 29 69 37 84 33 88 45 80 | 46 84 77 34 43 25 99  3 57 76 17 68 56 69  8 90 72 41 58 97 48 11 42 86 23
Card 161: 81 91 83 28 48 30 75 89 15 65 | 30 35  7 31 48 36  8 42 49 37  4 71 89 93 28 20 91 98 15 75 34 44 11 90 65
Card 162: 51 32  5 79 36 84 74 43 17  2 | 23 62 45 70 64  5 24 74 83 87 14 86 32 85 49 72 12 77 76 93  9 92 55 73 58
Card 163: 76 14  8 20 86 90 46 51 89 21 | 14 42 15 56 72 71 69 20 18 82 33 29 95 48 68 21 16 52 61 40 97 77 17 34 87
Card 164: 40 13 33 32 75 73 38 80 59 15 | 48 44 59 88 82 39 84 58 97 42 57 86 10  9  5 54 70  6 33 41 11 15  8 28 20
Card 165: 10 11 20 22 21  8 66 69 19 96 | 68 39 32 91 24 40 76 56 37 46 10 84 50  4 86 64 67 54 41 53 88 48 51 80 18
Card 166: 16  7 21 91 34 11 15 74 38 22 | 53 58  9 55 30 19 70 90 69 23 95 49 38 67 41 82 98 64 52 79 33 73 48 44 89
Card 167: 10 69 82  6  7 80 52 60 22 32 | 48  1 17 58 96 84 50 26 79 60 34 87 29 86  3 13 14 62 68 56 21 51 49 11 76
Card 168: 83 99 80 15 96 48 13 57 34  9 | 81 85  5 41 43  2 37 22 58 67 20 66  4 84 89  6 71 55 44 98 56 30 11 19 61
Card 169: 80  8 51 88 71 62 34 57 18 96 | 49 72 11 75  4 90 46 99 89 70 20 85 23  1 10 29  7 32 37 54 27  9 38 25 81
Card 170:  1 72 26 43 80 47 31 67 90 10 | 25 15 31 51 76 83  2  7 47 12 34 87 94 13 38 91 22 27 33 59 30  6 60 93 82
Card 171: 66 65 38 13 50 29  8 33 14 24 |  8 53 87 39 62 66 50 92 14 13 47  9 33 17 81 60 77 38 86  5 24 29 27 65 30
Card 172: 46 63 91 73 22 82 23 15 30 42 | 51 22 42 99 91 26 46 43 28 82  1 73 88 17 18 63 15 16 62 80  7 31 30 23 56
Card 173: 56  7  2 81 66 96 46 13 90 30 | 47 86 13 32  5 50 23 89 83  2 40 96 69  8 56 30 81 46 63 39 90  7 26 92 66
Card 174: 67 15 56 96 61 82 38 60 42 46 | 34 38 13 50  3 72 98 66 82 46 42 10 99 35 75 15 40 96 61 67 56 60 74 52  9
Card 175: 17 33 35  2 77 41 64 53 10 39 | 81 63 77 35 53  1 66 41  6 85 33 49 78 58 64 48 99 79 59 98 45 46 40 15 69
Card 176: 29 49 47 85 33 42 81 34 32 39 | 94  6 87 53 73 63 93 90 36 29 67 25 89  5 96 32 59 70 18 99 97  9 76 15 60
Card 177: 10  5 34 12 65 50  4 32 24 66 | 25 53 12 50 26 65 32 55 59 90 24 86 99 10  4  7 34 27 74 13 11  5 98 71 83
Card 178: 48 69 30 79  8 50 52 83 76  4 | 99 63 21 26 93 53 25 34 24 72 38 77 91  1 73 49 70 52 43 66 29 19 27 79 28
Card 179: 56 61 51 47  1 40 24 87 34 74 | 37 74 20  2 87 98 40 48 44 47 52 86 17 91 59 43 28 60 34 13 90 84 61 24 45
Card 180: 57 14 51 11 84 67 68 59 16 35 | 30 83 36 84 31 43 24 53 14 39 59 91 68  2 56 11 26 27 97 54 44 38 92 67 48
Card 181: 58 60 47 94 81 55 51  3 72 19 | 10 42 23  1 18 21 82 16 96 45 31 57 40 36 26 93 41 62 39 29 52 64 34 60 84
Card 182: 53 98 12 49  5 26 90 64 71 92 | 25 81 61 46 62  6 70 91 27 30 52 21 69 49 35 88 15 37 47 38  7 58 93 39 96
Card 183: 58 73 31 77 33 10 49 92 28 25 | 39  3 96 41 45 32 82 79  8 21 36  9  6 17 84 14 27 20 88 60 98 43 61 24  5
Card 184: 10 41 56 32 84 29 85 26 34 27 |  6  4  9 38 25 26  8 86 68 87 11 30  5 79 78 52 49 81 65 64 40  2 37 39 17
Card 185: 95 13 15 61 49 19 60 21 33 87 | 30 36 52 88 23 65 95 25 53 51 77 32 66 31 68 84 39 29  8 47 62 26 22 11 89
Card 186: 41 48 94 68 60 15 22 55 84  2 | 57 53  9 50  4 16 11 62 61  6 46 52  8 35 23 39 51 72 43 22 81 56 77 45 19
Card 187: 57 81 33  3 42 78 83 30  2  9 | 14 85 99  1 55 54 66 56 26 21 12 86 20 39 37 41 94 15 24 76 91 73 44 36  8";

    println!("4.2: {}", process(&input)?);

    Ok(())
}
