/// 素因数分解
///
/// 与えられた整数 $`n`$ を素因数分解する
///
/// $`n = p_1^{e_1} \times p_2^{e_2} \times \dots \times p_{k}^{e_{k}}`$ として、返り値は、$`[(p_1, e_1), (p_2, e_2), \dots, (p_{k}, e_{k})]`$ となる。
pub fn prime_factorize(mut n: u64) -> Vec<(u64, usize)> {
    let mut pf = vec![];

    for p in 2..=n {
        if p * p > n {
            break;
        }

        if n % p != 0 {
            continue;
        }

        let mut e = 0;
        while n % p == 0 {
            e += 1;
            n /= p;
        }

        pf.push((p, e));
    }

    if n != 1 {
        pf.push((n, 1));
    }

    pf
}
